/// If the current byte can be read
pub const PERM_READ:  u8 = 1 << 0;
/// If the current byte can be written
pub const PERM_WRITE: u8 = 1 << 1;
/// If the current byte can be executed
pub const PERM_EXEC:  u8 = 1 << 2;
/// If the current byte can be **Read only After it was Written to**
pub const PERM_RAW:   u8 = 1 << 3;
/// Access bit, register if a bit is read. This is used to do taint tracking
pub const PERM_ACC:   u8 = 1 << 4;

/// A permissions byte which corresponds to a memory byte and defines the
/// permissions it has
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Perm(pub u8);

impl Default for Perm {
    fn default() -> Self {
        Perm(0)
    }
}