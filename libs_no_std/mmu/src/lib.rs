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
#![feature(atomic_from_mut)]
#![feature(core_intrinsics)]
#![feature(allocator_api)]

extern crate alloc;

use alloc::vec::Vec;
use alloc::vec;
use alloc::collections::BTreeMap;

use core::intrinsics::unlikely;
use core::ops::Range;

//mod pagetable;

mod virtaddr;
pub use virtaddr::*;
mod bitmap;
pub use bitmap::*;
mod dirty;
pub use dirty::*;
mod perm;
pub use perm::*;
mod mmu_read_write_impls;
pub use mmu_read_write_impls::*;


/// An error that can be raised by trying to read or write in the MMU.
/// This tries to store all the relevant informations for debugging
#[derive(Debug)]
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
    },

    PermissionsFault{
        /// If the operation that generated the error was a Read or a Write
        is_read: bool,
        virtual_address: VirtAddr,
        /// these are initialized only for the len of the type read
        permissions: [Perm; 8],
        size: usize,
    },
}

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
pub struct Mmu<
    // size of the dirty blocks
    const DIRTY_BLOCK_SIZE: usize = 256,
    // if we should check for Read After Wrtie
    const RAW: bool = true,
    // If we should track the signed bytes if they are read
    const TAINT: bool = true,    
> {
    /// BLock of memory for this address space
    /// Offset 0 corresponds to address 0 in the guest address space
    memory: Vec<u8>,

    /// Holds the permission bytes for the corresponding byte in memory
    permissions: Vec<Perm>,

    /// Keep track of what was dirtied and what wasn't
    pub dirty: DirtyState,

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
}

impl<
    const DIRTY_BLOCK_SIZE: usize,
    const RAW: bool,
    const TAINT: bool,    
> Mmu<
    DIRTY_BLOCK_SIZE,
    RAW,
    TAINT,
> {

    /// Return a new empty MMU that can contains at most `size` bytes.
    /// We have an additional requirement, `size` must be a multiple of 
    /// `DIRTY_BLOCK_SIZE` and `DIRTY_BLOCK_SIZE` must be a multiple of 64.
    pub fn new() -> Result<Self, MmuError> {
        todo!();
        /*
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
        debug_assert_eq!(size, DIRTY_BLOCK_SIZE * (size / DIRTY_BLOCK_SIZE));

        Ok(Mmu {
            memory: vec![0; size],
            permissions: vec![Perm::default(); size],
            
            // init to a bit bigger size so that we catch null derefs
            end_address:VirtAddr(0x1000),

            allocations: BTreeMap::new(),

            // Here we can unwrap because the above checks implies that 
            // `size % 64 == 0` which is a superset of the requirement.
            // Except on 128 bits archs but 1) wtf are you using 2) just change 
            // the check
            dirty: DirtyState::new(size / DIRTY_BLOCK_SIZE).unwrap(),
        })
        */
    }

    /// Create a copy of the current memory resetting the dirty bytes infos so
    /// that when calling reset it will reset to the state of the memory at 
    /// the fork time.
    pub fn fork(&self) -> Self {
        Mmu { 
            memory: self.memory.clone(), 
            permissions: self.permissions.clone(), 

            end_address: self.end_address,
            allocations: self.allocations.clone(),

            // The size is already checked on creation so this cannot fail
            dirty: unsafe{DirtyState::new(self.dirty.len()).unwrap_unchecked()},
        }
    }

    /// Reset the memory to the state it was at creation. 
    pub fn reset(&mut self, reference_memory: &Self) {
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
        self.end_address = reference_memory.end_address;

        // on debug check (**expensive**) that the reset is done correctly
        debug_assert_eq!(self.end_address, reference_memory.end_address);
        debug_assert_eq!(self.permissions, reference_memory.permissions);
        debug_assert_eq!(self.allocations, reference_memory.allocations);
        debug_assert_eq!(self.memory, reference_memory.memory);
    }

    /// Allocate a new chunk of memory
    pub fn allocate(&mut self, size: usize)  -> Result<VirtAddr, MmuError> {
        // Add padding to be aligned and add a bit of memory
        let aligned_size = (size + 0xf) & !0xf;
        // compute the end address **after** the allocation
        let base = self.end_address;
        let new_end_addr = VirtAddr(self.end_address.0 + aligned_size);
        
        // check that we are in bound
        if new_end_addr > VirtAddr(self.memory.len()) {
            return Err(MmuError::CannotAllocate{
                virtual_address: self.end_address,
                mmu_length: self.memory.len(),
            });
        }

        // update the value, this is used to bound check `set_permissions`
        self.end_address = new_end_addr;

        // Set the permissions soso that we can write into it
        self.set_permissions(base..new_end_addr, 
        if RAW {
                PermField::Write | PermField::ReadAfterWrite
            } else {
                PermField::Write | PermField::Read
            }
        )?;

        Ok(base)
    }

    /// reset the memory of the allocation
    pub fn free(&mut self, addr: VirtAddr) -> Result<(), MmuError> {
        if let Some(size) = self.allocations.remove(&addr) {
            self.set_permissions(
                addr..VirtAddr(addr.0+size), 
                Perm::default()
            ).unwrap(); // TODO!: check unwrap
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
        if range.end > self.end_address {
            return Err(MmuError::SetPermissionsOutOfBound{
                end_address: self.end_address,
                range,
            });
        }

        // compute the range of bytes to update
        let range_to_modify = range.start.0..range.end.0; 

        // apply the permissions
        self.permissions[range_to_modify].fill(permissions);

        // compute the dirty range
        let dirty_start = (range.start.0 + DIRTY_BLOCK_SIZE - 1) / DIRTY_BLOCK_SIZE;
        let dirty_end   = (range.end.0   + DIRTY_BLOCK_SIZE - 1) / DIRTY_BLOCK_SIZE;

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
        unimplemented!();
    }

    /// Return a reference to a chunk of virtual memory, after ensuring the 
    /// bounds and that all the bytes have **at least** `permissions`.
    /// This is mainly used when decoding the code in an emulator where we can 
    /// execute `decode(mmu.get_with_perms(rip, 128, PermField::Execute))`
    pub fn get_with_perms(&self, address: VirtAddr, size: usize, permissions: Perm) -> Result<&[u8], MmuError> {
        unimplemented!()
    }

    // Read a value from memory at address `address` with native endianess
    pub fn read<T>(&mut self, address: VirtAddr) -> Result<T, MmuError> 
    where
        T: Copy,
        Self: MmuReadWrite<T>,
    {
        <Self as MmuReadWrite<T>>::read(self, address)
    }

    // Write a value `value` to memory at address `address` with native endianess
    pub fn write<T>(&mut self, address: VirtAddr, value: T) -> Result<(), MmuError> 
    where
        T: Copy,
        Self: MmuReadWrite<T>,
    {
        <Self as MmuReadWrite<T>>::write(self, address, value)
    }
}
