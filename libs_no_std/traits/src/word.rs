#![deny(unconditional_recursion)]
use core::fmt::{Debug, Display, LowerHex, Binary}; 
use core::ops::*;
use core::sync::atomic::*;
use core::num::*;
use crate::Broadcast;

/// Trait of operations possible on both Signed and Unsiged words
pub trait Number: Sized + Send + Sync +
    Display + LowerHex +
    Default + Debug + Clone + Copy +
    PartialOrd + Ord + PartialEq + Eq + Binary +
    Add<Output=Self> + AddAssign<Self> +
    BitAnd<Output=Self> + BitAndAssign<Self> +
    BitOr<Output=Self> + BitOrAssign<Self> +
    BitXor<Output=Self> + BitXorAssign<Self> +
    Div<Output=Self> + DivAssign<Self> +
    Mul<Output=Self> + MulAssign<Self> + 
    Not<Output=Self> + 
    Rem<Output=Self> + RemAssign<Self> +
    Shl<Output=Self> + ShlAssign<Self> +
    Shr<Output=Self> + ShrAssign<Self> +
    Sub<Output=Self> + SubAssign<Self> +
{
    /// Number of bits in the word
    const BITS: usize;
    /// Number of bytes in the word
    const BYTES: usize;
    /// The byte array form of the value = `[u8; Self::BYTES]`
    type BytesForm;
    /// Zero represented by `Self`
    const ZERO: Self;
    /// One represented by `Selfs`
    const ONE: Self;
    /// Minimum value represented by `Self`
    const MIN: Self;
    /// Maximum value represented by `Self`
    const MAX: Self;

    /// Get the i-th bit in the word. Valid values: [0, 63]
    fn extract_bit(&self, bit: usize) -> bool;

    /// Get the bits in range [START; END_BIT) in the word. 
    /// START valid values: [0, 63]
    /// END valid values: [1, 64]
    /// START < END!!!
    fn extract_bitfield(&self, start_bit: usize, end_bit: usize) -> Self;
    
    /// Checked integer addition. Computes self + rhs, returning None if 
    /// overflow occurred.
    fn checked_add(self, rhs: Self) -> Option<Self>;

    /// Checked integer division. Computes self / rhs, returning None 
    /// if rhs == 0.
    fn checked_div(self, rhs: Self) -> Option<Self>;

    /// Checked Euclidean division. Computes self.div_euclid(rhs), returning 
    /// None if rhs == 0.
    fn checked_div_euclid(self, rhs: Self) -> Option<Self>;

    /// Checked integer multiplication. Computes self * rhs, returning None if 
    /// overflow occurred.
    fn checked_mul(self, rhs: Self) -> Option<Self>;

    /// Checked negation. Computes -self, returning None unless self == 0.
    /// Note that negating any positive integer will overflow.
    fn checked_neg(self) -> Option<Self>;

    /// Checked exponentiation. Computes self.pow(exp), returning None if 
    /// overflow occurred.
    fn checked_pow(self, exp: u32) -> Option<Self>;

    /// Checked integer remainder. Computes self % rhs, returning None 
    /// if rhs == 0.
    fn checked_rem(self, rhs: Self) -> Option<Self>;

    /// Checked Euclidean modulo. Computes self.rem_euclid(rhs), returning None 
    /// if rhs == 0.
    fn checked_rem_euclid(self, rhs: Self) -> Option<Self>;

    /// Checked shift left. Computes self << rhs, returning None if rhs is 
    /// larger than or equal to the number of bits in self.
    fn checked_shl(self, rhs: u32) -> Option<Self>;

    /// Checked shift right. Computes self >> rhs, returning None if rhs is 
    /// larger than or equal to the number of bits in self.
    fn checked_shr(self, rhs: u32) -> Option<Self>;

    /// Checked integer subtraction. Computes self - rhs, returning None if 
    /// overflow occurred.
    fn checked_sub(self, rhs: Self) -> Option<Self>;

    /// Returns the number of ones in the binary representation of self.
    fn count_ones(self) -> u32;

    /// Returns the number of zeros in the binary representation of self.
    fn count_zeros(self) -> u32;

    /// Performs Euclidean division.
    /// Since, for the positive integers, all common definitions of division are 
    /// equal, this is exactly equal to self / rhs.
    fn div_euclid(self, rhs: Self) -> Self;

    /// Converts an integer from big endian to the target’s endianness.
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    fn from_be(rhs: Self) -> Self;

    /// Create a native endian integer value from its representation as a byte 
    /// array in big endian.
    fn from_be_bytes(bytes: Self::BytesForm) -> Self;

    /// Converts an integer from little endian to the target’s endianness.
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    fn from_le(rhs: Self) -> Self;

    /// Create a native endian integer value from its representation as a byte 
    /// array in little endian.
    fn from_le_bytes(bytes: Self::BytesForm) -> Self;

    /// Create a native endian integer value from its memory representation as 
    /// a byte array in native endianness.
    /// As the target platform’s native endianness is used, portable code likely 
    /// wants to use from_be_bytes or from_le_bytes, as appropriate instead.
    fn from_ne_bytes(bytes: Self::BytesForm) -> Self;

    /// Returns the number of leading ones in the binary representation of self.
    fn leading_ones(self) -> u32;
    /// Returns the number of trailing zeros in the binary representation of self.
    fn leading_zeros(self) -> u32;

    /// Raises self to the power of exp, using exponentiation by squaring.
    fn pow(self, exp: u32) -> Self;

    /// Calculates the least remainder of self (mod rhs).
    /// Since, for the positive integers, all common definitions of division are 
    /// equal, this is exactly equal to self % rhs.
    fn rem_euclid(self, rhs: Self) -> Self;

    /// Reverses the order of bits in the integer. The least significant bit 
    /// becomes the most significant bit, second least-significant bit becomes 
    /// second most-significant bit, etc.
    fn reverse_bits(self) -> Self;

    /// Shifts the bits to the left by a specified amount, n, wrapping the t
    /// runcated bits to the end of the resulting integer.
    /// Please note this isn’t the same operation as the << shifting operator!
    fn rotate_left(self, exp: u32) -> Self;

    /// Shifts the bits to the right by a specified amount, n, wrapping the 
    /// truncated bits to the beginning of the resulting integer.
    /// Please note this isn’t the same operation as the >> shifting operator!
    fn rotate_right(self, exp: u32) -> Self;

    /// Saturating integer addition. Computes self + rhs, saturating at the 
    /// numeric bounds instead of overflowing.
    fn saturating_add(self, rhs: Self) -> Self;

    /// Saturating integer division. Computes self / rhs, saturating at the 
    /// numeric bounds instead of overflowing.
    fn saturating_div(self, rhs: Self) -> Self;

    /// Saturating integer multiplication. Computes self * rhs, saturating at 
    /// the numeric bounds instead of overflowing.
    fn saturating_mul(self, rhs: Self) -> Self;

    /// Saturating integer exponentiation. Computes self.pow(exp), saturating 
    /// at the numeric bounds instead of overflowing.
    fn saturating_pow(self, rhs: u32) -> Self;

    /// Saturating integer subtraction. Computes self - rhs, saturating at the 
    /// numeric bounds instead of overflowing.
    fn saturating_sub(self, rhs: Self) -> Self;

    /// Reverse the byte order of the integer
    fn swap_bytes(self) -> Self;

    /// Converts self to big endian from the target’s endianness.
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    fn to_be(self) -> Self;

    /// Return the memory representation of this integer as a byte array in 
    /// big-endian (network) byte order.
    fn to_be_bytes(self) -> Self::BytesForm;

    /// Converts self to little endian from the target’s endianness.
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    fn to_le(self) -> Self;

    /// Return the memory representation of this integer as a byte array in 
    /// little-endian byte order.
    fn to_le_bytes(self) -> Self::BytesForm;

    /// Return the memory representation of this integer as a byte array in 
    /// native byte order.
    /// As the target platform’s native endianness is used, portable code should
    /// use to_be_bytes or to_le_bytes, as appropriate, instead.
    fn to_ne_bytes(self) -> Self::BytesForm;

    /// Returns the number of trailing ones in the binary representation of self.
    fn trailing_ones(self) -> u32;

    /// Returns the number of trailing zeros in the binary representation of self.
    fn trailing_zeros(self) -> u32;

    /// Add `self` and `rhs`, returning the result using wrapping arithmetic
    fn wrapping_add(self, rhs: Self) -> Self;

    /// Wrapping (modular) division. Computes self / rhs. Wrapped division on 
    /// unsigned types is just normal division. There’s no way wrapping could 
    /// ever happen. This function exists, so that all operations are accounted 
    /// for in the wrapping operations.
    fn wrapping_div(self, rhs: Self) -> Self;

    /// Wrapping Euclidean division. Computes self.div_euclid(rhs). Wrapped 
    /// division on unsigned types is just normal division. There’s no way 
    /// wrapping could ever happen. This function exists, so that all operations 
    /// are accounted for in the wrapping operations. Since, for the positive 
    /// integers, all common definitions of division are equal, this is exactly 
    /// equal to self.wrapping_div(rhs).
    fn wrapping_div_euclid(self, rhs: Self) -> Self;

    /// Wrapping (modular) multiplication. Computes self * rhs, wrapping around 
    /// at the boundary of the type.
    fn wrapping_mul(self, rhs: Self) -> Self;

    /// Wrapping (modular) negation. Computes -self, wrapping around at the 
    /// boundary of the type.
    /// Since unsigned types do not have negative equivalents all applications 
    /// of this function will wrap (except for -0). For values smaller than the 
    /// corresponding signed type’s maximum the result is the same as casting 
    /// the corresponding signed value. Any larger values are equivalent to 
    /// MAX + 1 - (val - MAX - 1) where MAX is the corresponding signed type’s 
    /// maximum.
    fn wrapping_neg(self) -> Self;

    /// Wrapping (modular) exponentiation. Computes self.pow(exp), wrapping 
    /// around at the boundary of the type.
    fn wrapping_pow(self, exp: u32) -> Self;

    /// Wrapping (modular) remainder. Computes self % rhs. Wrapped remainder 
    /// calculation on unsigned types is just the regular remainder calculation. 
    /// There’s no way wrapping could ever happen. This function exists, so 
    /// that all operations are accounted for in the wrapping operations.
    fn wrapping_rem(self, rhs: Self) -> Self;

    /// Wrapping Euclidean modulo. Computes self.rem_euclid(rhs). Wrapped modulo 
    /// calculation on unsigned types is just the regular remainder calculation. 
    /// There’s no way wrapping could ever happen. This function exists, so that 
    /// all operations are accounted for in the wrapping operations. Since, for 
    /// the positive integers, all common definitions of division are equal, 
    /// this is exactly equal to self.wrapping_rem(rhs).
    fn wrapping_rem_euclid(self, rhs: Self) -> Self;

    /// Panic-free bitwise shift-left; yields self << mask(rhs), where mask 
    /// removes any high-order bits of rhs that would cause the shift to exceed 
    /// the bitwidth of the type.
    /// Note that this is not the same as a rotate-left; the RHS of a wrapping 
    /// shift-left is restricted to the range of the type, rather than the bits 
    /// shifted out of the LHS being returned to the other end. The primitive 
    /// integer types all implement a rotate_left function, which may be what 
    /// you want instead.
    fn wrapping_shl(self, rhs: u32) -> Self;

    /// Panic-free bitwise shift-right; yields self >> mask(rhs), where mask 
    /// removes any high-order bits of rhs that would cause the shift to exceed 
    /// the bitwidth of the type.
    /// Note that this is not the same as a rotate-right; the RHS of a wrapping 
    /// shift-right is restricted to the range of the type, rather than the bits 
    /// shifted out of the LHS being returned to the other end. The primitive 
    /// integer types all implement a rotate_right function, which may be what 
    /// you want instead.
    fn wrapping_shr(self, rhs: u32) -> Self;

    /// Subtract `self` and `rhs`, returning the result using wrapping
    /// arithmetic
    fn wrapping_sub(self, rhs: Self) -> Self;
}

pub trait NonZero: Sized {
    type BaseType;

    /// Creates a non-zero without checking whether the value is non-zero. This 
    /// results in undefined behaviour if the value is zero.
    /// # Safety
    /// The value must not be zero.
    unsafe fn new_unchecked(n: Self::BaseType) -> Self;

    /// Creates a non-zero if the given value is not zero.
    fn new(n: Self::BaseType) -> Option<Self>;

    /// Returns the value as a primitive type.
    fn get(self) -> Self::BaseType;
}

/// Unsigned word
pub trait Word: Number + Broadcast<u8> {
    /// The signed variant of the word
    type SignedWord: SignedWord<UnsignedWord=Self>;
    /// The atomically modifiable variant of the word
    type AtomicWord: AtomicWord<NonAtomicWord=Self>;
    /// The non-zero variant of the word
    type NonZeroWord: NonZero<BaseType = Self>;
    
    /// Convert `self` into the signed variant of `Self`
    fn to_signed(self) -> Self::SignedWord;

    /// Convert `self` into the atomic variant of `Self`
    fn to_atomic(self) -> Self::AtomicWord;

    #[cfg(feature="atomic_from_mut")]
    fn get_mut_slice(this: &mut [Self::Atomic]) -> &mut [Self];
    
    #[cfg(feature="atomic_from_mut")]
    fn from_mut_slice(this: &mut [Self]) -> &mut [Self::Atomic];

    /// Computes the absolute difference between self and other.
    fn abs_diff(self, rhs: Self) -> Self;

    /// Logical shift left `self` by `rhs`, returing the result.
    /// Overshifting by larget rhan [`Self::BITS`] will result in zero.
    fn overflow_shl(self, rhs: Self) -> Self;

    /// Logical shift right `self` by `rhs`, returing the result.
    /// Overshifting by larget rhan [`Self::BITS`] will result in zero.
    fn overflow_shr(self, rhs: Self) -> Self;

    /// Arithmetic shift right `self` by `rhs`, returing the result.
    /// Overshifting by larger than [`Self::BITS`] will result in either
    /// `!0` or `0`, depending on the sign bit of `self`.
    fn overflow_sar(self, rhs: Self) -> Self;
    
    /// Interpret `self` as `rhs` bits and sign-extend it to [`Self::BITS`].
    fn sign_extend(self, rhs: u32) -> Self;
    
    /// Interpret `self` as `rhs` bits and zero-extend it to [`Self::BITS`].
    fn zero_extend(self, rhs: u32) -> Self;
    
    /// Checked addition with a signed integer. Computes self + rhs, returning 
    /// None if overflow occurred.
    fn checked_add_signed(self, rhs: Self::SignedWord) -> Option<Self>;
    /// Saturating integer addition. Computes self + rhs, saturating at the 
    /// numeric bounds instead of overflowing.
    fn saturating_add_signed(self, rhs: Self::SignedWord) -> Self;
    /// Wrapping (modular) addition with a signed integer. Computes self + rhs, 
    /// wrapping around at the boundary of the type.
    fn wrapping_add_signed(self, rhs: Self::SignedWord) -> Self;

    /// Returns the smallest power of two greater than or equal to n. 
    /// If the next power of two is greater than the type’s maximum value, None 
    /// is returned, otherwise the power of two is wrapped in Some.
    fn checked_next_power_of_two(self) -> Option<Self>;
    /// Returns true if and only if self == 2^k for some k.
    fn is_power_of_two(self) -> bool;
    /// Returns the smallest power of two greater than or equal to self.
    /// When return value overflows (i.e., self > (1 << (N-1)) for type uN), it 
    /// panics in debug mode and the return value is wrapped to 0 in release mode 
    /// (the only situation in which method can return 0).
    fn next_power_of_two(self) -> Self;
}

/// Signed word
pub trait SignedWord: Neg<Output=Self> + Number {
    type UnsignedWord: Word<SignedWord = Self>;
    /// The non-zero variant of the word
    type NonZeroWord: NonZero<BaseType = Self>;

    /// Convert `self` into the unsigned variant of `Self`
    fn to_unsigned(self) -> Self::UnsignedWord;

    /// Computes the absolute value of self.
    /// # Overflow behavior
    /// The absolute value of Self::MIN cannot be represented as an Self, and a
    /// ttempting to calculate it will cause an overflow. This means that code 
    /// in debug mode will trigger a panic on this case and optimized code will 
    /// return Self::MIN without a panic.
    fn abs(self) -> Self;

    /// Checked absolute value. Computes self.abs(), returning None if 
    /// self == MIN.
    fn checked_abs(self) -> Option<Self>;

    /// Checked negation. Computes -self, returning None if self == MIN.
    fn checked_neg(self) -> Option<Self>;

    /// Checked subtraction with an unsigned integer. Computes self - rhs, 
    /// returning None if overflow occurred.
    fn checked_sub_unsigned(self, rhs: Self::UnsignedWord) -> Option<Self>;

    /// Saturating addition with an unsigned integer. Computes self + rhs, 
    /// saturating at the numeric bounds instead of overflowing.
    fn saturating_add_unsigned(self, rhs: Self::UnsignedWord) -> Self;

    /// Saturating subtraction with an unsigned integer. Computes self - rhs, 
    /// saturating at the numeric bounds instead of overflowing.
    fn saturating_sub_unsigned(self, rhs: Self::UnsignedWord) -> Self;

    /// Wrapping (modular) addition with an unsigned integer. Computes 
    /// self + rhs, wrapping around at the boundary of the type.
    fn wrapping_add_unsigned(self, rhs: Self::UnsignedWord) -> Self;

    /// Wrapping (modular) subtraction with an unsigned integer. Computes 
    /// self - rhs, wrapping around at the boundary of the type.
    fn wrapping_sub_unsigned(self, rhs: Self::UnsignedWord) -> Self;

    /// Computes the absolute difference between self and other.
    fn abs_diff(self, rhs: Self) -> Self::UnsignedWord;
    
}

pub trait AtomicWord: Sized + Send + Sync {
    type NonAtomicWord: Word<AtomicWord=Self>;

    fn new(value: Self::NonAtomicWord) -> Self;
    fn load(&self, order: Ordering) -> Self::NonAtomicWord;
    fn store(&self, value: Self::NonAtomicWord, order: Ordering);
    fn get_mut(&mut self) -> &mut Self::NonAtomicWord;
    fn into_inner(self) -> Self::NonAtomicWord;

    #[cfg(feature="atomic_from_mut")]
    fn get_mut_slice(this: &mut [Self]) -> &mut [Self::NonAtomicWord];
    #[cfg(feature="atomic_from_mut")]
    fn from_mut_slice(this: &mut [Self::NonAtomicWord]) -> &mut [Self];

    fn compare_exchange(
        &self,
        current: Self::NonAtomicWord,
        new: Self::NonAtomicWord,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicWord, Self::NonAtomicWord>;

    fn compare_exchange_weak(
        &self,
        current: Self::NonAtomicWord,
        new: Self::NonAtomicWord,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicWord, Self::NonAtomicWord>;

    fn swap(
        &self,
        new: Self::NonAtomicWord,
        order: Ordering,
    ) -> Self::NonAtomicWord;

    fn fetch_add(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;
    fn fetch_saturating_add(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;
    fn fetch_and(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;
    fn fetch_max(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;
    fn fetch_min(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;
    fn fetch_nand(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;
    fn fetch_or(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;
    fn fetch_sub(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;
    fn fetch_xor(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord;

    fn fetch_update<F>(
        &self, 
        set_order: Ordering, 
        fetch_order: Ordering,
        f: F,
    ) -> Result<Self::NonAtomicWord, Self::NonAtomicWord>
    where
        F: FnMut(Self::NonAtomicWord) -> Option<Self::NonAtomicWord>;
}


macro_rules! impl_Number {
    ($ty:ty) => {
        
impl Number for $ty {
    const BITS: usize = <$ty>::BITS as _;
    const BYTES: usize = core::mem::size_of::<$ty>() as _;
    type BytesForm = [u8; core::mem::size_of::<$ty>()];
    const MIN: Self = <$ty>::MIN as _;
    const MAX: Self = <$ty>::MAX as _;
    const ZERO: Self = 0;
    const ONE: Self = 1;

    #[inline(always)]
    fn extract_bit(&self, bit: usize) -> bool {
        debug_assert!(bit < core::mem::size_of::<$ty>() * 8);
        let mask: $ty = 1 << bit;
        (self & mask) != 0
    }

    #[inline(always)]
    fn extract_bitfield(&self, start_bit: usize, end_bit: usize) -> Self {
        debug_assert!(start_bit < end_bit);
        debug_assert!(end_bit <= core::mem::size_of::<$ty>() * 8);
        let n_bits = core::mem::size_of::<$ty>() * 8;
        let mask: $ty = <$ty>::MAX >> (n_bits - (end_bit - start_bit));
        (self >> start_bit) & mask
    }

    #[inline(always)]
    fn checked_add(self, rhs: Self) -> Option<Self>{self.checked_add(rhs)}
    #[inline(always)]
    fn checked_div(self, rhs: Self) -> Option<Self>{self.checked_div(rhs)}
    #[inline(always)]
    fn checked_div_euclid(self, rhs: Self) -> Option<Self>{self.checked_div_euclid(rhs)}
    #[inline(always)]
    fn checked_mul(self, rhs: Self) -> Option<Self>{self.checked_mul(rhs)}
    #[inline(always)]
    fn checked_neg(self) -> Option<Self>{self.checked_neg()}
    #[inline(always)]
    fn checked_pow(self, exp: u32) -> Option<Self>{self.checked_pow(exp)}
    #[inline(always)]
    fn checked_rem(self, rhs: Self) -> Option<Self>{self.checked_rem(rhs)}
    #[inline(always)]
    fn checked_rem_euclid(self, rhs: Self) -> Option<Self>{self.checked_rem_euclid(rhs)}
    #[inline(always)]
    fn checked_shl(self, rhs: u32) -> Option<Self>{self.checked_shl(rhs)}
    #[inline(always)]
    fn checked_shr(self, rhs: u32) -> Option<Self>{self.checked_shr(rhs)}
    #[inline(always)]
    fn checked_sub(self, rhs: Self) -> Option<Self>{self.checked_sub(rhs)}
    #[inline(always)]
    fn count_ones(self) -> u32{self.count_ones()}
    #[inline(always)]
    fn count_zeros(self) -> u32{self.count_zeros()}
    #[inline(always)]
    fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs)}
    #[inline(always)]
    fn from_be(rhs: Self) -> Self {<$ty>::from_be(rhs)}
    #[inline(always)]
    fn from_be_bytes(bytes: Self::BytesForm) -> Self {<$ty>::from_be_bytes(bytes)}
    #[inline(always)]
    fn from_le(rhs: Self) -> Self {<$ty>::from_le(rhs)}
    #[inline(always)]
    fn from_le_bytes(bytes: Self::BytesForm) -> Self {<$ty>::from_le_bytes(bytes)}
    #[inline(always)]
    fn from_ne_bytes(bytes: Self::BytesForm) -> Self {<$ty>::from_ne_bytes(bytes)}
    #[inline(always)]
    fn leading_ones(self) -> u32{self.leading_ones()}
    #[inline(always)]
    fn leading_zeros(self) -> u32{self.leading_zeros()}
    #[inline(always)]
    fn pow(self, exp: u32) -> Self{self.pow(exp)}
    #[inline(always)]
    fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs)}
    #[inline(always)]
    fn reverse_bits(self) -> Self{self.reverse_bits()}
    #[inline(always)]
    fn rotate_left(self, rhs: u32) -> Self { self.rotate_left(rhs)}
    #[inline(always)]
    fn rotate_right(self, rhs: u32) -> Self { self.rotate_right(rhs)}
    #[inline(always)]
    fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs)}
    #[inline(always)]
    fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs)}
    #[inline(always)]
    fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs)}
    #[inline(always)]
    fn saturating_pow(self, rhs: u32) -> Self { self.saturating_pow(rhs)}
    #[inline(always)]
    fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs)}
    #[inline(always)]
    fn swap_bytes(self) -> Self{self.swap_bytes()}
    #[inline(always)]
    fn to_be(self) -> Self{self.to_be()}
    #[inline(always)]
    fn to_be_bytes(self) -> Self::BytesForm{self.to_be_bytes()}
    #[inline(always)]
    fn to_le(self) -> Self{self.to_le()}
    #[inline(always)]
    fn to_le_bytes(self) -> Self::BytesForm{self.to_le_bytes()}
    #[inline(always)]
    fn to_ne_bytes(self) -> Self::BytesForm{self.to_ne_bytes()}
    #[inline(always)]
    fn trailing_ones(self) -> u32{self.trailing_ones()}
    #[inline(always)]
    fn trailing_zeros(self) -> u32{self.trailing_zeros()}


    #[inline(always)]
    fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs)}
    #[inline(always)]
    fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs)}
    #[inline(always)]
    fn wrapping_div_euclid(self, rhs: Self) -> Self { self.wrapping_div_euclid(rhs)}
    #[inline(always)]
    fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs)}
    #[inline(always)]
    fn wrapping_neg(self) -> Self { self.wrapping_neg()}
    #[inline(always)]
    fn wrapping_pow(self, exp: u32) -> Self { self.wrapping_pow(exp)}
    #[inline(always)]
    fn wrapping_rem(self, rhs: Self) -> Self { self.wrapping_rem(rhs)}
    #[inline(always)]
    fn wrapping_rem_euclid(self, rhs: Self) -> Self { self.wrapping_rem_euclid(rhs)}
    #[inline(always)]
    fn wrapping_shl(self, exp: u32) -> Self { self.wrapping_shl(exp)}
    #[inline(always)]
    fn wrapping_shr(self, exp: u32) -> Self { self.wrapping_shr(exp)}
    #[inline(always)]
    fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs)}
}

    };
}

macro_rules! impl_word {
    ($ty:ty, $sty:ty, $aty:ty, $saty:ty, $nzty:ty, $nzsty:ty) => {

impl_Number!($ty);
impl_Number!($sty);

impl Word for $ty {
    type SignedWord = $sty;
    type AtomicWord = $aty;
    type NonZeroWord = $nzty;


    #[inline(always)]
    fn to_signed(self) -> Self::SignedWord {self as Self::SignedWord}
    #[inline(always)]
    fn to_atomic(self) -> Self::AtomicWord {Self::AtomicWord::new(self)}

    #[cfg(feature="atomic_from_mut")]
    #[inline(always)]
    fn get_mut_slice(this: &mut [Self::Atomic]) -> &mut [Self]{
        <$aty>::get_mut_slice(this)
    }

    #[cfg(feature="atomic_from_mut")]
    #[inline(always)]
    fn from_mut_slice(this: &mut [Self]) -> &mut [Self::Atomic]{
        <$aty>::from_mut_slice(this)
    }

    #[inline(always)]
    fn abs_diff(self, rhs: Self) -> Self { self.abs_diff(rhs)}

    #[inline(always)]
    fn checked_next_power_of_two(self) -> Option<Self>{self.checked_next_power_of_two()}

    #[inline(always)]
    fn overflow_shl(self, rhs: Self) -> Self { 
        self.checked_shl(rhs.try_into().unwrap_or(1024)).unwrap_or(0)
    }

    #[inline(always)]
    fn overflow_shr(self, rhs: Self) -> Self {
        self.checked_shr(rhs.try_into().unwrap_or(1024)).unwrap_or(0)
    }

    #[inline(always)]
    fn overflow_sar(self, rhs: Self) -> Self {
        let shift_amount = core::cmp::min(rhs, Self::BITS as Self - 1);
        ((self as Self::SignedWord) >> shift_amount) as Self
    }

    #[inline(always)]
    fn sign_extend(self, rhs: u32) -> Self {
        let shift_amount = Self::BITS as u32 - rhs;
        (((self << shift_amount) as Self::SignedWord) >> shift_amount) as Self
    }

    #[inline(always)]
    fn zero_extend(self, rhs: u32) -> Self {
        let shift_amount = Self::BITS as u32 - rhs;
        (self << shift_amount) >> shift_amount
    }

    #[inline(always)]
    fn checked_add_signed(self, rhs: Self::SignedWord) -> Option<Self>{self.checked_add_signed(rhs)}
    #[inline(always)]
    fn saturating_add_signed(self, rhs: Self::SignedWord) -> Self{self.saturating_add_signed(rhs)}
    #[inline(always)]
    fn wrapping_add_signed(self, rhs: Self::SignedWord) -> Self{self.wrapping_add_signed(rhs)}
    #[inline(always)]
    fn is_power_of_two(self) -> bool{self.is_power_of_two()}
    #[inline(always)]
    fn next_power_of_two(self) -> Self{self.next_power_of_two()}
}

impl SignedWord for $sty {
    type UnsignedWord = $ty;
    type NonZeroWord = $nzsty;

    #[inline(always)]
    fn to_unsigned(self) -> Self::UnsignedWord {self as Self::UnsignedWord}

    #[inline(always)]
    fn abs(self) -> Self { self.abs()}
    #[inline(always)]
    fn checked_abs(self) -> Option<Self> { self.checked_abs()}
    #[inline(always)]
    fn checked_neg(self) -> Option<Self> { self.checked_neg()}
    #[inline(always)]
    fn checked_sub_unsigned(self, rhs: Self::UnsignedWord) -> Option<Self> { self.checked_sub_unsigned(rhs)}
    #[inline(always)]
    fn saturating_add_unsigned(self, rhs: Self::UnsignedWord) -> Self {self.saturating_add_unsigned(rhs)}
    #[inline(always)]
    fn saturating_sub_unsigned(self, rhs: Self::UnsignedWord) -> Self {self.saturating_sub_unsigned(rhs)}
    #[inline(always)]
    fn wrapping_add_unsigned(self, rhs: Self::UnsignedWord) -> Self {self.wrapping_add_unsigned(rhs)}
    #[inline(always)]
    fn wrapping_sub_unsigned(self, rhs: Self::UnsignedWord) -> Self {self.wrapping_sub_unsigned(rhs)}

    #[inline(always)]
    fn abs_diff(self, rhs: Self) -> Self::UnsignedWord { self.abs_diff(rhs)}
}

impl AtomicWord for $aty {
    type NonAtomicWord = $ty;

    #[inline(always)]
    fn new(value: Self::NonAtomicWord) -> Self {
        <$aty>::new(value)
    }

    #[inline(always)]
    fn load(&self, order: Ordering) -> Self::NonAtomicWord {
        <$aty>::load(self, order)
    }

    #[inline(always)]
    fn store(&self, value: Self::NonAtomicWord, order: Ordering) {
        <$aty>::store(self, value, order)
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut Self::NonAtomicWord {
        <$aty>::get_mut(self)
    }

    #[inline(always)]

    fn into_inner(self) -> Self::NonAtomicWord {
        <$aty>::into_inner(self)
    }

    #[cfg(feature="atomic_from_mut")]
    #[inline(always)]
    fn get_mut_slice(this: &mut [Self]) -> &mut [Self::NonAtomicWord]{
        <$aty>::get_mut_slice(this)
    }

    #[cfg(feature="atomic_from_mut")]
    #[inline(always)]
    fn from_mut_slice(this: &mut [Self::NonAtomicWord]) -> &mut [Self]{
        <$aty>::from_mut_slice(this)
    }

    #[inline(always)]
    fn compare_exchange(
        &self,
        current: Self::NonAtomicWord,
        new: Self::NonAtomicWord,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicWord, Self::NonAtomicWord> {
        <$aty>::compare_exchange(
            self,
            current,
            new,
            success,
            failure,
        )
    }


    #[inline(always)]    
    fn compare_exchange_weak(
        &self,
        current: Self::NonAtomicWord,
        new: Self::NonAtomicWord,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::NonAtomicWord, Self::NonAtomicWord>{
        <$aty>::compare_exchange_weak(
            self,
            current,
            new,
            success,
            failure,
        )
    }

    #[inline(always)]
    fn swap(
        &self,
        new: Self::NonAtomicWord,
        order: Ordering,
    ) -> Self::NonAtomicWord{
        <$aty>::swap(
            self,
            new,
            order,
        )
    }

    #[inline(always)]
    fn fetch_add(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        <$aty>::fetch_add(self, value, order)
    }
    
    #[inline(always)]
    fn fetch_saturating_add(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        let mut base = <$aty>::load(self, order);
        loop {
            let new = base.saturating_add(value);
            let res = <$aty>::compare_exchange_weak(
                self,
                base,
                new,
                order,
                order,
            );
            match res {
                Ok(val) => {return val},
                Err(val) => {
                    base = val;
                }
            }
        }
    }

    #[inline(always)]
    fn fetch_and(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        <$aty>::fetch_and(self, value, order)
    }
    #[inline(always)]
    fn fetch_max(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        <$aty>::fetch_max(self, value, order)
    }
    #[inline(always)]
    fn fetch_min(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        <$aty>::fetch_min(self, value, order)
    }
    #[inline(always)]
    fn fetch_nand(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        <$aty>::fetch_nand(self, value, order)
    }
    #[inline(always)]
    fn fetch_or(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        <$aty>::fetch_or(self, value, order)
    }
    #[inline(always)]
    fn fetch_sub(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        <$aty>::fetch_sub(self, value, order)
    }
    #[inline(always)]
    fn fetch_xor(&self, value: Self::NonAtomicWord, order: Ordering) -> Self::NonAtomicWord{
        <$aty>::fetch_xor(self, value, order)
    }

    #[inline(always)]
    fn fetch_update<F>(
        &self, 
        set_order: Ordering, 
        fetch_order: Ordering,
        f: F,
    ) -> Result<Self::NonAtomicWord, Self::NonAtomicWord>
    where
        F: FnMut(Self::NonAtomicWord) -> Option<Self::NonAtomicWord> {
        <$aty>::fetch_update(self, set_order, fetch_order, f)
    }
}


impl NonZero for $nzty {
    type BaseType = $ty;

    unsafe fn new_unchecked(n: Self::BaseType) -> Self {
        <$nzty>::new_unchecked(n)
    }

    fn new(n: Self::BaseType) -> Option<Self>{
        <$nzty>::new(n)
    }

    fn get(self) -> Self::BaseType{
        <$nzty>::get(self)
    }
}


impl NonZero for $nzsty {
    type BaseType = $sty;

    unsafe fn new_unchecked(n: Self::BaseType) -> Self {
        <$nzsty>::new_unchecked(n)
    }

    fn new(n: Self::BaseType) -> Option<Self>{
        <$nzsty>::new(n)
    }

    fn get(self) -> Self::BaseType{
        <$nzsty>::get(self)
    }
}

    };
}

impl_word!(u8, i8, AtomicU8, AtomicI8, NonZeroU8, NonZeroI8);
impl_word!(u16, i16, AtomicU16, AtomicI16, NonZeroU16, NonZeroI16);
impl_word!(u32, i32, AtomicU32, AtomicI32, NonZeroU32, NonZeroI32);
impl_word!(u64, i64, AtomicU64, AtomicI64, NonZeroU64, NonZeroI64);
impl_word!(usize, isize, AtomicUsize, AtomicIsize, NonZeroUsize, NonZeroIsize);
//impl_word!(u128, i128, AtomicU128, AtomicI128);