/// An Efi guid representation
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq)]
pub struct EfiGuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl_enum!(
    /// EFI memory types
    EfiGuidEnum, EfiGuid,
    /// EFI_ACPI_20_TABLE_GUID
    ACPI20Table => EfiGuid{
        data1: 0x8868e871,
        data2: 0xe4f1, 
        data3: 0x11d3, 
        data4: [0xbc,0x22,0x00,0x80,0xc7,0x3c,0x88,0x81],
    },
    /// ACPI_TABLE_GUID
    ACPITable => EfiGuid{
        data1: 0xeb9d2d30,
        data2: 0x2d88, 
        data3: 0x2d88, 
        data4: [0x9a,0x16,0x00,0x90,0x27,0x3f,0xc1,0x4d],
    },
    /// SAL_SYSTEM_TABLE_GUID
    SalSystemTable => EfiGuid{
        data1: 0xeb9d2d32,
        data2: 0x2d88, 
        data3: 0x11d3, 
        data4: [0x9a,0x16,0x00,0x90,0x27,0x3f,0xc1,0x4d],
    },
    /// SMBIOS_TABLE_GUID
    SMBIOSTable => EfiGuid{
        data1: 0xeb9d2d31,
        data2: 0x2d88, 
        data3: 0x11d3, 
        data4: [0x9a,0x16,0x00,0x90,0x27,0x3f,0xc1,0x4d],
    },
    /// SMBIOS3_TABLE_GUID
    SMBIOS3Table => EfiGuid{
        data1: 0xeb9d2d31,
        data2: 0x2d88, 
        data3: 0x11d3, 
        data4: [0x9a,0x16,0x00,0x90,0x27,0x3f,0xc1,0x4d],
    },
    /// MPS_TABLE_GUID
    MPSTable => EfiGuid{
        data1: 0xeb9d2d2f,
        data2: 0x2d88, 
        data3: 0x11d3, 
        data4: [0x9a,0x16,0x00,0x90,0x27,0x3f,0xc1,0x4d],
    },
    /// EFI_ACPI_TABLE_GUID
    EfiACPITable => EfiGuid{
        data1: 0x8868e871,
        data2: 0xe4f1, 
        data3: 0x11d3, 
        data4: [0xbc,0x22,0x00,0x80,0xc7,0x3c,0x88,0x81],
    },
);
