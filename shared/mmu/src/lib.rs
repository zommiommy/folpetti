mod perm;
pub use perm::*;

pub struct VirtAddr(usize);

/// An isolated memory space
/// 
/// `DIRTY_BLOCK_SIZE` is the Block size used for resetting and tracking memory 
/// which has been modified.
/// The larger this is, the fewer but more expensive memcpys() need to occur,
/// the small, the greater but less expensive memcpys() need to occur.
/// It seems the sweet spot is often 128-4096 bytes
/// 
/// This is a generic const instead of just a const so that we can tune it for
/// different sections, depending on theirs access pattern.
pub struct Mmu<const DIRTY_BLOCK_SIZE: usize> {
    /// BLock of memory for this address space
    /// Offset 0 corresponds to address 0 in the guest address space
    memory: Vec<u8>,

    /// Holds the permission bytes for the corresponding byte in memory
    permissions: Vec<Perm>,

    /// Track the addresses of the block in guest memory which are dirty
    dirty: Vec<VirtAddr>,

    /// Track which partes of memory have been dirtied, it's used as a filter
    /// to avoid duplicated entries inside `dirty`.
    dirty_bitmap: Vec<u64>,

    /// Current base address of the next allocation
    cur_alc: VirtAddr,
}

impl<const DIRTY_BLOCK_SIZE: usize> Mmu<DIRTY_BLOCK_SIZE> {
    pub fn new(size: usize) -> Self {
        Mmu {
            memory: vec![0; size],
            permissions: vec![Perm::default(); size],
            dirty:         Vec::with_capacity[]
        }
    }
}