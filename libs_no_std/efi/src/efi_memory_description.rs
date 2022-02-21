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

/// EfiMemoryDescriptor but with usable type
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MemoryDescriptor {
    /// Type of the memory region. Type EFI_MEMORY_TYPE is defined in the
    /// AllocatePages() function description.
    pub typ: EfiMemoryType,

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
    pub attribute: EfiMemoryAttributes,
}

impl From<EfiMemoryDescriptor> for MemoryDescriptor {
    fn from(other: EfiMemoryDescriptor) -> MemoryDescriptor {
        MemoryDescriptor{
            typ: other.typ.into(),
            physical_start: other.physical_start,
            virtual_start: other.virtual_start,
            number_of_pages: other.number_of_pages,
            attribute: other.attribute.into(),
        }
    }
}


impl_enum!{
    /// EFI memory types
    EfiMemoryType, u32,
    /// Not used.
    ReservedMemoryType => 0,

    /// The code portions of a loaded application. (Note that UEFI OS loaders 
    /// are UEFI applications.)
    LoaderCode => 1,

    /// The data portions of a loaded application and the default data 
    /// allocation type used by an application to allocate pool memory.
    LoaderData => 2,

    /// The code portions of a loaded Boot Services Driver.
    BootServicesCode => 3,

    /// The data portions of a loaded Boot Serves Driver, and the default data 
    /// allocation type used by a Boot Services Driver to allocate pool memory.
    BootServicesData => 4,

    /// The code portions of a loaded Runtime Services Driver.
    RuntimeServicesCode => 5,

    /// The data portions of a loaded Runtime Services Driver and the default 
    /// data allocation type used by a Runtime Services Driver to allocate pool 
    /// memory.
    RuntimeServicesData => 6,

    /// Free (unallocated) memory.
    ConventionalMemory => 7,

    /// Memory in which errors have been detected.
    UnusableMemory => 8,

    /// Memory that holds the ACPI tables.
    ACPIReclaimMemory => 9,

    /// Address space reserved for use by the firmware.
    ACPIMemoryNVS => 10,

    /// Used by system firmware to request that a memory-mapped IO region be 
    /// mapped by the OS to a virtual address so it can be accessed by EFI 
    /// runtime services.
    MemoryMappedIO => 11,

    /// System memory-mapped IO region that is used to translate memory cycles 
    /// to IO cycles by the processor.
    MemoryMappedIOPortSpace => 12,

    /// Address space reserved by the firmware for code that is part of the 
    /// processor.
    PalCode => 13,

    /// A memory region that operates as EfiConventionalMemory, however it 
    /// happens to also support byte-addressable non-volatility.
    PersistentMemory => 14,
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

/// A wrapper for a bitfield of the [`EfiMemoryAttribute`] flags.
/// This is just a thin wrapper to get pretty debug and quality of life methods
#[derive(Clone, Copy, Default)]
pub struct EfiMemoryAttributes(u64);

impl From<u64> for EfiMemoryAttributes {
    fn from(x: u64) -> Self {
        EfiMemoryAttributes(x)
    }
}

impl core::ops::BitAnd<EfiMemoryAttribute> for EfiMemoryAttributes {
    type Output = bool;
    fn bitand(self, rhs: EfiMemoryAttribute) -> Self::Output {
        (self.0 & u64::from(rhs)) != 0
    }
}

impl core::ops::BitOr<EfiMemoryAttribute> for EfiMemoryAttributes {
    type Output = EfiMemoryAttributes;
    fn bitor(self, rhs: EfiMemoryAttribute) -> Self::Output {
        EfiMemoryAttributes(self.0 | u64::from(rhs))
    }
}

impl core::fmt::Debug for EfiMemoryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut fmt = f.debug_list();
        use EfiMemoryAttribute::*;

        if (self.0 & u64::from(NotCachable)) != 0 {
            fmt.entry(&NotCachable);
        }
        if (self.0 & u64::from(WriteCombining)) != 0 {
            fmt.entry(&WriteCombining);
        }
        if (self.0 & u64::from(WriteThrough)) != 0 {
            fmt.entry(&WriteThrough);
        }
        if (self.0 & u64::from(WriteBack)) != 0 {
            fmt.entry(&WriteBack);
        }
        if (self.0 & u64::from(FetchAndAdd)) != 0 {
            fmt.entry(&FetchAndAdd);
        }
        if (self.0 & u64::from(WriteProtected)) != 0 {
            fmt.entry(&WriteProtected);
        }
        if (self.0 & u64::from(ReadProtected)) != 0 {
            fmt.entry(&ReadProtected);
        }
        if (self.0 & u64::from(ExecProtected)) != 0 {
            fmt.entry(&ExecProtected);
        }
        if (self.0 & u64::from(NonVolatile)) != 0 {
            fmt.entry(&NonVolatile);
        }
        if (self.0 & u64::from(MoreRelaiable)) != 0 {
            fmt.entry(&MoreRelaiable);
        }
        if (self.0 & u64::from(ReadOnlyProtected)) != 0 {
            fmt.entry(&ReadOnlyProtected);
        }
        if (self.0 & u64::from(SpecificPurpose)) != 0 {
            fmt.entry(&SpecificPurpose);
        }
        if (self.0 & u64::from(CpuCrypto)) != 0 {
            fmt.entry(&CpuCrypto);
        }
        if (self.0 & u64::from(Runtime)) != 0 {
            fmt.entry(&Runtime);
        }

        fmt.finish()
    }
}


impl_enum!(
    /// EFI memory types
    EfiMemoryAttribute, u64,
    /// EFI_MEMORY_UC: Memory cacheability attribute: The memory region 
    /// supports being configured as not cacheable.
    NotCachable => 0x0000000000000001,

    /// EFI_MEMORY_WC: Memory cacheability attribute: The memory region 
    /// supports being configured as write combining.
    WriteCombining => 0x0000000000000002,

    /// EFI_MEMORY_WT: Memory cacheability attribute: The memory region 
    /// supports being configured as cacheable with a “write through” policy. 
    /// Writes that hit in the cache will also be written to main memory.
    WriteThrough => 0x0000000000000004,

    /// EFI_MEMORY_WB: Memory cacheability attribute: The memory region supports 
    /// being configured as cacheable with a “write back” policy. 
    /// Reads and writes that hit in the cache do not propagate to main memory. 
    /// Dirty data is written back to main memory when a new cache line is 
    /// allocated.
    WriteBack => 0x0000000000000008,

    /// EFI_MEMORY_UCE: Memory cacheability attribute: The memory region 
    /// supports being configured as not cacheable, exported, and supports 
    /// the “fetch and add” semaphore mechanism.
    FetchAndAdd => 0x0000000000000010,

    /// EFI_MEMORY_WP: Physical memory protection attribute: The memory region 
    /// supports being configured as write-protected by system hardware. This is
    /// typically used as a cacheability attribute today. The memory region
    /// supports being configured as cacheable with a "write protected"
    /// policy. Reads come from cache lines when possible, and read misses
    /// cause cache fills. Writes are propagated to the system bus and cause
    /// corresponding cache lines on all processors on the bus to be
    /// invalidated.
    WriteProtected => 0x0000000000001000,

    /// EFI_MEMORY_RP: Physical memory protection attribute: The memory region 
    /// supports being configured as read-protected by system hardware.
    ReadProtected => 0x0000000000002000,

    /// EFI_MEMORY_XP: Physical memory protection attribute: The memory region 
    /// supports being configured so it is protected by system hardware from
    /// executing code.
    ExecProtected => 0x0000000000004000,

    /// EFI_MEMORY_NV: Runtime memory attribute: The memory region refers to 
    /// persistent memory.
    NonVolatile => 0x0000000000008000,

    /// EFI_MEMORY_MORE_RELIABLE: The memory region provides higher reliability
    /// relative to other memory in the system. If all memory has the same 
    /// reliability, then this bit is not used.
    MoreRelaiable => 0x0000000000010000,

    /// EFI_MEMORY_RO: Physical memory protection attribute: The memory region 
    /// supports making this memory range read-only by system hardware.
    ReadOnlyProtected => 0x0000000000020000,

    /// EFI_MEMORY_SP: Specific-purpose memory (SPM). The memory is earmarked 
    /// for specific purposes such as for specific device drivers or 
    /// applications.
    /// The SPM attribute serves as a hint to the OS to avoid allocating this
    /// memory for core OS data or code that can not be relocated.
    /// Prolonged use of this memory for purposes other than the intended
    /// purpose may result in suboptimal platform performance.
    SpecificPurpose => 0x0000000000040000,

    /// EFI_MEMORY_CPU_CRYPTO: If this flag is set, the memory region is capable
    ///  of being protected with the CPU’s memory cryptographic
    /// capabilities. If this flag is clear, the memory region is not
    /// capable of being protected with the CPU’s memory
    /// cryptographic capabilities or the CPU does not support CPU
    /// memory cryptographic capabilities.
    CpuCrypto => 0x0000000000080000,

    /// EFI_MEMORY_RUNTIME: Runtime memory attribute: The memory region needs to 
    /// be given a virtual mapping by the operating system when
    /// SetVirtualAddressMap() is called (described in Section 8.4).
    Runtime => 0x8000000000000000,
);
