/// The memory descriptor for a record returned from `GetMemoryMap()`
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiMemoryDescriptor {
    /// Type of the memory region. Type EFI_MEMORY_TYPE is defined in the
    /// AllocatePages() function description.
    pub typ: u32,

    /// Physical address of the first byte in the memory region.
    /// PhysicalStart must be aligned on a 4kib boundaryu, and must not be above
    /// 0xfffffffffffff000. Type EFI_PHYSICAL_ADDRESS is defined in the
    /// AllocatePages() function description.
    pub physical_start: u64,

    /// Virtual address of the first byte in the memory region.
    /// VirtualStart must be aligned on a 4kib boundaryu, and must not be above
    /// 0xfffffffffffff000. Type EFI_Virtual_ADDRESS is defined in 
    /// "Related Definitions."
    pub virtual_start: u64,

    /// Number of 4KiB pages in the memory region. NumberOfPages must not be 
    /// 0, and must not be any value that would represent a memoruy page with a
    /// start address, either physical or virtual, above 0xfffffffffffff000
    pub number_of_pages: u64,

    /// Attributes of the memory region that describe the bit mask of
    /// capabilities for that memory region, and not necessarly the current.
    /// settings for that memory region. See the following
    /// "Memory Attribute Definitions."
    pub attribute: u64,
}



/// EFI memory types
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum EfiMemoryType {
    ReservedMemoryType,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
    Invalid,
}

impl EfiMemoryType {
    pub fn avail_post_exit_boot_services(&self) -> bool {
        use EfiMemoryType::*;
        match self {
            BootServicesCode |
            BootServicesData | 
            ConventionalMemory |
            PersistentMemory => true,
            _ => false
        }
    }
}

impl From<u32> for EfiMemoryType {
    fn from(val: u32) -> Self {
        use EfiMemoryType::*;
        match val {
            00 => ReservedMemoryType,
            01 => LoaderCode,
            02 => LoaderData,
            03 => BootServicesCode,
            04 => BootServicesData,
            05 => RuntimeServicesCode,
            06 => RuntimeServicesData,
            07 => ConventionalMemory,
            08 => UnusableMemory,
            09 => ACPIReclaimMemory,
            10 => ACPIMemoryNVS,
            11 => MemoryMappedIO,
            12 => MemoryMappedIOPortSpace,
            13 => PalCode,
            14 => PersistentMemory,
            _  => Invalid,
        }
    }
}