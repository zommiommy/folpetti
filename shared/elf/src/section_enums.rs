#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ELFSectionType {
    /// Identifies the section header as inactive. This section header does not 
    /// have an associated section. 
    /// Other members of the section header have undefined values.
    SHT_NULL,

    /// Identifies information defined by the program, 
    /// whose format and meaning are determined solely by the program.
    SHT_PROGBITS,

    /// Identifies a symbol table. Typically a SHT_SYMTAB section provides 
    /// symbols for link-editing. As a complete symbol table, it can contain 
    /// many symbols unnecessary for dynamic linking.
    /// Consequently, an object file can also contain a SHT_DYNSYM section, 
    /// which holds a minimal set of dynamic linking symbols, to save space. 
    SHT_SYMTAB,

    /// Identifies a string table. An object file can have multiple 
    /// string table sections
    SHT_STRTAB,

    /// Identifies relocation entries with explicit addends, such as type 
    /// Elf32_Rela for the 32-bit class of object files. 
    /// An object file can have multiple relocation sections. 
    SHT_RELA,

    /// Identifies a symbol hash table. All dynamically linked object files must
    /// contain a symbol hash table. Currently, an object file can have only one
    /// hash table, but this restriction might be relaxed in the future.
    SHT_HASH,

    /// Identifies information for dynamic linking. Currently, an object file 
    /// can have only one dynamic section.
    SHT_DYNAMIC,

    /// Identifies information that marks the file in some way. 
    SHT_NOTE,

    /// Identifies a section that occupies no space in the file but otherwise 
    /// resembles SHT_PROGBITS. Although this section contains no bytes, 
    /// the sh_offset member contains the conceptual file offset.
    /// (This usually is equal to the offset of the next section.)
    SHT_NOBITS,

    /// Identifies relocation entries without explicit addends,
    /// such as type Elf32_Rel for the 32-bit class of object files.
    /// An object file can have multiple relocation sections.
    SHT_REL,

    /// Identifies a reserved section which has unspecified semantics. 
    /// Programs that contain a section of this type do not conform to the ABI.
    SHT_SHLIB,
    
    /// Identifies a symbol table. Typically a SHT_SYMTAB section provides 
    /// symbols for link-editing. As a complete symbol table, it can contain 
    /// many symbols unnecessary for dynamic linking.
    /// Consequently, an object file can also contain a SHT_DYNSYM section, 
    /// which holds a minimal set of dynamic linking symbols, to save space. 
    SHT_DYNSYM,

    /// This section contains an array of pointers to initialization functions.
    /// Each pointer in the array is taken as a parameterless procedure with 
    /// a void return.
    SHT_INIT_ARRAY,

    /// This section contains an array of pointers to termination functions.
    /// Each pointer in the array is taken as a parameterless procedure with 
    /// a void return.
    SHT_FINI_ARRAY,

    /// This section contains an array of pointers to functions that are invoked
    /// before all other initialization functions.
    /// Each pointer in the array is taken as a parameterless procedure with a void return.
    SHT_PREINIT_ARRAY,

    /// This section defines a section group. A section group is a set of 
    /// sections that are related and that must be treated specially by the 
    /// linker. Sections of type SHT_GROUP may appear only in relocatable 
    /// objects (objects with the ELF header e_type member set to ET_REL). 
    /// The section header table entry for a group section must appear in the 
    /// section header table before the entries for any of the sections that are
    /// members of the group.
    SHT_GROUP,

    ///This section is associated with a section of type SHT_SYMTAB and is 
    /// required if any of the section header indexes referenced by that symbol 
    /// table contain the escape value SHN_XINDEX. The section is an array of 
    /// Elf32_Word values. Each value corresponds one to one with a symbol table
    /// entry and appear in the same order as those entries. The values 
    /// represent the section header indexes against which the symbol table 
    /// entries are defined. Only if corresponding symbol table entry's st_shndx
    /// field contains the escape value SHN_XINDEX will the matching Elf32_Word
    /// hold the actual section header index; otherwise, the entry must 
    /// be SHN_UNDEF (0).
    SHT_SYMTAB_SHNDX,

    /// Values in this inclusive range (0x60000000..=0x6fffffff) are reserved 
    /// for operating system-specific semantics.
    SHT_OS(u32),

    /// Values in this inclusive range (0x70000000..=0x7fffffff) are reserved 
    /// for processor-specific semantics.
    SHT_PROC(u32),
    
    /// Values in this inclusive range (0x80000000..=0x8fffffff) are reserved 
    /// for application programs. 
    /// Section types between may be used by the application, 
    ///without conflicting with current or future system-defined section types.
    SHT_USER(u32),

    /// Unknown section type
    UNKNOWN(u32)
}

impl From<u32> for ELFSectionType {
    fn from(item: u32) -> Self {
        match item {   
            0          => ELFSectionType::SHT_NULL,
            1          => ELFSectionType::SHT_PROGBITS,
            2          => ELFSectionType::SHT_SYMTAB,
            3          => ELFSectionType::SHT_STRTAB,
            4          => ELFSectionType::SHT_RELA,
            5          => ELFSectionType::SHT_HASH,
            6          => ELFSectionType::SHT_DYNAMIC,
            7          => ELFSectionType::SHT_NOTE,
            8          => ELFSectionType::SHT_NOBITS,
            9          => ELFSectionType::SHT_REL,
            10         => ELFSectionType::SHT_SHLIB,
            11         => ELFSectionType::SHT_DYNSYM,
            14         => ELFSectionType::SHT_INIT_ARRAY,
            15         => ELFSectionType::SHT_FINI_ARRAY,
            16         => ELFSectionType::SHT_PREINIT_ARRAY,
            17         => ELFSectionType::SHT_GROUP,
            18         => ELFSectionType::SHT_SYMTAB_SHNDX,
            0x60000000..=0x6fffffff => ELFSectionType::SHT_OS(item),
            0x70000000..=0x7fffffff => ELFSectionType::SHT_PROC(item),
            0x80000000..=0x8fffffff => ELFSectionType::SHT_USER(item),
            _ => ELFSectionType::UNKNOWN(item),
        }
    }
}

impl From<ELFSectionType> for u32 {
    fn from(item: ELFSectionType) -> Self {
        match item {
            ELFSectionType::SHT_NULL          =>  0,
            ELFSectionType::SHT_PROGBITS      =>  1,
            ELFSectionType::SHT_SYMTAB        =>  2,
            ELFSectionType::SHT_STRTAB        =>  3,
            ELFSectionType::SHT_RELA          =>  4,
            ELFSectionType::SHT_HASH          =>  5,
            ELFSectionType::SHT_DYNAMIC       =>  6,
            ELFSectionType::SHT_NOTE          =>  7,
            ELFSectionType::SHT_NOBITS        =>  8,
            ELFSectionType::SHT_REL           =>  9,
            ELFSectionType::SHT_SHLIB         => 10,
            ELFSectionType::SHT_DYNSYM        => 11,
            ELFSectionType::SHT_INIT_ARRAY    => 14,
            ELFSectionType::SHT_FINI_ARRAY    => 15,
            ELFSectionType::SHT_PREINIT_ARRAY => 16,
            ELFSectionType::SHT_GROUP         => 17,
            ELFSectionType::SHT_SYMTAB_SHNDX  => 18,
            ELFSectionType::SHT_OS(val)   => val,
            ELFSectionType::SHT_PROC(val) => val,
            ELFSectionType::SHT_USER(val) => val,
            ELFSectionType::UNKNOWN(val)  => val,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
#[repr(u64)]
pub enum ELFSectionAttributeFlags {
    /// The section contains data that should be writable during process 
    ///execution.
    SHF_WRITE,
    
    /// The section occupies memory during process execution. Some control sections do not reside in the memory image of
    /// an object file; this attribute is off for those sections.
    SHF_ALLOC,

    /// The section contains executable machine instructions.
    SHF_EXECINSTR,

    /// The data in the section may be merged to eliminate duplication. 
    /// Unless the SHF_STRINGS flag is also set, the data elements in the section 
    /// are of a uniform size. The size of each element is specified in the 
    /// section header's sh_entsize field. If the SHF_STRINGS flag is also set, 
    /// the data elements consist of null-terminated character strings. The size 
    /// of each character is specified in the section header's sh_entsize field.
    /// Each element in the section is compared against other elements in 
    /// sections with the same name, type and flags. Elements that would have 
    /// identical values at program ru-time may be merged. Relocations 
    /// referencing elements of such sections must be resolved to the merged 
    /// locations of the referenced values. Note that any relocatable values, 
    /// including values that would result in run-time relocations, must be 
    /// analyzed to determine whether the run-time values would actually be 
    /// identical. An ABI-conforming object file may not depend on specific 
    /// elements being merged, and an ABI-conforming link editor may choose not 
    /// to merge specific elements.
    SHF_MERGE,

    /// The data elements in the section consist of null-terminated character strings. The size of each character is 
    /// specified in the section header's sh_entsize field.
    SHF_STRINGS,
    
    /// The sh_info field of this section header holds a section header table 
    /// index.
    SHF_INFO_LINK,

    /// This flag adds special ordering requirements for link editors. 
    /// The requirements apply if the sh_link field of this section's header 
    /// references another section (the linked-to section). If this section is 
    /// combined with other sections in the output file, it must appear in the 
    /// same relative order with respect to those sections, as the linked-to 
    /// section appears with respect to sections the linked-to section is 
    /// combined with.
    SHF_LINK_ORDER,

    /// This section requires special OS-specific processing (beyond the 
    /// standard linking rules) to avoid incorrect behavior. If this section has
    /// either an sh_type value or contains sh_flags bits in the OS-specific 
    /// ranges for those fields, and a link editor processing this section does 
    /// not recognize those values, then the link editor should reject the 
    /// object file containing this section with an error.
    SHF_OS_NONCONFORMING,

    /// This section is a member (perhaps the only one) of a section group. 
    /// The section must be referenced by a section of type SHT_GROUP. 
    /// The SHF_GROUP flag may be set only for sections contained in relocatable
    /// objects (objects with the ELF header e_type member set to ET_REL).
    SHF_GROUP,

    /// This section holds Thread-Local Storage, meaning that each separate execution flow has its own distinct instance
    /// of this data. Implementations need not support this flag.
    SHF_TLS,

    /// All bits included in this mask are reserved for operating 
    ///system-specific semantics.
    SHF_MASKOS,

    /// All bits included in this mask are reserved for processor-specific 
    /// semantics.
    /// If meanings are specified, the processor supplement explains them.
    SHF_MASK_PROC,

    // ~~~~~~~~~~ Custom flags for utility ~~~~~~~~~~~~

    /// No flag is setted
    SHF_NO_FLAGS,

    /// Multiple flags are selected => Vector of flags
    SHF_MULTIFLAGS(Vec<ELFSectionAttributeFlags>),

    /// Not used bits
    UNKNOWN(u64),
}

/// A constant vector of the possibl flags.
/// This is only needed to iterate over the enum.
const ELF_SECTION_ATTRIBUTE_FLAGS: [ELFSectionAttributeFlags; 12] = [
    ELFSectionAttributeFlags::SHF_WRITE,
    ELFSectionAttributeFlags::SHF_ALLOC,
    ELFSectionAttributeFlags::SHF_EXECINSTR,
    ELFSectionAttributeFlags::SHF_MERGE,
    ELFSectionAttributeFlags::SHF_STRINGS,
    ELFSectionAttributeFlags::SHF_INFO_LINK,
    ELFSectionAttributeFlags::SHF_LINK_ORDER,
    ELFSectionAttributeFlags::SHF_OS_NONCONFORMING,
    ELFSectionAttributeFlags::SHF_GROUP,
    ELFSectionAttributeFlags::SHF_TLS,
    ELFSectionAttributeFlags::SHF_MASKOS,
    ELFSectionAttributeFlags::SHF_MASK_PROC,
];

impl ELFSectionAttributeFlags {
    pub fn to_flags(mut val: u64) -> Vec<ELFSectionAttributeFlags> {
        let mut result = Vec::new();

        for flag in &ELF_SECTION_ATTRIBUTE_FLAGS {
            let flag_num = u64::from(flag.clone());
            if 0 != (val & flag_num) {
                result.push(flag.clone());
                val ^= flag_num;
            }    
        }
        
        if val != 0 {
            result.push(
                ELFSectionAttributeFlags::UNKNOWN(val)
            );
        }

        if result.is_empty() {
            result.push(ELFSectionAttributeFlags::SHF_NO_FLAGS);
        }

        result
    }
}

impl From<u64> for ELFSectionAttributeFlags {
    fn from(item: u64) -> Self {
        let flags = ELFSectionAttributeFlags::to_flags(item);

        if flags.len() == 1 {
            flags[0].clone()
        } else {
            ELFSectionAttributeFlags::SHF_MULTIFLAGS(flags)
        }
    }
}

impl From<ELFSectionAttributeFlags> for u64 {
    fn from(item: ELFSectionAttributeFlags) -> Self {
        match item {
            ELFSectionAttributeFlags::SHF_NO_FLAGS         =>  0x0,
            ELFSectionAttributeFlags::SHF_WRITE            =>  0x1,
            ELFSectionAttributeFlags::SHF_ALLOC            =>  0x2,
            ELFSectionAttributeFlags::SHF_EXECINSTR        =>  0x4,
            ELFSectionAttributeFlags::SHF_MERGE            =>  0x10,
            ELFSectionAttributeFlags::SHF_STRINGS          =>  0x20,
            ELFSectionAttributeFlags::SHF_INFO_LINK        =>  0x40,
            ELFSectionAttributeFlags::SHF_LINK_ORDER       =>  0x80,
            ELFSectionAttributeFlags::SHF_OS_NONCONFORMING =>  0x100,
            ELFSectionAttributeFlags::SHF_GROUP            =>  0x200,
            ELFSectionAttributeFlags::SHF_TLS              =>  0x400,
            ELFSectionAttributeFlags::SHF_MASKOS           =>  0x0ff00000,
            ELFSectionAttributeFlags::SHF_MASK_PROC        =>  0xf0000000,
            ELFSectionAttributeFlags::UNKNOWN(val)        => val,
            ELFSectionAttributeFlags::SHF_MULTIFLAGS(val) => {
                let mut result = 0;
                for flag in val {
                    result |= u64::from(flag);
                }
                result
            },
        }
    }
}