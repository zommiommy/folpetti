use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ELFHeader {
    pub magic: [u8; 4],
    pub ei_class: ELFClass,         
    pub ei_data: ELFData,
    pub ei_version: ELFIntVersion,
    pub ei_osabi: ELFOsAbi,        
    pub ei_abiversion: u8,      
    pub pad: [u8; 7],
    pub e_type: ELFType,            
    pub e_machine: ELFMachine,      
    pub e_version: ELFVersion,     
    pub e_entry: u64,              
    pub e_phoff: u64,              
    pub e_shoff: u64,               
    pub e_flags: u32,               
    pub e_ehsize: u16,              
    pub e_phentsize: u16,           
    pub e_phnum: u16,               
    pub e_shentsize: u16,           
    pub e_shnum: u16,               
    pub e_shstrndx: u16,            
}

impl<'a> Parse<ELFHeader> for Data<'a> {
    fn inner_parse(&mut self) -> ELFHeader {
        let magic = self.parse();
        assert_eq!(magic, [0x7F, 0x45, 0x4c, 0x46]);

        let ei_class = self.parse();
        assert_eq!(ei_class, ELFClass::ELFCLASS64, "This code parses only ELF64.");

        // read ei_data and handle the endianess
        let ei_data = self.parse();
        match ei_data {
            ELFData::ELFDATANONE => {},
            ELFData::ELFDATA2LSB => self.set_little_endian(),
            ELFData::ELFDATA2MSB => self.set_big_endian(),
            ELFData::Unknown(x) => panic!("unknown endianess 0x{:02x}", x),
        };

        ELFHeader {
            magic,
            ei_class,         
            ei_data,
            ei_version:    self.parse(),
            ei_osabi:      self.parse(),        
            ei_abiversion: self.parse(),      
            pad:           self.parse(),
            e_type:        self.parse(),            
            e_machine:     self.parse(),      
            e_version:     self.parse(),     
            e_entry:       self.parse(),               
            e_phoff:       self.parse(),             
            e_shoff:       self.parse(),               
            e_flags:       self.parse(),              
            e_ehsize:      self.parse(),              
            e_phentsize:   self.parse(),           
            e_phnum:       self.parse(),               
            e_shentsize:   self.parse(),           
            e_shnum:       self.parse(),               
            e_shstrndx:    self.parse(),            
        }
    }
}

impl_enum!(
    /// ELF header Version, this should always be `EM_CURRENT`
    ELFIntVersion, u8, 
    EM_NONE => 0,
    EM_CURRENT => 1,
);

impl_enum!(
    /// ELF Version, this should always be `EM_CURRENT`
    ELFVersion, u32, 
    EM_NONE => 0,
    EM_CURRENT => 1,
);

impl_enum!(
    /// Endianess of the file
    ELFData, u8, 
    /// Invalid data encoding
    ELFDATANONE => 0,
    /// Little endian
    ELFDATA2LSB => 1,
    /// Big endian
    ELFDATA2MSB => 2,
);

impl_enum!(
    /// If the ELF is 32 or 64 bits
    ELFClass, u8,
    /// Used only for validity checks (this can be ignored)
    ELFCLASSNONE => 0,
    /// A 32 bit ELF
    ELFCLASS32   => 1,
    /// A 64-bit ELF
    ELFCLASS64   => 2,
);

impl_enum!(
    /// Identifies object file type.
    ELFType, u16, 
    ///  An unknown type.
    ET_NONE   => 0,
    ///  A relocatable file.
    ET_REL    => 1,
    /// An executable file.
    ET_EXEC   => 2,
    /// A shared object.
    ET_DYN    => 3,
    /// A core file.
    ET_CORE   => 4,
    /// Operating system specific
    ET_LOOS   => 0xfe00,
    /// Operating system specific
    ET_HIOS   => 0xfeff,
    /// Processor specific
    ET_LOPROC => 0xff00,
    /// Processor specific
    ET_HIPROC => 0xffff,
);

impl_enum!(
    /// This enum identifies the version of the ABI to which 
    /// the object is targeted. This field is used to distinguish among 
    /// incompatible versions of an ABI. The interpretation of this version 
    /// number is dependent on the ABI identified by the EI_OSABI field. If no 
    /// values are specified for the EI_OSABI field for the processor, or no 
    /// version values are specified for the ABI determined by a particular 
    /// value of the EI_OSABI byte, the value 0 is used to indicate unspecified.
    /// 
    /// This is usually set to `ELFOSABI_NONE`.
    ELFOsAbi, u8, 
    ELFOSABI_NONE    =>  0,
    ELFOSABI_HPUX    =>  1,
    ELFOSABI_NETBSD  =>  2,
    ELFOSABI_LINUX   =>  3,
    ELFOSABI_SOLARIS =>  6,
    ELFOSABI_AIX     =>  7,
    ELFOSABI_IRIX    =>  8,
    ELFOSABI_FREEBSD =>  9,
    ELFOSABI_TRU64   => 10,
    ELFOSABI_MODESTO => 11,
    ELFOSABI_OPENBSD => 12,
    ELFOSABI_OPENVMS => 13,
    ELFOSABI_NSK     => 14,
);

impl_enum!(
    /// The target CPU arch of the current ELF.
    ELFMachine, u16, 
    EM_NONE        =>   0,
    EM_M32         =>   1,
    EM_SPARC       =>   2,
    EM_386         =>   3,
    EM_68k         =>   4,
    EM_88k         =>   5,
    EM_860         =>   7,
    EM_MIPS        =>   8,
    EM_S370        =>   9,
    EM_MIPS_RS3_LE =>  10,
    EM_PARSIC      =>  15,
    EM_VPP500      =>  17,
    EM_SPARC32PLUS =>  18,
    EM_960         =>  19,
    EM_PPC         =>  20,
    EM_PPC64       =>  21,
    EM_S390        =>  22,
    EM_V800        =>  36,
    EM_FR20        =>  37,
    EM_RH32        =>  38,
    EM_RCE         =>  39,
    EM_ARM         =>  40,
    EM_ALPHA       =>  41,
    EM_SH          =>  42,
    EM_SPARCV9     =>  43,
    EM_TRICORE     =>  44,
    EM_ARC         =>  45,
    EM_H8_300      =>  46,
    EM_H8_300H     =>  47,
    EM_H8S         =>  48,
    EM_H8_500      =>  49,
    EM_IA_64       =>  50,
    EM_MIPS_X      =>  51,
    EM_COLDFIRE    =>  52,
    EM_68HC12      =>  53,
    EM_MMA         =>  54,
    EM_PCP         =>  55,
    EM_NCPU        =>  56,
    EM_NDR1        =>  57,
    EM_STARCORE    =>  58,
    EM_ME16        =>  59,
    EM_ST100       =>  60,
    EM_TINYJ       =>  61,
    EM_X86_64      =>  62,
    EM_PDSP        =>  63,
    EM_PDP10       =>  64,
    EM_PDP11       =>  65,
    EM_FX66        =>  66,
    EM_ST9PLUS     =>  67,
    EM_ST7         =>  68,
    EM_68HC16      =>  69,
    EM_68HC11      =>  70,
    EM_68HC08      =>  71,
    EM_68HC05      =>  72,
    EM_SVX         =>  73,
    EM_ST19        =>  74,
    EM_VAX         =>  75,
    EM_CRIS        =>  76,
    EM_JAVELIN     =>  77,
    EM_FIREPATH    =>  78,
    EM_ZSP         =>  79,
    EM_MMIX        =>  80,
    EM_HUNAY       =>  81,
    EM_PRISM       =>  82,
    EM_AVR         =>  83,
    EM_FR30        =>  84,
    EM_D10V        =>  85,
    EM_D30V        =>  86,
    EM_V850        =>  87,
    EM_M32R        =>  88,
    EM_MN10300     =>  89,
    EM_MN10200     =>  90,
    EM_PJ          =>  91,
    EM_OPENRISC    =>  92,
    EM_ARC_A5      =>  93,
    EM_XTENSA      =>  94,
    EM_VIDEOCORE   =>  95,
    EM_TMM_GPP     =>  96,
    EM_NS32K       =>  97,
    EM_TPC         =>  98,
    EM_SNP1K       =>  99,
    EM_ST200       => 100,
);
