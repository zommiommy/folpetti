use super::Bitmap;
use alloc::vec::Vec;


#[derive(Debug, PartialEq, Eq)]
pub struct DirtyState {
    /// Track the addresses of the block in guest memory which are dirty
    dirty_indices: Vec<usize>,

    /// Track which partes of memory have been dirtied, it's used as a filter
    /// to avoid duplicated entries inside `dirty`.
    dirty_bitmap: Bitmap,

    /// How many indices we can store, this is a parameter passed by 
    /// [`DirtyState::new`]
    len: usize,
}

impl DirtyState {
    /// Create a new Dirty State. To be compatible with the underlaying Bitmap
    /// `len` must be a multiple of the number of bits in a word of memory 
    /// (usize)
    #[inline]
    pub fn new(len: usize) -> Result<Self, usize> {
        Ok(DirtyState {
            dirty_indices: Vec::with_capacity(len),
            dirty_bitmap: Bitmap::new(len)?,
            len,
        })
    }

    /// Sign a certain block as dirty
    #[inline]
    pub fn dirty(&mut self, block_idx: usize) {
        // if it wasn't dirty
        if !self.dirty_bitmap.get(block_idx) {
            // add the block idx to the indices
            self.dirty_indices.push(block_idx);
            // and set it's bit in the bitmap for deduplication
            self.dirty_bitmap.set(block_idx);
        }
    }

    /// Returns an iterator over the dirtied indices while resetting itself
    /// so that the allocations can be re-used.
    #[inline]
    pub fn drain(&mut self) -> impl Iterator<Item=usize> + '_ {
        self.dirty_indices.drain(..).map(|idx| {
            self.dirty_bitmap.reset_wide(idx);
            idx
        })
    }

    /// Return the size with which the dirty state was initialized
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
}