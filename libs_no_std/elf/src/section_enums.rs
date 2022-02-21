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

    /// Relocation entries; only offsets.
    SHT_RELR,        

    /// Values in this inclusive range (0x60000000..=0x6fffffff) are reserved 
    /// for operating system-specific semantics.
    SHT_OS(u32),

    SHT_ANDROID_REL,
    SHT_ANDROID_RELA,
    
    /// LLVM ODR table.
    SHT_LLVM_ODRTAB,
    /// LLVM Linker Options.
    SHT_LLVM_LINKER_OPTIONS,
    /// List of address-significant symbols or safe ICF.
    SHT_LLVM_ADDRSIG,
    /// LLVM Dependent Library Specifiers.
    SHT_LLVM_DEPENDENT_LIBRARIES,
    /// Symbol partition specification.
    SHT_LLVM_SYMPART,
    /// ELF header for loadable partition.
    SHT_LLVM_PART_EHDR,
    /// Phdrs for loadable partition.
    SHT_LLVM_PART_PHDR,
    /// LLVM Basic Block Address Map.
    SHT_LLVM_BB_ADDR_MAP,
    /// LLVM Call Graph Profile.
    SHT_LLVM_CALL_GRAPH_PROFILE,
    
    /// Relocation entries; only offsets.
    SHT_ANDROID_RELR,

    /// Object attributes.
    SHT_GNU_ATTRIBUTES,
    /// GNU-style hash table.
    SHT_GNU_HASH,
    /// GNU version definitions.
    SHT_GNU_verdef,
    /// GNU version references.
    SHT_GNU_verneed,
    /// GNU symbol versions table.
    SHT_GNU_versym,
    
    /// Values in this inclusive range (0x70000000..=0x7fffffff) are reserved 
    /// for processor-specific semantics.
    SHT_PROC(u32),
    
    /// Exception Index table
    SHT_ARM_EXIDX,
    // BPABI DLL dynamic linking pre-emption map
    SHT_ARM_PREEMPTMAP,
    //  Object file compatibility attributes
    SHT_ARM__RISCV_MSP430_ATTRIBUTES,

    SHT_ARM_DEBUGOVERLAY,
    SHT_ARM_OVERLAYSECTION,
    /// Link editor is to sort the entries in this section based on their sizes
    SHT_HEX_ORDERED ,   
    /// Unwind information
    SHT_X86_64_UNWIND,
    /// Register usage information
    SHT_MIPS_REGINFO,  
    /// General options
    SHT_MIPS_OPTIONS,  
    /// DWARF debugging section.
    SHT_MIPS_DWARF,    
    /// ABI information.
    SHT_MIPS_ABIFLAGS, 


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
            19         => ELFSectionType::SHT_RELR,

            0x60000001 => ELFSectionType::SHT_ANDROID_REL,
            0x60000002 => ELFSectionType::SHT_ANDROID_RELA,
            0x6fff4c00 => ELFSectionType::SHT_LLVM_ODRTAB,
            0x6fff4c01 => ELFSectionType::SHT_LLVM_LINKER_OPTIONS,
            0x6fff4c03 => ELFSectionType::SHT_LLVM_ADDRSIG,
            0x6fff4c04 => ELFSectionType::SHT_LLVM_DEPENDENT_LIBRARIES,
            0x6fff4c05 => ELFSectionType::SHT_LLVM_SYMPART,
            0x6fff4c06 => ELFSectionType::SHT_LLVM_PART_EHDR,
            0x6fff4c07 => ELFSectionType::SHT_LLVM_PART_PHDR,
            0x6fff4c08 => ELFSectionType::SHT_LLVM_BB_ADDR_MAP,
            0x6fff4c09 => ELFSectionType::SHT_LLVM_CALL_GRAPH_PROFILE,
            0x6fffff00 => ELFSectionType::SHT_ANDROID_RELR,
            0x6ffffff5 => ELFSectionType::SHT_GNU_ATTRIBUTES,
            0x6ffffff6 => ELFSectionType::SHT_GNU_HASH,
            0x6ffffffd => ELFSectionType::SHT_GNU_verdef,
            0x6ffffffe => ELFSectionType::SHT_GNU_verneed,
            0x6fffffff => ELFSectionType::SHT_GNU_versym,

            0x70000001 => ELFSectionType::SHT_ARM_EXIDX,
            0x70000002 => ELFSectionType::SHT_ARM_PREEMPTMAP,
            0x70000003 => ELFSectionType::SHT_ARM__RISCV_MSP430_ATTRIBUTES,
            0x70000004 => ELFSectionType::SHT_ARM_DEBUGOVERLAY,
            0x70000005 => ELFSectionType::SHT_ARM_OVERLAYSECTION,
            0x70000000 => ELFSectionType::SHT_HEX_ORDERED,
            0x70000001 => ELFSectionType::SHT_X86_64_UNWIND,
            0x70000006 => ELFSectionType::SHT_MIPS_REGINFO,
            0x7000000d => ELFSectionType::SHT_MIPS_OPTIONS,
            0x7000001e => ELFSectionType::SHT_MIPS_DWARF,
            0x7000002a => ELFSectionType::SHT_MIPS_ABIFLAGS,
             
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
            ELFSectionType::SHT_NULL                         =>  0,
            ELFSectionType::SHT_PROGBITS                     =>  1,
            ELFSectionType::SHT_SYMTAB                       =>  2,
            ELFSectionType::SHT_STRTAB                       =>  3,
            ELFSectionType::SHT_RELA                         =>  4,
            ELFSectionType::SHT_HASH                         =>  5,
            ELFSectionType::SHT_DYNAMIC                      =>  6,
            ELFSectionType::SHT_NOTE                         =>  7,
            ELFSectionType::SHT_NOBITS                       =>  8,
            ELFSectionType::SHT_REL                          =>  9,
            ELFSectionType::SHT_SHLIB                        => 10,
            ELFSectionType::SHT_DYNSYM                       => 11,
            ELFSectionType::SHT_INIT_ARRAY                   => 14,
            ELFSectionType::SHT_FINI_ARRAY                   => 15,
            ELFSectionType::SHT_PREINIT_ARRAY                => 16,
            ELFSectionType::SHT_GROUP                        => 17,
            ELFSectionType::SHT_SYMTAB_SHNDX                 => 18,
            ELFSectionType::SHT_RELR                         => 19,

            ELFSectionType::SHT_ANDROID_REL                  => 0x60000001,
            ELFSectionType::SHT_ANDROID_RELA                 => 0x60000002,
            ELFSectionType::SHT_LLVM_ODRTAB                  => 0x6fff4c00,
            ELFSectionType::SHT_LLVM_LINKER_OPTIONS          => 0x6fff4c01,
            ELFSectionType::SHT_LLVM_ADDRSIG                 => 0x6fff4c03,
            ELFSectionType::SHT_LLVM_DEPENDENT_LIBRARIES     => 0x6fff4c04,
            ELFSectionType::SHT_LLVM_SYMPART                 => 0x6fff4c05,
            ELFSectionType::SHT_LLVM_PART_EHDR               => 0x6fff4c06,
            ELFSectionType::SHT_LLVM_PART_PHDR               => 0x6fff4c07,
            ELFSectionType::SHT_LLVM_BB_ADDR_MAP             => 0x6fff4c08,
            ELFSectionType::SHT_LLVM_CALL_GRAPH_PROFILE      => 0x6fff4c09,
            ELFSectionType::SHT_ANDROID_RELR                 => 0x6fffff00,
            ELFSectionType::SHT_GNU_ATTRIBUTES               => 0x6ffffff5,
            ELFSectionType::SHT_GNU_HASH                     => 0x6ffffff6,
            ELFSectionType::SHT_GNU_verdef                   => 0x6ffffffd,
            ELFSectionType::SHT_GNU_verneed                  => 0x6ffffffe,
            ELFSectionType::SHT_GNU_versym                   => 0x6fffffff,
            
            ELFSectionType::SHT_ARM_EXIDX                    => 0x70000001,
            ELFSectionType::SHT_ARM_PREEMPTMAP               => 0x70000002,
            ELFSectionType::SHT_ARM__RISCV_MSP430_ATTRIBUTES => 0x70000003,
            ELFSectionType::SHT_ARM_DEBUGOVERLAY             => 0x70000004,
            ELFSectionType::SHT_ARM_OVERLAYSECTION           => 0x70000005,
            ELFSectionType::SHT_HEX_ORDERED                  => 0x70000000,   
            ELFSectionType::SHT_X86_64_UNWIND                => 0x70000001, 
            ELFSectionType::SHT_MIPS_REGINFO                 => 0x70000006,  
            ELFSectionType::SHT_MIPS_OPTIONS                 => 0x7000000d,  
            ELFSectionType::SHT_MIPS_DWARF                   => 0x7000001e,  
            ELFSectionType::SHT_MIPS_ABIFLAGS                => 0x7000002a, 

            ELFSectionType::SHT_OS(val)   => val,
            ELFSectionType::SHT_PROC(val) => val,
            ELFSectionType::SHT_USER(val) => val,
            ELFSectionType::UNKNOWN(val)  => val,
        }
    }
}

impl_enum!(
    /// Bitfields inside of a [`ELFSectionAttributeFlags`]
    ELFSectionAttributeFlagsField, u64, 
    /// Section data should be writable during execution.
    SHF_WRITE => 0x1,
    /// Section occupies memory during program execution.
    SHF_ALLOC => 0x2,
    /// Section contains executable machine instructions.
    SHF_EXECINSTR => 0x4,
    /// The data in this section may be merged.
    SHF_MERGE => 0x10,
    /// The data in this section is null-terminated strings.
    SHF_STRINGS => 0x20,
    /// A field in this section holds a section header table index.
    SHF_INFO_LINK => 0x40,
    /// Adds special ordering requirements for link editors.
    SHF_LINK_ORDER => 0x80,
    /// This section requires special OS-specific processing to avoid incorrect
    /// behavior.
    SHF_OS_NONCONFORMING => 0x100,
    /// This section is a member of a section group.
    SHF_GROUP => 0x200,
    /// This section holds Thread-Local Storage.
    SHF_TLS => 0x400,
    /// Identifies a section containing compressed data.
    SHF_COMPRESSED => 0x800,
    /// This section should not be garbage collected by the linker.
    SHF_GNU_RETAIN => 0x200000,
    /// This section is excluded from the final executable or shared library.
    SHF_EXCLUDE => 0x80000000,
    /// Start of target-specific flags.
    SHF_MASKOS => 0x0ff00000,
    /// Bits indicating processor-specific flags.
    SHF_MASKPROC => 0xf0000000,
    /// All sections with the "d" flag are grouped together by the linker to 
    /// form the data section and the dp register is set to the start of the 
    /// section by the boot code.
    XCORE_SHF_DP_SECTION => 0x10000000,
    /// All sections with the "c" flag are grouped together by the linker to 
    /// form the constant pool and the cp register is set to the start of the 
    /// constant pool by the boot code.
    XCORE_SHF_CP_SECTION => 0x20000000,
    /// If an object file section does not have this flag set, then it may not 
    /// hold more than 2GB and can be freely referred to in objects using 
    /// smaller code models. Otherwise, only objects using larger code models 
    /// can refer to them. For example, a medium code model object can refer to 
    /// data in a section that sets this flag besides being able to refer to 
    /// data in a section that does not set it; likewise, a small code model 
    /// object can refer only to code in a section that does not set this flag.
    SHF_X86_64_LARGE => 0x10000000,
    /// All sections with the GPREL flag are grouped into a global data area
    /// for faster accesses
    SHF_HEX_GPREL => 0x10000000,
    /// Section contains text/data which may be replicated in other sections.
    /// Linker must retain only one copy.
    SHF_MIPS_NODUPES => 0x01000000,
    /// Linker must generate implicit hidden weak names.
    SHF_MIPS_NAMES => 0x02000000,
    /// Section data local to process.
    SHF_MIPS_LOCAL => 0x04000000,
    /// Do not strip this section.
    SHF_MIPS_NOSTRIP => 0x08000000,
    /// Section must be part of global data area.
    SHF_MIPS_GPREL => 0x10000000,
    /// This section should be merged.
    SHF_MIPS_MERGE => 0x20000000,
    /// Address size to be inferred from section entry size.
    SHF_MIPS_ADDR => 0x40000000,
    /// Section data is string data by default.
    SHF_MIPS_STRING => 0x80000000,
    /// Make code section unreadable when in execute-only mode
    SHF_ARM_PURECODE => 0x20000000,
);

#[derive(Debug, Clone, Copy)]
pub struct ELFSectionAttributeFlags(u64);

impl ELFSectionAttributeFlags {
    pub fn is_superset_of<P: Into<u64>>(&self, other: P) -> bool {
        let other = other.into();
        (self.0 & other) == other
    }
}

impl From<ELFSectionAttributeFlagsField> for ELFSectionAttributeFlags {
    fn from(value: ELFSectionAttributeFlagsField) -> Self {
        ELFSectionAttributeFlags(value.into())
    }
}

impl TryFrom<ELFSectionAttributeFlags> for ELFSectionAttributeFlagsField {
    fn try_from(value: ELFSectionAttributeFlagsField) -> Self {
        ELFSectionAttributeFlags(value.into())
    }
}
