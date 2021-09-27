
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum ELFType {
    ///  An unknown type.
    ET_NONE,
    ///  A relocatable file.
    ET_REL,
    /// An executable file.
    ET_EXEC,
    /// A shared object.
    ET_DYN,
    /// A core file.
    ET_CORE,
    /// Operating system specific
    ET_LOOS,
    /// Operating system specific
    ET_HIOS,
    /// Processor specific
    ET_LOPROC,
    /// Processor specific
    ET_HIPROC,
    /// Unknown / not recognized elf type
    UNKNOWN(u16),
}

impl From<u16> for ELFType {
    fn from(item: u16) -> Self {
        match item {
            0      => ELFType::ET_NONE,
            1      => ELFType::ET_REL,
            2      => ELFType::ET_EXEC,
            3      => ELFType::ET_DYN,
            4      => ELFType::ET_CORE,
            0xfe00 => ELFType::ET_LOOS,
            0xfeff => ELFType::ET_HIOS,
            0xff00 => ELFType::ET_LOPROC,
            0xffff => ELFType::ET_HIPROC,
            _ => ELFType::UNKNOWN(item),
        }
    }
}

impl From<ELFType> for u16 {
    fn from(item: ELFType) -> Self {
        match item {
            ELFType::ET_NONE   => 0,
            ELFType::ET_REL    => 1,
            ELFType::ET_EXEC   => 2,
            ELFType::ET_DYN    => 3,
            ELFType::ET_CORE   => 4,
            ELFType::ET_LOOS   => 0xfe00,
            ELFType::ET_HIOS   => 0xfeff,
            ELFType::ET_LOPROC => 0xff00,
            ELFType::ET_HIPROC => 0xffff,
            ELFType::UNKNOWN(val) => val,
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum ELFMachine {
    EM_NONE,
    EM_M32,
    EM_SPARC,
    EM_386,
    EM_68k,
    EM_88k,
    EM_860,
    EM_MIPS,
    EM_S370,
    EM_MIPS_RS3_LE,
    EM_PARSIC,
    EM_VPP500,
    EM_SPARC32PLUS,
    EM_960,
    EM_PPC,
    EM_PPC64,
    EM_S390,
    EM_V800,
    EM_FR20,
    EM_RH32,
    EM_RCE,
    EM_ARM,
    EM_ALPHA,
    EM_SH,
    EM_SPARCV9,
    EM_TRICORE,
    EM_ARC,
    EM_H8_300,
    EM_H8_300H,
    EM_H8S,
    EM_H8_500,
    EM_IA_64,
    EM_MIPS_X,
    EM_COLDFIRE,
    EM_68HC12,
    EM_MMA,
    EM_PCP,
    EM_NCPU,
    EM_NDR1,
    EM_STARCORE,
    EM_ME16,
    EM_ST100,
    EM_TINYJ,
    EM_X86_64,
    EM_PDSP,
    EM_PDP10,
    EM_PDP11,
    EM_FX66,
    EM_ST9PLUS,
    EM_ST7,
    EM_68HC16,
    EM_68HC11,
    EM_68HC08,
    EM_68HC05,
    EM_SVX,
    EM_ST19,
    EM_VAX,
    EM_CRIS,
    EM_JAVELIN,
    EM_FIREPATH,
    EM_ZSP,
    EM_MMIX,
    EM_HUNAY,
    EM_PRISM,
    EM_AVR,
    EM_FR30,
    EM_D10V,
    EM_D30V,
    EM_V850,
    EM_M32R,
    EM_MN10300,
    EM_MN10200,
    EM_PJ,
    EM_OPENRISC,
    EM_ARC_A5,
    EM_XTENSA,
    EM_VIDEOCORE,
    EM_TMM_GPP,
    EM_NS32K,
    EM_TPC,
    EM_SNP1K,
    EM_ST200,
    UNKNOWN(u16),
}

impl From<u16> for ELFMachine {
    fn from(item: u16) -> Self {
        match item {
            0   => ELFMachine::EM_NONE,
            1   => ELFMachine::EM_M32,
            2   => ELFMachine::EM_SPARC,
            3   => ELFMachine::EM_386,
            4   => ELFMachine::EM_68k,
            5   => ELFMachine::EM_88k,
            7   => ELFMachine::EM_860,
            8   => ELFMachine::EM_MIPS,
            9   => ELFMachine::EM_S370,
            10  => ELFMachine::EM_MIPS_RS3_LE,
            15  => ELFMachine::EM_PARSIC,
            17  => ELFMachine::EM_VPP500,
            18  => ELFMachine::EM_SPARC32PLUS,
            19  => ELFMachine::EM_960,
            20  => ELFMachine::EM_PPC,
            21  => ELFMachine::EM_PPC64,
            22  => ELFMachine::EM_S390,
            36  => ELFMachine::EM_V800,
            37  => ELFMachine::EM_FR20,
            38  => ELFMachine::EM_RH32,
            39  => ELFMachine::EM_RCE,
            40  => ELFMachine::EM_ARM,
            41  => ELFMachine::EM_ALPHA,
            42  => ELFMachine::EM_SH,
            43  => ELFMachine::EM_SPARCV9,
            44  => ELFMachine::EM_TRICORE,
            45  => ELFMachine::EM_ARC,
            46  => ELFMachine::EM_H8_300,
            47  => ELFMachine::EM_H8_300H,
            48  => ELFMachine::EM_H8S,
            49  => ELFMachine::EM_H8_500,
            50  => ELFMachine::EM_IA_64,
            51  => ELFMachine::EM_MIPS_X,
            52  => ELFMachine::EM_COLDFIRE,
            53  => ELFMachine::EM_68HC12,
            54  => ELFMachine::EM_MMA,
            55  => ELFMachine::EM_PCP,
            56  => ELFMachine::EM_NCPU,
            57  => ELFMachine::EM_NDR1,
            58  => ELFMachine::EM_STARCORE,
            59  => ELFMachine::EM_ME16,
            60  => ELFMachine::EM_ST100,
            61  => ELFMachine::EM_TINYJ,
            62  => ELFMachine::EM_X86_64,
            63  => ELFMachine::EM_PDSP,
            64  => ELFMachine::EM_PDP10,
            65  => ELFMachine::EM_PDP11,
            66  => ELFMachine::EM_FX66,
            67  => ELFMachine::EM_ST9PLUS,
            68  => ELFMachine::EM_ST7,
            69  => ELFMachine::EM_68HC16,
            70  => ELFMachine::EM_68HC11,
            71  => ELFMachine::EM_68HC08,
            72  => ELFMachine::EM_68HC05,
            73  => ELFMachine::EM_SVX,
            74  => ELFMachine::EM_ST19,
            75  => ELFMachine::EM_VAX,
            76  => ELFMachine::EM_CRIS,
            77  => ELFMachine::EM_JAVELIN,
            78  => ELFMachine::EM_FIREPATH,
            79  => ELFMachine::EM_ZSP,
            80  => ELFMachine::EM_MMIX,
            81  => ELFMachine::EM_HUNAY,
            82  => ELFMachine::EM_PRISM,
            83  => ELFMachine::EM_AVR,
            84  => ELFMachine::EM_FR30,
            85  => ELFMachine::EM_D10V,
            86  => ELFMachine::EM_D30V,
            87  => ELFMachine::EM_V850,
            88  => ELFMachine::EM_M32R,
            89  => ELFMachine::EM_MN10300,
            90  => ELFMachine::EM_MN10200,
            91  => ELFMachine::EM_PJ,
            92  => ELFMachine::EM_OPENRISC,
            93  => ELFMachine::EM_ARC_A5,
            94  => ELFMachine::EM_XTENSA,
            95  => ELFMachine::EM_VIDEOCORE,
            96  => ELFMachine::EM_TMM_GPP,
            97  => ELFMachine::EM_NS32K,
            98  => ELFMachine::EM_TPC,
            99  => ELFMachine::EM_SNP1K,
            100 => ELFMachine::EM_ST200,
            _   => ELFMachine::UNKNOWN(item),
        }
    }
}

impl From<ELFMachine> for u16 {
    fn from(item: ELFMachine) -> Self {
        match item {
            ELFMachine::EM_NONE        =>   0,
            ELFMachine::EM_M32         =>   1,
            ELFMachine::EM_SPARC       =>   2,
            ELFMachine::EM_386         =>   3,
            ELFMachine::EM_68k         =>   4,
            ELFMachine::EM_88k         =>   5,
            ELFMachine::EM_860         =>   7,
            ELFMachine::EM_MIPS        =>   8,
            ELFMachine::EM_S370        =>   9,
            ELFMachine::EM_MIPS_RS3_LE =>  10,
            ELFMachine::EM_PARSIC      =>  15,
            ELFMachine::EM_VPP500      =>  17,
            ELFMachine::EM_SPARC32PLUS =>  18,
            ELFMachine::EM_960         =>  19,
            ELFMachine::EM_PPC         =>  20,
            ELFMachine::EM_PPC64       =>  21,
            ELFMachine::EM_S390        =>  22,
            ELFMachine::EM_V800        =>  36,
            ELFMachine::EM_FR20        =>  37,
            ELFMachine::EM_RH32        =>  38,
            ELFMachine::EM_RCE         =>  39,
            ELFMachine::EM_ARM         =>  40,
            ELFMachine::EM_ALPHA       =>  41,
            ELFMachine::EM_SH          =>  42,
            ELFMachine::EM_SPARCV9     =>  43,
            ELFMachine::EM_TRICORE     =>  44,
            ELFMachine::EM_ARC         =>  45,
            ELFMachine::EM_H8_300      =>  46,
            ELFMachine::EM_H8_300H     =>  47,
            ELFMachine::EM_H8S         =>  48,
            ELFMachine::EM_H8_500      =>  49,
            ELFMachine::EM_IA_64       =>  50,
            ELFMachine::EM_MIPS_X      =>  51,
            ELFMachine::EM_COLDFIRE    =>  52,
            ELFMachine::EM_68HC12      =>  53,
            ELFMachine::EM_MMA         =>  54,
            ELFMachine::EM_PCP         =>  55,
            ELFMachine::EM_NCPU        =>  56,
            ELFMachine::EM_NDR1        =>  57,
            ELFMachine::EM_STARCORE    =>  58,
            ELFMachine::EM_ME16        =>  59,
            ELFMachine::EM_ST100       =>  60,
            ELFMachine::EM_TINYJ       =>  61,
            ELFMachine::EM_X86_64      =>  62,
            ELFMachine::EM_PDSP        =>  63,
            ELFMachine::EM_PDP10       =>  64,
            ELFMachine::EM_PDP11       =>  65,
            ELFMachine::EM_FX66        =>  66,
            ELFMachine::EM_ST9PLUS     =>  67,
            ELFMachine::EM_ST7         =>  68,
            ELFMachine::EM_68HC16      =>  69,
            ELFMachine::EM_68HC11      =>  70,
            ELFMachine::EM_68HC08      =>  71,
            ELFMachine::EM_68HC05      =>  72,
            ELFMachine::EM_SVX         =>  73,
            ELFMachine::EM_ST19        =>  74,
            ELFMachine::EM_VAX         =>  75,
            ELFMachine::EM_CRIS        =>  76,
            ELFMachine::EM_JAVELIN     =>  77,
            ELFMachine::EM_FIREPATH    =>  78,
            ELFMachine::EM_ZSP         =>  79,
            ELFMachine::EM_MMIX        =>  80,
            ELFMachine::EM_HUNAY       =>  81,
            ELFMachine::EM_PRISM       =>  82,
            ELFMachine::EM_AVR         =>  83,
            ELFMachine::EM_FR30        =>  84,
            ELFMachine::EM_D10V        =>  85,
            ELFMachine::EM_D30V        =>  86,
            ELFMachine::EM_V850        =>  87,
            ELFMachine::EM_M32R        =>  88,
            ELFMachine::EM_MN10300     =>  89,
            ELFMachine::EM_MN10200     =>  90,
            ELFMachine::EM_PJ          =>  91,
            ELFMachine::EM_OPENRISC    =>  92,
            ELFMachine::EM_ARC_A5      =>  93,
            ELFMachine::EM_XTENSA      =>  94,
            ELFMachine::EM_VIDEOCORE   =>  95,
            ELFMachine::EM_TMM_GPP     =>  96,
            ELFMachine::EM_NS32K       =>  97,
            ELFMachine::EM_TPC         =>  98,
            ELFMachine::EM_SNP1K       =>  99,
            ELFMachine::EM_ST200       => 100,
            ELFMachine::UNKNOWN(val) => val,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ELFClass {
    ELFCLASSNONE,
    ELFCLASS32,
    ELFCLASS64,
    UNKNOWN(u8),
}

impl From<u8> for ELFClass {
    fn from(item: u8) -> Self {
        match item {
            0 => ELFClass::ELFCLASSNONE,
            1 => ELFClass::ELFCLASS32,
            2 => ELFClass::ELFCLASS64,
            _ => ELFClass::UNKNOWN(item),
        }
    }
}

impl From<ELFClass> for u8 {
    fn from(item: ELFClass) -> Self {
        match item {
            ELFClass::ELFCLASSNONE => 0,
            ELFClass::ELFCLASS32   => 1,
            ELFClass::ELFCLASS64   => 2,
            ELFClass::UNKNOWN(val) => val,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ELFData {
    ELFDATANONE,
    ELFDATA2LSB,
    ELFDATA2MSB,
    UNKNOWN(u8),
}

impl From<u8> for ELFData {
    fn from(item: u8) -> Self {
        match item {
            0 => ELFData::ELFDATANONE,
            1 => ELFData::ELFDATA2LSB,
            2 => ELFData::ELFDATA2MSB,
            _ => ELFData::UNKNOWN(item),
        }
    }
}

impl From<ELFData> for u8 {
    fn from(item: ELFData) -> Self {
        match item {
            ELFData::ELFDATANONE => 0,
            ELFData::ELFDATA2LSB => 1,
            ELFData::ELFDATA2MSB => 2,
            ELFData::UNKNOWN(val) => val,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ELFOsAbi {
    ELFOSABI_NONE,
    ELFOSABI_HPUX,
    ELFOSABI_NETBSD,
    ELFOSABI_LINUX,
    ELFOSABI_SOLARIS,
    ELFOSABI_AIX,
    ELFOSABI_IRIX,
    ELFOSABI_FREEBSD,
    ELFOSABI_TRU64,
    ELFOSABI_MODESTO,
    ELFOSABI_OPENBSD,
    ELFOSABI_OPENVMS,
    ELFOSABI_NSK,
    UNKNOWN(u8),
}

impl From<u8> for ELFOsAbi {
    fn from(item: u8) -> Self {
        match item {
             0 => ELFOsAbi::ELFOSABI_NONE,
             1 => ELFOsAbi::ELFOSABI_HPUX,
             2 => ELFOsAbi::ELFOSABI_NETBSD,
             3 => ELFOsAbi::ELFOSABI_LINUX,
             6 => ELFOsAbi::ELFOSABI_SOLARIS,
             7 => ELFOsAbi::ELFOSABI_AIX,
             8 => ELFOsAbi::ELFOSABI_IRIX,
             9 => ELFOsAbi::ELFOSABI_FREEBSD,
            10 => ELFOsAbi::ELFOSABI_TRU64,
            11 => ELFOsAbi::ELFOSABI_MODESTO,
            12 => ELFOsAbi::ELFOSABI_OPENBSD,
            13 => ELFOsAbi::ELFOSABI_OPENVMS,
            14 => ELFOsAbi::ELFOSABI_NSK,
            _ => ELFOsAbi::UNKNOWN(item),
        }
    }
}

impl From<ELFOsAbi> for u8 {
    fn from(item: ELFOsAbi) -> Self {
        match item {
            ELFOsAbi::ELFOSABI_NONE    =>  0,
            ELFOsAbi::ELFOSABI_HPUX    =>  1,
            ELFOsAbi::ELFOSABI_NETBSD  =>  2,
            ELFOsAbi::ELFOSABI_LINUX   =>  3,
            ELFOsAbi::ELFOSABI_SOLARIS =>  6,
            ELFOsAbi::ELFOSABI_AIX     =>  7,
            ELFOsAbi::ELFOSABI_IRIX    =>  8,
            ELFOsAbi::ELFOSABI_FREEBSD =>  9,
            ELFOsAbi::ELFOSABI_TRU64   => 10,
            ELFOsAbi::ELFOSABI_MODESTO => 11,
            ELFOsAbi::ELFOSABI_OPENBSD => 12,
            ELFOsAbi::ELFOSABI_OPENVMS => 13,
            ELFOsAbi::ELFOSABI_NSK     => 14,
            ELFOsAbi::UNKNOWN(val)     => val,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ELFIntVersion {
    EM_NONE,
    EM_CURRENT,
    UNKNOWN(u8),
}

impl From<u8> for ELFIntVersion {
    fn from(item: u8) -> Self {
        match item {
            0 => ELFIntVersion::EM_NONE,
            1 => ELFIntVersion::EM_CURRENT,
            _ => ELFIntVersion::UNKNOWN(item),
        }
    }
}

impl From<ELFIntVersion> for u8 {
    fn from(item: ELFIntVersion) -> Self {
        match item {
            ELFIntVersion::EM_NONE      => 0,
            ELFIntVersion::EM_CURRENT   => 1,
            ELFIntVersion::UNKNOWN(val) => val,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ELFVersion {
    EV_NONE,
    EV_CURRENT,
    UNKNOWN(u32),
}

impl From<u32> for ELFVersion {
    fn from(item: u32) -> Self {
        match item {
            0 => ELFVersion::EV_NONE,
            1 => ELFVersion::EV_CURRENT,
            _ => ELFVersion::UNKNOWN(item),
        }
    }
}

impl From<ELFVersion> for u32 {
    fn from(item: ELFVersion) -> Self {
        match item {
            ELFVersion::EV_NONE      => 0,
            ELFVersion::EV_CURRENT   => 1,
            ELFVersion::UNKNOWN(val) => val,
        }
    }
}
