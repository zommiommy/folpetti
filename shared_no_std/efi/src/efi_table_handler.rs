/// Data structure that precedes all of the standard EFI table types.
#[derive(Debug)]
#[repr(C)]
pub struct EfiTableHeader {
    /// A 64-bit signature that identifies the type of table that follows.
    /// Unique signatures have been generate for the EFI System Table, the EFI
    /// Boot Services Table, and the EFI Runtime Services Table
    pub signature: u64,

    /// The revision of the EFI Specification to which this table conforms. The
    /// upper 16 bits of this field contains the major revision value, and the 
    /// lower 16 bits contain the minor revision value. The minor revision 
    /// values are binary coded decimales that are limited to the range of
    /// 00..99
    ///
    /// When printed or displayed UEFI spec revision is referred as <Major
    /// revision>.<Minor revision upper decimal>.<Minor revision lower decimal
    /// or 
    /// <Major revision>.<Minor revision upper decimal> in case Minor revision 
    /// lower decimal is set to 0. For example:
    ///
    /// A specification with the revision value ((2<<16) | 30) would be 
    /// referred as 2.3;
    ///
    /// A specification with the revision value ((2<<16) | 31) would be
    /// referred as 2.3.1
    pub revision: u32,

    /// The size, in bytes of the entire table including the EfiTableHeader`
    pub header_size: u32,

    /// The 32-bit CRC for the entire table. This value is computed by setting
    /// this field to 0, and computing the 32-bit CRC for `header_size` bytes.
    pub crc32: u32,

    /// Reserved field that must be set to 0.
    pub reserved: u32,
}
