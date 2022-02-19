
#[repr(C, packed)]
#[derive(Clone, Copy, PartialEq, Eq)]
/// All system description tables begin with this struct
pub struct DescriptionHeader {
    pub signature: [u8; 4],
    /// Length, in bytes, of the entire RSDT. The length implies the
    /// number of Entry fields (n) at the end of the table.
    pub length: u32, 
    /// This should always be 1
    pub revision: u8,
    /// Entire table must sum to zero.
    pub checksum: u8, 
    /// OEM ID
    pub oem_id: [u8; 6],
    /// For the RSDT, the table ID is the manufacture model ID. This
    /// field must match the OEM Table ID in the FADT.
    pub oem_table_id: [u8; 8],
    /// OEM revision of RSDT table for supplied OEM Table ID.
    pub oem_revision: u32,
    /// Vendor ID of utility that created the table. For tables
    /// containing Definition Blocks, this is the ID for the ASL Compiler.
    pub creator_id: [u8; 4],
    /// Revision of utility that created the table. For tables containing
    /// Definition Blocks, this is the revision for the ASL Compiler.
    pub creator_revision: u32,
}

impl DescriptionHeader {
    /// Validate the checksum and signature
    pub fn validate(&self) -> Result<(), u8> {
        let length = self.length as usize;
        let sum = unsafe{
            let base_addr = self as *const Self as usize;
            (base_addr..base_addr + length)
                .fold(0_u8, |acc, ptr| {
                    acc.wrapping_add(*(ptr as *const u8))
                })
        };

        if sum != 0 {
            return Err(sum);
        }

        Ok(())
    }
}

impl core::fmt::Debug for DescriptionHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut fmt = f.debug_struct("DescriptionHeader");

        let signature = self.signature;
        if let Ok(as_str) = core::str::from_utf8(&signature) {
            fmt.field("signature", &format_args!("{:?}", as_str));
        } else {
            fmt.field("signature", &signature);
        }

        let length = self.length;
        fmt.field("length", &length);
        let revision = self.revision;
        fmt.field("revision", &revision);
        let checksum = self.checksum;
        fmt.field("checksum", &checksum);
        
        let oem_id = self.oem_id;
        if let Ok(as_str) = core::str::from_utf8(&oem_id) {
            fmt.field("oem_id", &format_args!("{:?}", as_str));
        } else {
            fmt.field("oem_id", &oem_id);
        }
        let oem_table_id = self.oem_table_id;
        if let Ok(as_str) = core::str::from_utf8(&oem_table_id) {
            fmt.field("oem_table_id", &format_args!("{:?}", as_str));
        } else {
            fmt.field("oem_table_id", &oem_table_id);
        }

        let oem_revision = self.oem_revision;
        fmt.field("oem_revision", &oem_revision);
        let creator_id = self.creator_id;
        if let Ok(as_str) = core::str::from_utf8(&creator_id) {
            fmt.field("creator_id", &format_args!("{:?}", as_str));
        } else {
            fmt.field("creator_id", &creator_id);
        }
        let creator_revision = self.creator_revision;
        fmt.field("creator_revision", &creator_revision);

        fmt.finish()
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// APIC 1.0 Root System Description Table (RSDT)
/// 
/// At the end of this struct there is a list of entries as documented:
/// ```
/// An array of 32-bit physical addresses that point to other
/// DESCRIPTION_HEADERs. OSPM assumes at least the
/// DESCRIPTION_HEADER is addressable, and then can further
/// address the table based upon its Length field.
/// ```
pub struct RSDT {
    pub header: DescriptionHeader,
}

impl RSDT {
    pub fn validate(&self) -> Result<(), u8> {
        self.header.validate()
    }

    pub fn get_number_of_entries(&self) -> usize {
        let payload_length = (self.header.length as usize)
            .checked_sub(core::mem::size_of::<Self>())
            .expect("Integer underflow in table length");
        assert!(payload_length % 4 == 0, "unaligned RSDT length");
        payload_length / 4
    }

    pub fn get_entries(&self) -> &[u32] {
        unsafe{
            let base_address = (self as *const Self as *const u8)
                .add(core::mem::size_of::<Self>()) as *const u32;
            core::slice::from_raw_parts(
                base_address,
                self.get_number_of_entries(),
            )
        }
    }
}



#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// APIC 2.0 Extended Root System Description Table (XSDT)
/// 
/// At the end of this struct there is a list of entries as documented:
/// ```
/// An array of 64-bit physical addresses that point to other
/// DESCRIPTION_HEADERs. OSPM assumes at least the
/// DESCRIPTION_HEADER is addressable, and then can further
/// address the table based upon its Length field.
/// ```
pub struct XSDT {
    pub header: DescriptionHeader,
}

impl XSDT {
    pub fn validate(&self) -> Result<(), u8> {
        self.header.validate()
    }

    pub fn get_number_of_entries(&self) -> usize {
        let payload_length = (self.header.length as usize).checked_sub(
            core::mem::size_of::<Self>())
            .expect("Integer underflow on table length");

        assert!(payload_length % 8 == 0, "unaligned XSDT length");
        payload_length / 8
    }

    pub fn get_entries(&self) -> &[*const DescriptionHeader] {
        unsafe{
            let base_address = (self as *const Self as *const u8)
                .add(core::mem::size_of::<Self>()) 
                as *const *const DescriptionHeader;
            core::slice::from_raw_parts(
                base_address,
                self.get_number_of_entries(),
            )
        }
    }
}