use crate::*;
use traits::*;

#[derive(Debug)]
pub struct Mmu<
    // size of the dirty blocks
    const DIRTY_BLOCK_SIZE: usize = 256,
    // if we should check for Read After Wrtie
    const RAW: bool = true,
    // If we should track the signed bytes if they are read
    const TAINT: bool = true,    
>  {
    pub segments: alloc::vec::Vec<(VirtAddr, SegmentMmu<DIRTY_BLOCK_SIZE, RAW, TAINT>)>,
    pub data_segment_idx: usize,
    pub stack_segment_idx: usize,
    pub segments_alloc_addr: VirtAddr,
    pub segment_redzone: usize,
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
    pub fn new() -> Self {
        Self { 
            segments: alloc::vec::Vec::with_capacity(10), 
            data_segment_idx: 0, 
            stack_segment_idx: 0,
            segments_alloc_addr: VirtAddr(0x0000004000000000),
            segment_redzone: 0x1000,
        }
    }

    #[inline]
    fn resolve_segment(&mut self, addr: VirtAddr) -> Result<&mut (VirtAddr, SegmentMmu<DIRTY_BLOCK_SIZE, RAW, TAINT>), MmuError> {
        self.segments.iter_mut().find(|(start_addr, smmu)| {
            (start_addr.0..start_addr.0 + smmu.len()).contains(&addr.0)
        }).ok_or_else(|| {
            MmuError::SegmentNotFound { virtual_address: addr }
        })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.segments.iter().map(|(_addr, smmu)| smmu.len()).sum()
    }

    /// check validity on allocate overlapping
    /// this should be extremely rare as the only syscall that modify the length 
    /// are brk and remmap
    fn validate(&self) {
        // TODO!:
    }

    pub fn fork(&self) -> Self {
        let forked_segments = self.segments.iter()
            .map(|(addr, smmu)| (*addr, smmu.fork()))
            .collect::<alloc::vec::Vec<_>>();
        Self {
            segments: forked_segments,
            data_segment_idx: self.data_segment_idx,
            stack_segment_idx: self.stack_segment_idx,
            segments_alloc_addr: self.segments_alloc_addr,
            segment_redzone: self.segment_redzone,
        }
    }

    pub fn reset(&mut self, reference_memory: &Self) {
        todo!()
    }

    /// Read a value from memory at address `address` with native endianess
    /// using custom permissions (mainly used for reading the code to disassemble 
    /// with execution perms)
    pub unsafe fn read_with_perm<T>(&mut self, address: VirtAddr, perm: Perm) -> Result<T, MmuError> 
    where
        T: Copy + Word,
        SegmentMmu<DIRTY_BLOCK_SIZE, RAW, TAINT>: MmuReadWrite<T>,
    {
        let (base_addr, segment_mmu) = self.resolve_segment(address)?;
        segment_mmu.read_with_perm(VirtAddr(address.0 - base_addr.0), perm)
    }


    /// Read a value from memory at address `address` with native endianess
    pub fn read<T>(&mut self, address: VirtAddr) -> Result<T, MmuError> 
    where
        T: Copy + Word,
        SegmentMmu<DIRTY_BLOCK_SIZE, RAW, TAINT>: MmuReadWrite<T>,
    {   

        let (base_addr, segment_mmu) = self.resolve_segment(address)?;
        segment_mmu.read(VirtAddr(address.0 - base_addr.0))
    }

    /// Write a value `value` to memory at address `address` with native endianess
    pub fn write<T>(&mut self, address: VirtAddr, value: T) -> Result<(), MmuError> 
    where
        T: Copy + Word,
        SegmentMmu<DIRTY_BLOCK_SIZE, RAW, TAINT>: MmuReadWrite<T>,
    {
        let (base_addr, segment_mmu) = self.resolve_segment(address)?;
        segment_mmu.write(VirtAddr(address.0 - base_addr.0), value)
    }
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
    pub fn allocate_segment(&mut self, addr: Option<VirtAddr>, size: usize, perm: Perm) -> Result<(usize, &mut SegmentMmu<DIRTY_BLOCK_SIZE, RAW, TAINT>), MmuError> {
        let new_segment = <SegmentMmu<DIRTY_BLOCK_SIZE, RAW, TAINT>>::new(size, perm)?;
        let idx = self.segments.len();
        
        let addr = match addr {
            Some(addr) => addr,
            None => {
                todo!();
            }
        };

        self.segments.push((addr, new_segment));
        self.validate();
        Ok((idx, &mut self.segments[idx].1))
    }
}

/// Linux memory syscalls and libc functions implementations
impl<
    const DIRTY_BLOCK_SIZE: usize,
    const RAW: bool,
    const TAINT: bool,    
> Mmu<
    DIRTY_BLOCK_SIZE,
    RAW,
    TAINT,
> {
    ///  brk() sets the end of the data segment to the value specified by
    ///  addr, when that value is reasonable, the system has enough
    ///  memory, and the process does not exceed its maximum data size
    pub fn brk(&mut self, addr: VirtAddr) -> Result<(), MmuError> {
        let (data_addr, data_seg) = &mut self.segments[self.data_segment_idx];
        let segment_length = addr.0 - data_addr.0;
        data_seg.resize(segment_length, PermField::Write | PermField::ReadAfterWrite)?;
        Ok(())
    }

    /// sbrk() increments the program's data space by increment bytes.
    /// Calling sbrk() with an increment of 0 can be used to find the
    /// current location of the program break.
    pub fn sbrk(&mut self, increment: isize) -> Result<VirtAddr, MmuError> {
        let (data_addr, data_seg) = &mut self.segments[self.data_segment_idx];
        let new_length = data_seg.len().checked_add_signed(increment).unwrap();
        data_seg.resize(new_length, PermField::Write | PermField::ReadAfterWrite)?;
        Ok(VirtAddr(data_addr.0 + new_length))
    }

    pub fn mmap() {
        todo!()
    }

    pub fn memremap() {
        todo!()
    }

    pub fn munmap() {
        todo!()
    }
}
