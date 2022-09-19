use core::ops::*;

/// A word type. THis is used when we're working with constant values or 
/// actually running an emulator producing them. This trait will be inplemented
/// for any unsigned integer
pub trait Word: Default + core::fmt::Debug + Clone + Copy + BitAnd<Output=Self> 
    + BitOr<Output=Self> + BitXor<Output=Self> + Not<Output=Self> + PartialOrd
    + Ord + core::fmt::Display + core::fmt::LowerHex
{
    type Signed: PartialOrd + Ord;

    /// Number of bits in the current type
    const BITS: u32;

    /// Minimum value encodable in this type
    const MIN: Self;

    /// Maximum value encodable in this type
    const MAX: Self;

    /// Convert `self` into the signed variant of `Self`
    fn signed(self) -> Self::Signed;

    /// Add `self` and `rhs` returing the result using wrapping arithmetic
    fn wrapping_add(self, rhs: Self) -> Self;

    /// Subtract `rhs` from `self` returing the result using wrapping arithmetic
    fn wrapping_sub(self, rhs: Self) -> Self;

    /// Logical shift left `self` by `rhis` bits returining the result.
    /// Overshifting by larger than `Self::BITS` will result in zero.
    fn overflow_shl(self, rhs: Self) -> Self;

    /// Logical shift right `self` by `rhis` bits returining the result.
    /// Overshifting by larger than `Self::BITS` will result in zero.
    fn overflow_shr(self, rhs: Self) -> Self;

    /// Arithmetic shift right `self` by `rhs`. 
    /// Overshifting by larger than `Self::BITS` will result in either `!0` or
    /// `0` dpending on the sign bit of `self`.
    fn overflow_sar(self, rhs: Self) -> Self;

    /// Interpret `self` as `rhs` bits and sign-extend it to the full `Self`
    /// width
    fn sign_extend(self, rhs: u32) -> Self;

    /// Interpret `self` as `rhs` bits and zero-extend it to the full `Self`
    /// width
    fn zero_extend(self, rhs: u32) -> Self;
}

macro_rules! impl_word {
    ($ty:ty, $sty:ty) => {
impl Word for $ty {
    type Signed = $sty;
    const BITS: u32 = <$ty>::BITS;
    const MIN: $ty = <$ty>::MIN;
    const MAX: $ty = <$ty>::MAX;

    fn signed(self) -> $sty {
        self as $sty
    }

    fn wrapping_add(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }

    fn wrapping_sub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }

    fn overflow_shl(self, rhs: Self) -> Self {
        self.checked_shl(rhs.try_into().unwrap_or(1024)).unwrap_or(0)
    }

    fn overflow_shr(self, rhs: Self) -> Self {
        self.checked_shr(rhs.try_into().unwrap_or(1024)).unwrap_or(0)
    }

    fn overflow_sar(self, rhs: Self) -> Self {
        let shamt = core::cmp::min(rhs, Self::BITS as $ty - 1);
        ((self as $sty) >> shamt) as $ty
    }

    fn sign_extend(self, rhs: u32) -> Self {
        let shift = Self::BITS - rhs;
        (((self << shift) as $ty) >> shift) as $ty
    }

    fn zero_extend(self, rhs: u32) -> Self {
        let shift = Self::BITS - rhs;
        ((self << shift) as $ty) >> shift
    }

}
    };
}

impl_word!(  u8,   i8);
impl_word!( u16,  i16);
impl_word!( u32,  i32);
impl_word!( u64,  i64);
impl_word!(u128, i128);