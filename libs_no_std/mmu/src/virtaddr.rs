use core::ops::*;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A strongly typed guest address 
pub struct VirtAddr(pub usize);

impl core::fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("VirtAddr({:016x})", self.0))
    }
}

impl Add<usize> for VirtAddr {
    type Output = VirtAddr;
    fn add(self, rhs: usize) -> Self::Output {
        VirtAddr(self.0 + rhs)
    }
}

impl Sub<usize> for VirtAddr {
    type Output = VirtAddr;
    fn sub(self, rhs: usize) -> Self::Output {
        VirtAddr(self.0 - rhs)
    }
}

impl AddAssign<usize> for VirtAddr {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl SubAssign<usize> for VirtAddr {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// A strongly typed host address
pub struct HostAddr(pub usize);