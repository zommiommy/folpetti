//! A Memory Manager for fuzzing, heavely inspired by Gamozo's 
//! [`fuzz_with_emu`](https://github.com/gamozolabs/fuzz_with_emus/tree/master/src).
//! 
//! **Suggested approach**:
//! This crate exposes a memory manager for a contiguous chunk of memory.
//! For fuzzing this means that we can use a single Mmu for the Program 
//! segments + heap. On x86_64 we can check the first bit to know if we can use
//! this memory or it's kernel / libraries releated code (which we genrally want
//! to caputre).
//! This apporachs breaks whenthe target starts to `mmap` stuff (but we can support `brk`).
//! 
//! TODO: add a from_core_dump for memory
//! Prob we need to to set coredump_filter to 255 to dump everything
#![no_std]
#![feature(core_intrinsics)]

extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;
use alloc::collections::BTreeMap;
use core::intrinsics::unlikely;
use core::ops::Range;

mod bitmap;
pub use bitmap::*;
mod dirty;
pub use dirty::*;
mod perm;
pub use perm::*;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// A strongly typed address **relative to the start of the current segment**
pub struct VirtAddr(usize);

/// An contiguous isolated memory space
/// 
/// `DIRTY_BLOCK_SIZE` is the Block size used for resetting and tracking memory 
/// which has been modified.
/// The larger this is, the fewer but more expensive memcpys() need to occur,
/// the small, the greater but less expensive memcpys() need to occur.
/// It seems the sweet spot is often 128-4096 bytes
/// 
/// This is a generic const instead of just a const so that we can tune it for
/// different sections, depending on theirs access pattern.
#[derive(Debug, PartialEq, Eq)]
pub struct Mmu<const DIRTY_BLOCK_SIZE: usize = 256> {
    /// BLock of memory for this address space
    /// Offset 0 corresponds to address 0 in the guest address space
    memory: Vec<u8>,

    /// Holds the permission bytes for the corresponding byte in memory
    permissions: Vec<Perm>,

    /// Keep track of what was dirtied and what wasn't
    dirty: DirtyState,

    /// The base address of this Chunk of contiguos memory
    base_address: VirtAddr,

    /// The last valid virtual address in this memory, this value is increased
    /// with [`Mmu::allocate`].
    end_address: VirtAddr,

    /// List of the active allocations, this is useful because we add ASAN like
    /// dead zones around each allocation.
    /// Also this can be used to detect double frees or arbirtary frees.
    /// 
    /// TODO!: On Gamozo's impl this is an hashmap, but with no_std we don't 
    /// have them so a BTreeMap is the next best thing. In the future I can just 
    /// implement a no_std hashmap but for now it's not a priority.
    allocations: BTreeMap<VirtAddr, usize>,

    /// How much space we will leave between each allocation to amplify the 
    /// detection of out of bounds writes and reads.
    asan_dead_zones_size: usize,
}

impl<const DIRTY_BLOCK_SIZE: usize> Mmu<DIRTY_BLOCK_SIZE> {

    /// Return a new empty MMU that can contains at most `size` bytes.
    /// We have an additional requirement, `size` must be a multiple of 
    /// `DIRTY_BLOCK_SIZE` and `DIRTY_BLOCK_SIZE` must be a multiple of 64.
    pub fn new(base_address: VirtAddr, size: usize, asan_dead_zones_size: usize) -> Result<Self, MmuError> {
        
        // Check that the dirty block size is reasonable. This should be a
        // static assert but in rust these are not awesome lol
        if unlikely(DIRTY_BLOCK_SIZE % 64 != 0) {
            panic!("The given DIRTY_BLOCK_SIZE for this MMU is {} which is not a multiple of 64.", DIRTY_BLOCK_SIZE);
        }

        // Check that the size is properly aligned with the dirty block size
        if unlikely(size % DIRTY_BLOCK_SIZE != 0) {
            return Err(MmuError::SizeIsNotMultipleOfDirtyBlockSize{
                size,
                dirty_block_size: DIRTY_BLOCK_SIZE,
            });
        }

        // This should always be cached by the above if, but debug asserts don't
        // cost anythig so I can just add them and sleep at night
        debug_assert!(size % DIRTY_BLOCK_SIZE == 0);
        debug_assert_eq!(size, DIRTY_BLOCK_SIZE * (size % DIRTY_BLOCK_SIZE));

        Ok(Mmu {
            memory: vec![0; size],
            permissions: vec![Perm::default(); size],
            
            base_address,
            end_address:VirtAddr(0),

            allocations: BTreeMap::new(),
            asan_dead_zones_size,

            // Here we can unwrap because the above checks implies that 
            // `size % 64 == 0` which is a superset of the requirement.
            // Except on 128 bits archs but 1) wtf are you using 2) just change 
            // the check
            dirty: DirtyState::new(size / DIRTY_BLOCK_SIZE).unwrap(),
        })
    }

    /// Create a copy of the current memory resetting the dirty bytes infos so
    /// that when calling reset it will reset to the state of the memory at 
    /// the fork time.
    pub fn fork(&self) -> Self {
        Mmu { 
            memory: self.memory.clone(), 
            permissions: self.permissions.clone(), 

            base_address: self.base_address,
            end_address: self.end_address,
            allocations: self.allocations.clone(),
            asan_dead_zones_size: self.asan_dead_zones_size,

            // The size is already checked on creation so this cannot fail
            dirty: unsafe{DirtyState::new(self.dirty.len()).unwrap_unchecked()},
        }
    }

    /// Reset the memory to the state it was at creation. 
    pub fn reset(&mut self, reference_memory: &Mmu<DIRTY_BLOCK_SIZE>) {
        // Clean the blocks and remove the indices from the vector
        for dirty_block_index in self.dirty.drain() {
            // Compute the range of bytes we need to reset
            let start = DIRTY_BLOCK_SIZE * dirty_block_index;
            let end   = DIRTY_BLOCK_SIZE * (dirty_block_index + 1);

            // Reset the data
            self.memory[start..end].copy_from_slice(
                &reference_memory.memory[start..end]
            );
            // Reset the permissions
            self.permissions[start..end].copy_from_slice(
                &reference_memory.permissions[start..end]
            );          
        }

        // restore the allocations metadata
        self.allocations.clear();
        self.allocations.extend(reference_memory.allocations.iter());
        
        // Reset the adress informations
        self.base_address = reference_memory.base_address;
        self.end_address = reference_memory.end_address;

        // on debug check (**expensive**) that the reset is done correctly
        debug_assert_eq!(self, reference_memory);
    }

    /// Allocate a new chunk of memory
    pub fn allocate(&mut self, size: usize)  -> Result<VirtAddr, MmuError> {
        // Add padding to be aligned and add a bit of memory
        let mut align_size = (size + 0xf) & !0xf;

        // add a little padding to have an ASAN-like dead zone between
        // allocations to catch even small out of bounds
        align_size += self.asan_dead_zones_size;

        

        if end > self.memory.len() {
            return Err(CannotAllocate{
                
            });
        }

        self.set_permissions(base..(base + size), PermField::Write | PermField::ReadAfterWrite)?;


        Ok(base)
    }

    /// reset the memory of the allocation
    pub fn free(&mut self, addr: VirtAddr) -> Result<(), MmuError> {
        if let Some(size) = self.allocations.remove(&addr) {
            self.set_permissions(
                addr..VirtAddr(addr.0+size), 
                Perm::default()
            );
            Ok(())
        } else {
            Err(MmuError::InvalidFree(addr))
        }
    }

    /// Set the given permissions to a given range of virtual addresses
    pub fn set_permissions(&mut self, range: Range<VirtAddr>, permissions: Perm) 
        -> Result<(), MmuError> {
        // compute the number of bytes to set permissions for
        let size = range.end.0 - range.start.0;

        // fast path, nothing to do
        if size == 0 {
            return Ok(());
        }

        // check that we are in bound
        if range.start < self.base_address || range.end >= self.end_address {
            return Err(MmuError::SetPermissionsOutOfBound{
                base_address: self.base_address,
                end_address: self.end_address,
                range,
            });
        }

        // compute the range of bytes to update
        let start = range.start.0 - self.base_address.0;
        let end = range.end.0 - self.base_address.0;
        let range_to_modify = start..end; 

        // apply the permissions
        self.permissions[range_to_modify].fill(permissions);

        // compute the dirty range
        let dirty_start = start / DIRTY_BLOCK_SIZE;
        let dirty_end = end / DIRTY_BLOCK_SIZE;

        // dirty the blocks
        for idx in dirty_start..dirty_end {
            self.dirty.dirty(idx);
        }

        Ok(())
    }

    /// MAYBE NON SERVER
    /// 
    /// Write the slice to memory **ignoring the permissions**. This is mainly 
    /// meant to be used when setupping the memory for the process and should
    /// not be used when emulating. For this reason the function is unsafe.
    /// 
    /// If permisison is None the permissions will not be touched. If it's Some
    /// the permission of the slice wull be setted to it.
    /// 
    /// The MMU **have to** enough memory for the slice. This will not grow the
    /// size of the memory nor will allocate new memory.
    /// 
    /// If permission has [`PermField::ReadAfterWrite`] setted, this function 
    /// will return an error with [`MmuError::UselessReadAfterWrite`]. 
    /// 
    /// # Safety
    /// This function is not actually unsafe but it's a reminder that this should
    /// only be used in initializzation, and potential crashes in the emulator
    /// could be caused by the missuse of this function. So it's "indirectly"
    /// unsafe. 
    pub unsafe fn write_from_slice(&mut self, address: VirtAddr, slice: &[u8], permission: Option<Perm>) -> Result<(), MmuError> {
        if let Some(perm) = permission {

        }
    }

    /// Return a reference to a chunk of virtual memory, after ensuring the 
    /// bounds and that all the bytes have **at least** `permissions`.
    /// This is mainly used when decoding the code in an emulator where we can 
    /// execute `decode(mmu.get_with_perms(rip, 128, PermField::Execute))`
    pub fn get_with_perms(&self, address: VirtAddr, size: usize, permissions: Perm) -> Result<&[u8], MmuError> {
        unimplemented!()
    }

    // Read a value from memory at address `address` with little endianess
    pub fn read_little<T>(&mut self, address: VirtAddr) -> Result<T, MmuError> 
    where
        T: Copy,
    {
        <Self as MmuReadWrite<T>>::read_inner_little(self, address)
    }

    // Read a value from memory at address `address` with big endianess
    pub fn read_big<T>(&mut self, address: VirtAddr) -> Result<T, MmuError> 
    where
        T: Copy,
    {
        <Self as MmuReadWrite<T>>::read_inner_big(self, address)
    }

    // Write a value `value` to memory at address `address` with little endianess
    pub fn write_little<T>(&mut self, address: VirtAddr, value: T) -> Result<(), MmuError> 
    where
        T: Copy,
        Self: MmuReadWrite<T>,
    {
        <Self as MmuReadWrite<T>>::write_inner_little(self, address, value)
    }

    // Write a value `value` to memory at address `address` with big endianess
    pub fn write_big<T>(&mut self, address: VirtAddr, value: T) -> Result<(), MmuError> 
    where
        T: Copy,
        Self: MmuReadWrite<T>,
    {
        <Self as MmuReadWrite<T>>::write_inner_big(self, address, value)
    }
}

/// An error that can be raised by trying to read or write in the MMU.
/// This tries to store all the relevant informations for debugging
pub enum MmuError {
    /// This error is raised if an initializzation write wants to write data
    /// with [`PermField::ReadAfterWrite`]. While this is not an error, it's 
    /// useless and means that we probably made an error some where, so I prefer
    /// to have an early stop and figure it out rather than continue silently.
    UselessReadAfterWrite,

    /// This error is raised when the requested allocation memory is bigger than
    /// the **initial** size of memory handable by this MMU
    CannotAllocate {
        virtual_address: VirtAddr,
        mmu_length: usize,
    },

    /// To avoid possible bugs / out of bounds I choose to force the size of the
    /// MMU to be a multiple of the dirty block size.
    SizeIsNotMultipleOfDirtyBlockSize{
        size: usize,
        dirty_block_size: usize,
    },

    /// This error is raised when a free is called on something that wasn't an
    /// allocation done by this MMU.
    InvalidFree(VirtAddr),

    /// This error is raised when trying to set permissions, the virtual address
    /// range to set is out of bound
    SetPermissionsOutOfBound{
        base_address: VirtAddr,
        end_address: VirtAddr,
        /// Range to set
        range: Range<VirtAddr>,
    },

    /// This error is raised when a Read or a Write access memory out of bound.
    /// This could tecnically be a [`MmuError::PermissionFault`] with permissions
    /// 0, but this is more specific and let us better understand where this
    /// error araises.
    OutOfBound {
        /// If the operation that generated the error was a Read or a Write
        is_read: bool,
        virtual_address: VirtAddr,
        mmu_length: usize,
    },

    /// permissions and memory are valid only for `size` bytes, the rest will be
    /// probably zero padded but for now it's not something relayable
    PermissionsFault{
        /// If the operation that generated the error was a Read or a Write
        is_read: bool,
        virtual_address: VirtAddr,
        permissions: [Perm; 8],
        memory: [u8; 8],
        size: usize,
    },
}

/// This traits allows us to separate the implementations of read and write
/// for current and future types. This might be overwkill but allows for a 
/// really easy time form an user prospective that can just to 
/// `mmu.read::<u32>(0)` and everything figured out optimally at compile time
trait MmuReadWrite<T>
where
    T: Copy,
{
    fn read_inner_little(&mut self, address: VirtAddr) -> Result<T, MmuError>;
    fn write_inner_little(&mut self, address: VirtAddr, value: T) 
        -> Result<(), MmuError>;
    fn read_inner_big(&mut self, address: VirtAddr) -> Result<T, MmuError>;
    fn write_inner_big(&mut self, address: VirtAddr, value: T) 
        -> Result<(), MmuError>;
}

impl<const DIRTY_BLOCK_SIZE: usize> MmuReadWrite<u8> for Mmu<DIRTY_BLOCK_SIZE> {
    fn read_inner_little(&mut self, address: VirtAddr) -> Result<u8, MmuError> {
        unimplemented!()
    }
    fn read_inner_big(&mut self, address: VirtAddr) -> Result<u8, MmuError> {
        unimplemented!()
    }

    fn write_inner_little(&mut self, address: VirtAddr, value: u8) -> Result<(), MmuError> {
        // check if we can write on all the bytes needed
        if unlikely(!self.permissions[address.0].is_superset_of(
            PermField::Write | PermField::ReadAfterWrite
        )) {
            let size = core::mem::size_of::<u8>();

            // Get the bytes from 
            let mut permissions: [Perm; 8] = Default::default();
            permissions.copy_from_slice(&self.permissions[address.0..self.permissions.len().min(address.0 + size)]);

            let mut memory: [u8; 8] = Default::default();
            memory.copy_from_slice(&self.memory[address.0..address.0 + size]);

            return Err(MmuError::PermissionsFault{
                is_read: false, 
                virtual_address: address,
                permissions,
                memory,
                size,
            });
        }
 
        // write the value
        self.memory[address.0] = value;
        
        // update the access
        self.permissions[address.0] |= PermField::Accessed;

        // update the dirty bitmap and push memory
        if self.permissions[address.0].is_superset_of(PermField::ReadAfterWrite) {
            self.permissions[address.0] |= PermField::Read;
        }

        Ok(())
    }

    fn write_inner_big(&mut self, address: VirtAddr, value: u8) -> Result<(), MmuError> {
        unimplemented!()
    }
}