use core::ops::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Enum of the bit-fields in a Perm u8
pub enum PermField {
    /// If the current byte can be read
    Read           = 1 << 0,
    /// If the current byte can be written
    Write          = 1 << 1,
    /// If the current byte can be executed
    Executable     = 1 << 2,
    /// If the current byte can be **Read only After it was Written to**
    ReadAfterWrite = 1 << 3,
    /// Access bit, register if a bit is read. This is used to do taint tracking
    Accessed       = 1 << 4,
}

// Convertion utilities
impl From<PermField> for u8 {
    #[inline(always)]
    fn from(value: PermField) -> Self {
        value as u8
    }
}

impl BitOr<PermField> for PermField {
    type Output = Perm;

    fn bitor(self, rhs: PermField) -> Self::Output {
        Perm(self as u8 | rhs as u8)
    }
}

// Convertion utilities
impl TryFrom<u8> for PermField {
    type Error = u8;

    #[inline(always)]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            _ if value == (PermField::Read as u8) => {
                Ok(PermField::Read)
            }
            _ if value == (PermField::Write as u8) => {
                Ok(PermField::Write)
            }
            _ if value == (PermField::Executable as u8) => {
                Ok(PermField::Executable)
            }
            _ if value == (PermField::ReadAfterWrite as u8) => {
                Ok(PermField::ReadAfterWrite)
            }
            _ if value == (PermField::Accessed as u8) => {
                Ok(PermField::Accessed)
            }
            x @ _ => {
                Err(x)
            }
        }
    }
}


/// A permissions byte which corresponds to a memory byte and defines the
/// permissions it has. This is a fancy way to have an u8 bitmap for the 
/// various permissions but it's less prone to bugs thanks to rust pedantic
/// compiler.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Perm(u8);

/// By default No permissions
impl Default for Perm {
    #[inline(always)]
    fn default() -> Self {
        Perm(0)
    }
}

impl From<Perm> for u8 {
    fn from(value: Perm) -> Self {
        value.0
    }
}

impl Perm {
    pub fn is_superset_of<P: Into<u8>>(&self, other: P) -> bool {
        let other = other.into();
        (self.0 & other) == other
    }
}

impl BitOr<Perm> for Perm {
    type Output = Perm;

    #[inline(always)]
    fn bitor(self, rhs: Perm) -> Self::Output {
        Perm(self.0 | rhs.0)
    }
}

impl BitOr<PermField> for Perm {
    type Output = Perm;

    #[inline(always)]
    fn bitor(self, rhs: PermField) -> Self::Output {
        Perm(self.0 | u8::from(rhs))
    }
}

impl BitOrAssign<Perm> for Perm {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Perm) {
        self.0 = self.0 | rhs.0;
    }
}

impl BitOrAssign<PermField> for Perm {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: PermField) {
        self.0 = self.0 | rhs as u8;
    }
}