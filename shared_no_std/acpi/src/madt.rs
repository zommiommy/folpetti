use crate::DescriptionHeader;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MADT {
    pub header: DescriptionHeader,
    pub local_apic_address: u32,
    pub flags: u32,
}

impl MADT {
    pub fn validate(&self) -> Result<(), u8> {
        self.header.validate()
    }

}