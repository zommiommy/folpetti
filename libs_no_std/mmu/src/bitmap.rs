use alloc::vec;
use alloc::vec::Vec;

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq)]
/// A simple bitmap
/// 
pub struct Bitmap(Vec<usize>);

/// How many bits there are in a word of memory
const BITS_IN_WORD: usize = 8 * core::mem::size_of::<usize>();

impl Bitmap {
    /// Return a new bitmap, the given size **have to** be a multiple of the 
    /// number of bits in a word of memory (usize).
    /// This is needed to have tight and correct bound checking on the values
    /// without the overhead of **also** checking the len.
    pub fn new(size: usize) -> Result<Self, usize> {
        Ok(Bitmap(vec![0; (size + BITS_IN_WORD - 1) / BITS_IN_WORD]))
    }

    /// Get the bit of index `index`
    #[inline]
    pub fn get(&self, index: usize) -> bool {
        (self.0[index / BITS_IN_WORD] & (1 << index % BITS_IN_WORD)) != 0
    }

    /// set the bit of index `index` to one
    #[inline]
    pub fn set(&mut self, index: usize) {
        self.0[index / BITS_IN_WORD] |= 1 << index % BITS_IN_WORD;
    }

    /// set the bit of index `index` to zero
    #[inline]
    pub fn reset(&mut self, index: usize) {
        self.0[index / BITS_IN_WORD] &= !(1 << index % BITS_IN_WORD);
    }

    /// set the whole word of where `index` bit lives to zero.
    /// This is useful only if you know that you will clear anyway
    /// the adjacenct bits
    #[inline]
    pub fn reset_wide(&mut self, index: usize) {
        self.0[index / BITS_IN_WORD] = 0;
    }

    #[inline]
    pub fn resize(&mut self, size: usize, value: bool) {
        let fill_value = if value {
            usize::MAX
        } else {
            0
        };
        
        self.0.resize((size + BITS_IN_WORD - 1)  / BITS_IN_WORD, fill_value);
    }

    /// Clean the bitmap (as in reset everything to zero)
    #[inline]
    pub fn clear(&mut self) {
        self.0.fill(0)
    }
}