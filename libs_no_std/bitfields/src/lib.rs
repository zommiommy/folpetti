//! Simple traits to extract bitfields from a word.
//! This is simple but it's realayable as every case is tested and it pass
//! MIRI so no UB yay
#![no_std]
#[allow(non_snake_case)]

pub trait BitFields {
    /// Get the i-th bit in the word. Valid values: [0, 63]
    fn extract_bit<const BIT: usize>(&self) -> bool;

    /// Get the bits in range [START; END_BIT) in the word. 
    /// START valid values: [0, 63]
    /// END valid values: [1, 64]
    /// START < END!!!
    fn extract_bitfield<const START_BIT: usize, const END_BIT: usize>(&self) -> Self;
}

macro_rules! impl_bitfields {
    ($($t:ty),*) => {
$(
impl BitFields for $t {
    #[inline(always)]
    fn extract_bit<const BIT: usize>(&self) -> bool {
        debug_assert!(BIT < core::mem::size_of::<$t>() * 8);
        let mask: $t = 1 << BIT;
        (self & mask) != 0
    }

    #[inline(always)]
    fn extract_bitfield<const START_BIT: usize, const END_BIT: usize>(&self) -> Self {
        debug_assert!(START_BIT < END_BIT);
        let n_bits = core::mem::size_of::<$t>() * 8;
        let mask: $t = <$t>::MAX >> (n_bits - (END_BIT - START_BIT));
        (self >> START_BIT) & mask
    }
}
)*
    };
}

impl_bitfields!{
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize
}