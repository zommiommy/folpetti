use super::*;

/// This traits allows us to separate the implementations of read and write
/// for current and future types. This might be overwkill but allows for a 
/// really easy time form an user prospective that can just to 
/// `mmu.read::<u32>(0)` and everything figured out optimally at compile time.
pub trait MmuReadWrite<T>
where
    T: Copy,
{
    /// Read a value T from the memory at address `address`,
    fn read(&mut self, address: VirtAddr) -> Result<T, MmuError>;
    /// Write a `value` T to the memory at address `address`,
    fn write(&mut self, address: VirtAddr, value: T) 
        -> Result<(), MmuError>;
}

/// Create a word of memory of type $ty with Perm in all the bytes,
/// this allows us to check the permissions all at once in small reads and
/// writes. (Thanks to B3NNY for the more readable code, this should compile to
/// the original multiplication by 0x0101010101010101).
macro_rules! to_wide_perm {
    ($perm_byte:expr, $ty:ty) => {
        <$ty>::from_ne_bytes([$perm_byte; core::mem::size_of::<$ty>()])
    };
}

/// Saddly rust misses this blanket impl, and thanks to the orphan rules we can't
/// add it. For this reason we will just use this dump function.
/// ```ignore
/// impl<const N: usize, FromType, IntoType> From<[FromType; N]> for [IntoType; N]
/// where
///    FromType: Into<IntoType> + Copy,
///    IntoType: From<FromType> + Default + Copy,
/// {
///     fn from(x: [FromType; N]) -> [IntoType; N] {
///         let mut y: [IntoType; N] = [Default::default(); N];
///         for i in 0..N {
///             y[i] = x[i].into();
///         }
///         y
///     }
/// }
/// ```
#[inline(always)]
fn convert_arrays<
    const N: usize,
    FromType: Into<IntoType> + Copy,
    IntoType: From<FromType> + Default + Copy,
>(x: [FromType; N]) -> [IntoType; N] {
    let mut y: [IntoType; N] = [Default::default(); N];
    for i in 0..N {
        y[i] = x[i].into();
    }
    y
} 

/// Implement reads and writes for primitive unsigned integers
macro_rules! impl_read_write {
    ($($ty:ty),*) => {
$(
impl<
    const DIRTY_BLOCK_SIZE: usize,
    const RAW: bool,
    const TAINT: bool,    
> MmuReadWrite<$ty> for Mmu<
    DIRTY_BLOCK_SIZE,
    RAW,
    TAINT,
> {

    #[inline]
    fn read(&mut self, address: VirtAddr) -> Result<$ty, MmuError> {
        const SIZE: usize = core::mem::size_of::<$ty>();
        const READ_WIDE: $ty = to_wide_perm!(PermField::Read as u8, $ty);

        // Get the permissions while checking for out of bounds
        let perms = self.permissions.get(address.0..address.0 + SIZE)
            .ok_or_else(|| MmuError::OutOfBound{
                is_read: false,
                virtual_address: address,
        })?;
        
        // Convert the perms from a slice to a $ty. These functions should
        // not generate **any** instruction but just make rust happy
        let perms_wide: $ty = <$ty>::from_ne_bytes(
            convert_arrays(<[Perm; SIZE]>::try_from(perms).unwrap())
        );

        // check if we can write on all the bytes needed
        if unlikely((perms_wide & READ_WIDE) != READ_WIDE) {
            let mut permissions: [Perm; 8] = Default::default();
            permissions[..SIZE].copy_from_slice(perms);
            // TODO add non initialized
            return Err(MmuError::PermissionsFault{
                is_read: true, 
                virtual_address: address,
                permissions,
                size: SIZE,
            });
        }
    
        // write the value
        let result = <$ty>::from_le_bytes(
            self.memory[address.0..address.0 + SIZE].try_into().unwrap()
        );
        

        // taint the bit if signed as to taint
        if TAINT {
            let mut permissions_update = Perm::default();

            if self.permissions[address.0].is_superset_of(PermField::ToTaint) {
                permissions_update |= PermField::Tainted;
            }

            self.permissions[address.0] |= permissions_update;

            // Dirty the memory because we changed the permissions
            self.dirty.dirty(address.0 / DIRTY_BLOCK_SIZE);
        }

        Ok(result)
    }

    #[inline]
    fn write(&mut self, address: VirtAddr, value: $ty) -> Result<(), MmuError> {
        const SIZE: usize = core::mem::size_of::<$ty>();
        const WRITE_WIDE: $ty = to_wide_perm!(PermField::Write as u8, $ty);
        const RAW_WRITE_WIDE: $ty = to_wide_perm!(
            PermField::Write as u8 | PermField::ReadAfterWrite as u8, $ty
        );

        // Get the permissions while checking for out of bounds
        let perms = self.permissions.get(address.0..address.0 + SIZE)
            .ok_or_else(|| MmuError::OutOfBound{
                is_read: false,
                virtual_address: address,
        })?;
        
        // Convert the perms from a slice to a $ty. These functions should
        // not generate **any** instruction but just make rust happy
        let perms_wide: $ty = <$ty>::from_ne_bytes(
            convert_arrays(<[Perm; SIZE]>::try_from(perms).unwrap())
        );

        // check if we can write on all the bytes needed
        if unlikely((perms_wide & WRITE_WIDE) != WRITE_WIDE) {
            let mut permissions: [Perm; 8] = Default::default();
            permissions[..SIZE].copy_from_slice(perms);
            return Err(MmuError::PermissionsFault{
                is_read: false, 
                permissions,
                virtual_address: address,
                size: SIZE,
            });
        }
    
        // write the value in memory
        self.memory[address.0..address.0 + SIZE]
            .copy_from_slice(&value.to_le_bytes());
        
        // update the dirty bitmap and push memory
        if RAW {
            // check all the bytes in a single step
            if (perms_wide & RAW_WRITE_WIDE) == RAW_WRITE_WIDE {
                // bad hack to set the Read bytes on only the RAW bytes
                // this relays on the representation of the perms so 
                // at least I added some debug asserts, ideally this should be
                // a static_assert tho. 
                debug_assert_eq!(PermField::Read as u8, 1);
                debug_assert_eq!(PermField::ReadAfterWrite as u8, 8);
                let update = (perms_wide & RAW_WRITE_WIDE) >> 3;
                // This sucks but the compiler should gen a single move
                for (i, byte) in update.to_ne_bytes().iter().enumerate() {
                    self.permissions[address.0 + i] |= Perm(*byte);
                }
            }
        }

        // Update the dirty list
        self.dirty.dirty(address.0 / DIRTY_BLOCK_SIZE);

        Ok(())
    }
}
)*
    };
}

impl_read_write!{
    u8, u16, u32, u64
}