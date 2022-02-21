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

/// Quality of life conversion for acpi RSDP
impl TryFrom<&EfiConfigurationTable> for &acpi::RSDP {
    type Error = &'static str;
    fn try_from(value: &EfiConfigurationTable) -> Result<Self, Self::Error> {
        // check that the guid is correct
        if value.vendor_guid != EfiGuidEnum::Acpi20Table.into() 
            && value.vendor_guid != EfiGuidEnum::AcpiTable.into() {
                return Err("Invalid configuration table for RSDP conversion");
        }
        // type cast
        let rsdp: &acpi::RSDP = unsafe{&*(value.vendor_table as *const acpi::RSDP)};
        // validate the checksum
        rsdp.validate().map_err(|_| "corrupted RSDP table")?;
        Ok(rsdp)
    }
}

/// Quality of life conversion for acpi XSDP
impl TryFrom<&EfiConfigurationTable> for &acpi::XSDP {
    type Error = &'static str;
    fn try_from(value: &EfiConfigurationTable) -> Result<Self, Self::Error> {
        // check that the guid is correct
        if value.vendor_guid != EfiGuidEnum::Acpi20Table.into() {
                return Err("Invalid configuration table for XSDP conversion");
        }
        // type cast
        let xsdp: &acpi::XSDP = unsafe{&*(value.vendor_table as *const acpi::XSDP)};
        // validate the checksum
        xsdp.validate().map_err(|_| "corrupted XSDP table")?;
        Ok(xsdp)
    }
}