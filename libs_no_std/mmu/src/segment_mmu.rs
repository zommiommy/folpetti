use super::*;
use traits::Word;
use alloc::vec::Vec;
use alloc::vec;

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
pub struct SegmentMmu<
    // size of the dirty blocks
    const DIRTY_BLOCK_SIZE: usize = 256,
    // if we should check for Read After Wrtie
    const RAW: bool = true,
    // If we should track the signed bytes if they are read
    const TAINT: bool = true,    
> {
    /// BLock of memory for this address space
    /// Offset 0 corresponds to address 0 in the guest address space
    pub memory: Vec<u8>,

    /// Holds the permission bytes for the corresponding byte in memory
    pub permissions: Vec<Perm>,

    /// Keep track of what was dirtied and what wasn't
    pub dirty: DirtyState,
}

impl<
    const DIRTY_BLOCK_SIZE: usize,
    const RAW: bool,
    const TAINT: bool,    
> SegmentMmu<
    DIRTY_BLOCK_SIZE,
    RAW,
    TAINT,
> {

    #[inline(always)]
    pub fn len(&self) -> usize {
        debug_assert_eq!(self.memory.len(), self.permissions.len());
        self.memory.len()
    }

    /// Return a new empty MMU that can contains at most `size` bytes.
    /// We have an additional requirement, `size` must be a multiple of 
    /// `DIRTY_BLOCK_SIZE` and `DIRTY_BLOCK_SIZE` must be a multiple of 64.
    pub fn new(size: usize, perm: Perm) -> Result<Self, MmuError> {
        // Check that the dirty block size is reasonable. This should be a
        // static assert but in rust these are not awesome lol
        if unlikely(DIRTY_BLOCK_SIZE % 64 != 0) {
            panic!("The given DIRTY_BLOCK_SIZE for this MMU is {} which is not a multiple of 64.", DIRTY_BLOCK_SIZE);
        }

        Ok(SegmentMmu {
            memory: vec![0; size],
            permissions: vec![perm; size],
            dirty: DirtyState::new(
                (size + DIRTY_BLOCK_SIZE - 1) / DIRTY_BLOCK_SIZE // ceil
            ).unwrap(),
        })
    }   

    /// Create a copy of the current memory resetting the dirty bytes infos so
    /// that when calling reset it will reset to the state of the memory at 
    /// the fork time.
    pub fn fork(&self) -> Self {
        SegmentMmu { 
            memory: self.memory.clone(), 
            permissions: self.permissions.clone(), 

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
            let end   = DIRTY_BLOCK_SIZE + start;

            // Reset the data
            self.memory[start..end].copy_from_slice(
                &reference_memory.memory[start..end]
            );
            // Reset the permissions
            self.permissions[start..end].copy_from_slice(
                &reference_memory.permissions[start..end]
            );          
        }

        // Reset the adress informations
        // on debug check (**expensive**) that the reset is done correctly
        debug_assert_eq!(self.permissions, reference_memory.permissions);
        debug_assert_eq!(self.memory, reference_memory.memory);
    }

    pub fn resize(&mut self, size: usize, perm: Perm) -> Result<(), MmuError> {
        // TODO! should we leave the allocation? is better an out of bound or
        // a permission denied?
        self.memory.resize(size, 0);
        self.permissions.resize(size, perm);
        self.dirty.resize(size);
        Ok(())
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
        if range.end.0 > self.len() {
            return Err(MmuError::SetPermissionsOutOfBound{
                end_address:VirtAddr(self.len()),
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
    pub unsafe fn write_from_slice(&mut self, address: VirtAddr, slice: &[u8]) -> Result<(), MmuError> {
        self.memory[address.0..address.0 + slice.len()].copy_from_slice(slice);
        Ok(())
    }

    pub unsafe fn write_from_slice_with_perm(&mut self, address: VirtAddr, slice: &[u8], perm: Perm) -> Result<(), MmuError> {
        self.memory[address.0..address.0 + slice.len()].copy_from_slice(slice);
        self.permissions[address.0..address.0 + slice.len()].fill(perm);
        Ok(())
    }

    /// Read a value from memory at address `address` with native endianess
    /// using custom permissions (mainly used for reading the code to disassemble 
    /// with execution perms)
    pub unsafe fn read_with_perm<T>(&mut self, address: VirtAddr, perm: Perm) -> Result<T, MmuError> 
    where
        T: Copy + Word,
        Self: MmuReadWrite<T>,
    {
        <Self as MmuReadWrite<T>>::read_with_perm(self, address, perm)
    }


    /// Read a value from memory at address `address` with native endianess
    pub fn read<T>(&mut self, address: VirtAddr) -> Result<T, MmuError> 
    where
        T: Copy + Word,
        Self: MmuReadWrite<T>,
    {
        <Self as MmuReadWrite<T>>::read(self, address)
    }

    /// Write a value `value` to memory at address `address` with native endianess
    pub fn write<T>(&mut self, address: VirtAddr, value: T) -> Result<(), MmuError> 
    where
        T: Copy + Word,
        Self: MmuReadWrite<T>,
    {
        <Self as MmuReadWrite<T>>::write(self, address, value)
    }
}
