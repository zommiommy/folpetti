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
mod segment_mmu;
pub use segment_mmu::*;
mod mmu;
pub use mmu::*;


/// An error that can be raised by trying to read or write in the MMU.
/// This tries to store all the relevant informations for debugging
#[derive(Debug)]
#[repr(C)]
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

    /// The address is not in any mapped segment
    SegmentNotFound{
        virtual_address: VirtAddr,
    },
}