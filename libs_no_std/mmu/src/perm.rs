use core::ops::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Enum of the bit-fields in a Perm u8
pub enum PermField {
    /// No permissions, this is tecnically not a PermField but it's usefull
    None           = 0,
    /// If the current byte can be read
    Read           = 1 << 0,
    /// If the current byte can be written
    Write          = 1 << 1,
    /// If the current byte can be executed
    Executable     = 1 << 2,
    /// If the current byte can be **Read only After it was Written to**
    ReadAfterWrite = 1 << 3,
    /// If the current byte has to be tainted, as in if we will add the 
    /// [`PermField::Tainted`] permission field when a read or write occurs 
    /// for that byte. I deviate from Gamozo's design to mitigate the additional
    /// dirting of blocks caused by the access bit. In this case we can 
    /// surgically select the bits to tracks. This should be sensible because
    /// the main goal of taint analysis is to track which byts of the inputs
    /// are read, these bits will be dirtied anyway so it shouldn't be a
    /// big overhead.
    ToTaint        = 1 << 4,
    /// If the value had [`PermField::ToTaint`] and was accessed by a read or
    /// write
    Tainted        = 1 << 5,
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
            _ if value == (PermField::ToTaint as u8) => {
                Ok(PermField::ToTaint)
            }
            _ if value == (PermField::Tainted as u8) => {
                Ok(PermField::Tainted)
            }
            x @ _ => {
                Err(x)
            }
        }
    }
}

// /////////////////////////////////////////////////////////////////////////////
// Colleciton of perms
// /////////////////////////////////////////////////////////////////////////////

/// A permissions byte which corresponds to a memory byte and defines the
/// permissions it has. This is a fancy way to have an u8 bitmap for the 
/// various permissions but it's less prone to bugs thanks to rust pedantic
/// compiler.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Perm(pub u8);

/// Impl Debug, the impl is a bit ugly but it doens't need any allocation which
/// is nice
impl core::fmt::Debug for Perm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0 == 0 {
            return f.write_str("None");
        }
        let mut is_first = true;

        let perms = [
            PermField::Read,
            PermField::Write,
            PermField::Executable,
            PermField::ReadAfterWrite,
            PermField::Tainted,
            PermField::ToTaint,
        ];

        for perm in perms {
            if self.is_superset_of(perm) {
                if is_first {
                    is_first = false;
                } else {
                    f.write_str(" | ")?;
                }
                f.write_fmt(format_args!("{:?}", perm))?;
            }
        }
        
        Ok(())
    }
}

/// By default No permissions
impl Default for Perm {
    #[inline(always)]
    fn default() -> Self {
        PermField::None.into()
    }
}

impl From<Perm> for u8 {
    fn from(value: Perm) -> Self {
        value.0
    }
}

impl From<PermField> for Perm {
    fn from(value: PermField) -> Self {
        Perm(value.into())
    }
}

impl Perm {
    pub fn is_superset_of<P: Into<u8>>(&self, other: P) -> bool {
        let other = other.into();
        (self.0 & other) == other
    }

    pub const fn const_into_u8(self) -> u8 {
        self.0
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