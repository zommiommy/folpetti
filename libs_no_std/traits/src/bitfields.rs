//! Simple traits to extract bitfields from a word.
//! This is simple but it's realayable as every case is tested and it pass
//! MIRI so no UB yay

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
        debug_assert!(END_BIT < core::mem::size_of::<$t>() * 8);
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

/// Sign extend a value from an unsigned value to a signed one.
pub trait SignExtend: UnsignedInteger {
    /// Signe-extend the value assiming BIT-width
    fn sign_extend<const BIT: usize>(&self) -> <Self as UnsignedInteger>::Signed;
}

macro_rules! impl_sign_extend {
    ($($t:ty),*) => {
$(
impl SignExtend for $t {
    #[inline(always)]
    fn sign_extend<const BIT: usize>(&self) -> <Self as UnsignedInteger>::Signed {
        debug_assert!(BIT < core::mem::size_of::<$t>() * 8);
        let shamt = <Self as Word>::BITS - BIT;
        ((self.as_signed() << shamt) >> shamt)
    }
}
)*
    };
}

impl_sign_extend!{
    u8, u16, u32, u64, usize
}

pub trait Word {
    const BITS: usize;
    const BYTES: usize;
}

pub trait SignedInteger: Word {
    type Unsigned: UnsignedInteger;

    fn as_unsigned(self) -> Self::Unsigned;
}

pub trait UnsignedInteger: Word {
    type Signed: SignedInteger;

    fn as_signed(self) -> Self::Signed;
}

macro_rules! impl_stuff{
    ($(($ut:ty, $st:ty)),*) => {
$(
impl Word for $ut {
    const BYTES: usize = core::mem::size_of::<$ut>();
    const BITS:  usize = 8 * Self::BYTES;
}
impl Word for $st {
    const BYTES: usize = core::mem::size_of::<$st>();
    const BITS:  usize = 8 * Self::BYTES;
}

impl SignedInteger for $st {
    type Unsigned = $ut;

    #[inline(always)]
    fn as_unsigned(self) -> Self::Unsigned {
        self as $ut
    }
}    

impl UnsignedInteger for $ut {
    type Signed = $st;

    #[inline(always)]
    fn as_signed(self) -> Self::Signed {
        self as $st
    }
}    
)*
    };
}

impl_stuff!{
    (u8, i8), (u16, i16), (u32, i32), (u64, i64), (usize, isize)
}