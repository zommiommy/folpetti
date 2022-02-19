
#[repr(C, packed)]
#[derive(Clone, Copy, PartialEq, Eq)]
/// APIC 1.0 pointer to RSDT
pub struct RDSP {
    /// This should always be “RSD PTR ”
    pub signature: [u8; 8],
    /// This is the checksum of the fields defined in the ACPI 1.0
    /// specification. This includes only the first 20 bytes of this table,
    /// bytes 0 to 19, including the checksum field. These bytes must sum
    /// to zero.
    pub checksum: u8,
    /// An OEM-supplied string that identifies the OEM.
    pub oem_id: [u8; 6],
    /// The revision of this structure. Larger revision numbers are
    /// backward compatible to lower revision numbers. The ACPI version
    /// 1.0 revision number of this table is zero. The ACPI version 1.0 RSDP
    /// Structure only includes the first 20 bytes of this table, bytes 0 to
    /// 19. It does not include the Length field and beyond. The current
    /// value for this field is 2.
    pub revision: u8,
    /// 32 bit physical address of the RSDT.
    pub rsdt_address: u32,
}

impl RDSP {
    /// Validate the checksum and signature
    pub fn validate(&self) -> Result<(), u8> {
        if &self.signature != b"RSD PTR " {
            return Err(0);
        }

        // TODO!: figure this out
        let sum = unsafe{
            let base_addr = self as *const Self as usize;
            (base_addr..base_addr + core::mem::size_of::<Self>())
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

impl core::fmt::Debug for RDSP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut fmt = f.debug_struct("RDSP");

        let signature = self.signature;
        if let Ok(as_str) = core::str::from_utf8(&signature) {
            fmt.field("signature", &format_args!("{:?}", as_str));
        } else {
            fmt.field("signature", &signature);
        }
        let checksum = self.checksum;
        fmt.field("checksum", &checksum);

        let oem_id = self.oem_id;
        if let Ok(as_str) = core::str::from_utf8(&oem_id) {
            fmt.field("oem_id", &format_args!("{:?}", as_str));
        } else {
            fmt.field("oem_id", &oem_id);
        }
        let revision = self.revision;
        fmt.field("revision", &revision);
        let rsdt_address = self.rsdt_address;
        fmt.field("rsdt_address", &rsdt_address);

        fmt.finish()
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, PartialEq, Eq)]
/// APIC 2.0 pointer to RDST and XSDT
pub struct XSDP {
    /// This should always be “RSD PTR ”
    pub signature: [u8; 8],
    /// This is the checksum of the fields defined in the ACPI 1.0
    /// specification. This includes only the first 20 bytes of this table,
    /// bytes 0 to 19, including the checksum field. These bytes must sum
    /// to zero.
    pub checksum: u8,
    /// An OEM-supplied string that identifies the OEM.
    pub oem_id: [u8; 6],
    /// The revision of this structure. Larger revision numbers are
    /// backward compatible to lower revision numbers. The ACPI version
    /// 1.0 revision number of this table is zero. The ACPI version 1.0 RSDP
    /// Structure only includes the first 20 bytes of this table, bytes 0 to
    /// 19. It does not include the Length field and beyond. The current
    /// value for this field is 2.
    pub revision: u8,
    /// 32 bit physical address of the RSDT.
    pub rsdt_address: u32,
    /// The length of the table, in bytes, including the header, starting
    /// from offset 0. This field is used to record the size of the entire
    /// table. This field is not available in the ACPI version 1.0 RSDP
    /// Structure.
    pub length: u32,
    /// 64 bit physical address of the XSDT.
    pub xsdt_address: u64,
    /// This is a checksum of the entire table, including both checksum
    /// fields.
    pub extended_checksum: u8,
    /// Reserved field
    pub reserved: [u8; 3],
}

impl XSDP {
    /// Validate the checksum and signature
    pub fn validate(&self) -> Result<(), u8> {
        if &self.signature != b"RSD PTR " {
            return Err(0);
        }

        // TODO!: figure this out
        let sum = unsafe{
            let base_addr = self as *const Self as usize;
            (base_addr..base_addr + core::mem::size_of::<RDSP>())
                .fold(0_u8, |acc, ptr| {
                    acc.wrapping_add(*(ptr as *const u8))
                })
        };

        if sum != 0 {
            return Err(sum);
        }

        // TODO!: figure this out
        let sum = unsafe{
            let base_addr = self as *const Self as usize;
            (base_addr + core::mem::size_of::<RDSP>()..base_addr + core::mem::size_of::<Self>())
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

impl core::fmt::Debug for XSDP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut fmt = f.debug_struct("XSDP");
        
        let signature = self.signature;
        if let Ok(as_str) = core::str::from_utf8(&signature) {
            fmt.field("signature", &format_args!("{:?}", as_str));
        } else {
            fmt.field("signature", &signature);
        }
        let checksum = self.checksum;
        fmt.field("checksum", &checksum);

        let oem_id = self.oem_id;
        if let Ok(as_str) = core::str::from_utf8(&oem_id) {
            fmt.field("oem_id", &format_args!("{:?}", as_str));
        } else {
            fmt.field("oem_id", &oem_id);
        }
        let revision = self.revision;
        fmt.field("revision", &revision);
        let rsdt_address = self.rsdt_address;
        fmt.field("rsdt_address", &rsdt_address);


        let length = self.length;
        fmt.field("length", &length);
        let xsdt_address = self.xsdt_address;
        fmt.field("xsdt_address", &xsdt_address);
        let extended_checksum = self.extended_checksum;
        fmt.field("extended_checksum", &extended_checksum);
        let reserved = self.reserved;
        fmt.field("reserved", &reserved);

        fmt.finish()
    }
}