use super::*;

#[repr(C)]
#[derive(Debug)]
pub struct EfiConfigurationTable {
    /// The 128-bit GUID value that uniquely identifies the system
    /// configurationt able.
    pub vendor_guid: EfiGuid,

    /// A pointer to the table associated with `guid`
    pub vendor_table: usize,
}