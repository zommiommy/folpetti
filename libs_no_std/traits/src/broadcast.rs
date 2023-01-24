
/// Take a smaller value and broadcast to all the values
/// Create a word of memory of type $ty with Perm in all the bytes,
/// this allows us to check the permissions all at once in small reads and
/// writes. (Thanks to B3NNY for the more readable code, this should compile to
/// the original multiplication by 0x0101010101010101).
pub trait Broadcast<T> {
    fn broadcast(value: T) -> Self;
}

macro_rules! impl_broadcast {
    ($($ty1:ty => $ty2:ty,)*) => {      
$(
impl Broadcast<$ty1> for $ty2 {
    #[inline(always)]
    fn broadcast(value: $ty1) -> Self {
        const SIZE: usize = core::mem::size_of::<$ty2>() / core::mem::size_of::<$ty1>();
        <$ty2>::from_ne_bytes([value; SIZE])
    }
}
)*
    };
}

impl_broadcast!(
    u8 => u8,
    u8 => u16,
    u8 => u32,
    u8 => u64,
    u8 => usize,

//    u16 => u16,
//    u16 => u32,
//    u16 => u64,
//
//    u32 => u32,
//    u32 => u64,
//
//    u64 => u64,
);