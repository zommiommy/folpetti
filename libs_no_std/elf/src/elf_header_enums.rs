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
    /// An unknown type.
    ET_NONE   => 0,
    /// A relocatable file.
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
    /// UNIX System V ABI
	ELFOSABI_NONE => 0,
	/// HP-UX operating system
	ELFOSABI_HPUX => 1,
	/// NetBSD
	ELFOSABI_NETBSD => 2,
	/// GNU/Linux
	ELFOSABI_GNU => 3,
	/// Historical alias for ELFOSABI_GNU.
	ELFOSABI_LINUX => 3,
	/// GNU/Hurd
	ELFOSABI_HURD => 4,
	/// Solaris
	ELFOSABI_SOLARIS => 6,
	/// AIX
	ELFOSABI_AIX => 7,
	/// IRIX
	ELFOSABI_IRIX => 8,
	/// FreeBSD
	ELFOSABI_FREEBSD => 9,
	/// TRU64 UNIX
	ELFOSABI_TRU64 => 10,
	/// Novell Modesto
	ELFOSABI_MODESTO => 11,
	/// OpenBSD
	ELFOSABI_OPENBSD => 12,
	/// OpenVMS
	ELFOSABI_OPENVMS => 13,
	/// Hewlett-Packard Non-Stop Kernel
	ELFOSABI_NSK => 14,
	/// AROS
	ELFOSABI_AROS => 15,
	/// FenixOS
	ELFOSABI_FENIXOS => 16,
	/// Nuxi CloudABI
	ELFOSABI_CLOUDABI => 17,
	/// First architecture-specific OS ABI
	ELFOSABI_FIRST_ARCH => 64,
	/// AMD HSA runtime
	ELFOSABI_AMDGPU_HSA => 64,
	/// AMD PAL runtime
	ELFOSABI_AMDGPU_PAL => 65,
	/// AMD GCN GPUs (GFX6+) for MESA runtime
	ELFOSABI_AMDGPU_MESA3D => 66,
	/// ARM
	ELFOSABI_ARM => 97,
	/// Bare-metal TMS320C6000
	ELFOSABI_C6000_ELFABI => 64,
	/// Linux TMS320C6000
	ELFOSABI_C6000_LINUX => 65,
	/// Standalone (embedded) application
	ELFOSABI_STANDALONE => 255,
);

impl_enum!(
    /// The target CPU arch of the current ELF.
    ELFMachine, u16, 	
    /// No machine
	EM_NONE => 0,
	/// AT&T WE 32100
	EM_M32 => 1,
	/// SPARC
	EM_SPARC => 2,
	/// Intel 386
	EM_386 => 3,
	/// Motorola 68000
	EM_68K => 4,
	/// Motorola 88000
	EM_88K => 5,
	/// Intel MCU
	EM_IAMCU => 6,
	/// Intel 80860
	EM_860 => 7,
	/// MIPS R3000
	EM_MIPS => 8,
	/// IBM System/370
	EM_S370 => 9,
	/// MIPS RS3000 Little-endian
	EM_MIPS_RS3_LE => 10,
	/// Hewlett-Packard PA-RISC
	EM_PARISC => 15,
	/// Fujitsu VPP500
	EM_VPP500 => 17,
	/// Enhanced instruction set SPARC
	EM_SPARC32PLUS => 18,
	/// Intel 80960
	EM_960 => 19,
	/// PowerPC
	EM_PPC => 20,
	/// PowerPC64
	EM_PPC64 => 21,
	/// IBM System/390
	EM_S390 => 22,
	/// IBM SPU/SPC
	EM_SPU => 23,
	/// NEC V800
	EM_V800 => 36,
	/// Fujitsu FR20
	EM_FR20 => 37,
	/// TRW RH-32
	EM_RH32 => 38,
	/// Motorola RCE
	EM_RCE => 39,
	/// ARM
	EM_ARM => 40,
	/// DEC Alpha
	EM_ALPHA => 41,
	/// Hitachi SH
	EM_SH => 42,
	/// SPARC V9
	EM_SPARCV9 => 43,
	/// Siemens TriCore
	EM_TRICORE => 44,
	/// Argonaut RISC Core
	EM_ARC => 45,
	/// Hitachi H8/300
	EM_H8_300 => 46,
	/// Hitachi H8/300H
	EM_H8_300H => 47,
	/// Hitachi H8S
	EM_H8S => 48,
	/// Hitachi H8/500
	EM_H8_500 => 49,
	/// Intel IA-64 processor architecture
	EM_IA_64 => 50,
	/// Stanford MIPS-X
	EM_MIPS_X => 51,
	/// Motorola ColdFire
	EM_COLDFIRE => 52,
	/// Motorola M68HC12
	EM_68HC12 => 53,
	/// Fujitsu MMA Multimedia Accelerator
	EM_MMA => 54,
	/// Siemens PCP
	EM_PCP => 55,
	/// Sony nCPU embedded RISC processor
	EM_NCPU => 56,
	/// Denso NDR1 microprocessor
	EM_NDR1 => 57,
	/// Motorola Star*Core processor
	EM_STARCORE => 58,
	/// Toyota ME16 processor
	EM_ME16 => 59,
	/// STMicroelectronics ST100 processor
	EM_ST100 => 60,
	/// Advanced Logic Corp. TinyJ embedded processor family
	EM_TINYJ => 61,
	/// AMD x86-64 architecture
	EM_X86_64 => 62,
	/// Sony DSP Processor
	EM_PDSP => 63,
	/// Digital Equipment Corp. PDP-10
	EM_PDP10 => 64,
	/// Digital Equipment Corp. PDP-11
	EM_PDP11 => 65,
	/// Siemens FX66 microcontroller
	EM_FX66 => 66,
	/// STMicroelectronics ST9+ 8/16 bit microcontroller
	EM_ST9PLUS => 67,
	/// STMicroelectronics ST7 8-bit microcontroller
	EM_ST7 => 68,
	/// Motorola MC68HC16 Microcontroller
	EM_68HC16 => 69,
	/// Motorola MC68HC11 Microcontroller
	EM_68HC11 => 70,
	/// Motorola MC68HC08 Microcontroller
	EM_68HC08 => 71,
	/// Motorola MC68HC05 Microcontroller
	EM_68HC05 => 72,
	/// Silicon Graphics SVx
	EM_SVX => 73,
	/// STMicroelectronics ST19 8-bit microcontroller
	EM_ST19 => 74,
	/// Digital VAX
	EM_VAX => 75,
	/// Axis Communications 32-bit embedded processor
	EM_CRIS => 76,
	/// Infineon Technologies 32-bit embedded processor
	EM_JAVELIN => 77,
	/// Element 14 64-bit DSP Processor
	EM_FIREPATH => 78,
	/// LSI Logic 16-bit DSP Processor
	EM_ZSP => 79,
	/// Donald Knuth's educational 64-bit processor
	EM_MMIX => 80,
	/// Harvard University machine-independent object files
	EM_HUANY => 81,
	/// SiTera Prism
	EM_PRISM => 82,
	/// Atmel AVR 8-bit microcontroller
	EM_AVR => 83,
	/// Fujitsu FR30
	EM_FR30 => 84,
	/// Mitsubishi D10V
	EM_D10V => 85,
	/// Mitsubishi D30V
	EM_D30V => 86,
	/// NEC v850
	EM_V850 => 87,
	/// Mitsubishi M32R
	EM_M32R => 88,
	/// Matsushita MN10300
	EM_MN10300 => 89,
	/// Matsushita MN10200
	EM_MN10200 => 90,
	/// picoJava
	EM_PJ => 91,
	/// OpenRISC 32-bit embedded processor
	EM_OPENRISC => 92,
	/// ARC International ARCompact processor (oldspelling/synonym: EM_ARC_A5)
	EM_ARC_COMPACT => 93,
	/// Tensilica Xtensa Architecture
	EM_XTENSA => 94,
	/// Alphamosaic VideoCore processor
	EM_VIDEOCORE => 95,
	/// Thompson Multimedia General Purpose Processor
	EM_TMM_GPP => 96,
	/// National Semiconductor 32000 series
	EM_NS32K => 97,
	/// Tenor Network TPC processor
	EM_TPC => 98,
	/// Trebia SNP 1000 processor
	EM_SNP1K => 99,
	/// STMicroelectronics (www.st.com) ST200
	EM_ST200 => 100,
	/// Ubicom IP2xxx microcontroller family
	EM_IP2K => 101,
	/// MAX Processor
	EM_MAX => 102,
	/// National Semiconductor CompactRISC microprocessor
	EM_CR => 103,
	/// Fujitsu F2MC16
	EM_F2MC16 => 104,
	/// Texas Instruments embedded microcontroller msp430
	EM_MSP430 => 105,
	/// Analog Devices Blackfin (DSP) processor
	EM_BLACKFIN => 106,
	/// S1C33 Family of Seiko Epson processors
	EM_SE_C33 => 107,
	/// Sharp embedded microprocessor
	EM_SEP => 108,
	/// Arca RISC Microprocessor
	EM_ARCA => 109,
	/// Microprocessor series from PKU-Unity Ltd. and MPRC of Peking University
	EM_UNICORE => 110,
	/// eXcess: 16/32/64-bit configurable embedded CPU
	EM_EXCESS => 111,
	/// Icera Semiconductor Inc. Deep Execution Processor
	EM_DXP => 112,
	/// Altera Nios II soft-core processor
	EM_ALTERA_NIOS2 => 113,
	/// National Semiconductor CompactRISC CRX
	EM_CRX => 114,
	/// Motorola XGATE embedded processor
	EM_XGATE => 115,
	/// Infineon C16x/XC16x processor
	EM_C166 => 116,
	/// Renesas M16C series microprocessors
	EM_M16C => 117,
	/// Microchip Technology dsPIC30F Digital Signal Controller
	EM_DSPIC30F => 118,
	/// Freescale Communication Engine RISC core
	EM_CE => 119,
	/// Renesas M32C series microprocessors
	EM_M32C => 120,
	/// Altium TSK3000 core
	EM_TSK3000 => 131,
	/// Freescale RS08 embedded processor
	EM_RS08 => 132,
	/// Analog Devices SHARC family of 32-bit DSP processors
	EM_SHARC => 133,
	/// Cyan Technology eCOG2 microprocessor
	EM_ECOG2 => 134,
	/// Sunplus S+core7 RISC processor
	EM_SCORE7 => 135,
	/// New Japan Radio (NJR) 24-bit DSP Processor
	EM_DSP24 => 136,
	/// Broadcom VideoCore III processor
	EM_VIDEOCORE3 => 137,
	/// RISC processor for Lattice FPGA architecture
	EM_LATTICEMICO32 => 138,
	/// Seiko Epson C17 family
	EM_SE_C17 => 139,
	/// The Texas Instruments TMS320C6000 DSP family
	EM_TI_C6000 => 140,
	/// The Texas Instruments TMS320C2000 DSP family
	EM_TI_C2000 => 141,
	/// The Texas Instruments TMS320C55x DSP family
	EM_TI_C5500 => 142,
	/// Texas Instruments App. Specific RISC
	EM_TI_ARP32 => 143,
	/// Texas Instruments Prog. Realtime Unit
	EM_TI_PRU => 144,
	/// STMicroelectronics 64bit VLIW Data Signal Processor
	EM_MMDSP_PLUS => 160,
	/// Cypress M8C microprocessor
	EM_CYPRESS_M8C => 161,
	/// Renesas R32C series microprocessors
	EM_R32C => 162,
	/// NXP Semiconductors TriMedia architecture family
	EM_TRIMEDIA => 163,
	/// Qualcomm Hexagon processor
	EM_HEXAGON => 164,
	/// Intel 8051 and variants
	EM_8051 => 165,
	/// STMicroelectronics STxP7x family of configurable and extensible RISC 
    /// processors
	EM_STXP7X => 166,
	/// Andes Technology compact code size embedded RISC processor family
	EM_NDS32 => 167,
	/// Cyan Technology eCOG1X family
	EM_ECOG1 => 168,
	/// Cyan Technology eCOG1X family
	EM_ECOG1X => 168,
	/// Dallas Semiconductor MAXQ30 Core Micro-controllers
	EM_MAXQ30 => 169,
	/// New Japan Radio (NJR) 16-bit DSP Processor
	EM_XIMO16 => 170,
	/// M2000 Reconfigurable RISC Microprocessor
	EM_MANIK => 171,
	/// Cray Inc. NV2 vector architecture
	EM_CRAYNV2 => 172,
	/// Renesas RX family
	EM_RX => 173,
	/// Imagination Technologies META processor architecture
	EM_METAG => 174,
	/// MCST Elbrus general purpose hardware architecture
	EM_MCST_ELBRUS => 175,
	/// Cyan Technology eCOG16 family
	EM_ECOG16 => 176,
	/// National Semiconductor CompactRISC CR16 16-bit microprocessor
	EM_CR16 => 177,
	/// Freescale Extended Time Processing Unit
	EM_ETPU => 178,
	/// Infineon Technologies SLE9X core
	EM_SLE9X => 179,
	/// Intel L10M
	EM_L10M => 180,
	/// Intel K10M
	EM_K10M => 181,
	/// ARM AArch64
	EM_AARCH64 => 183,
	/// Atmel Corporation 32-bit microprocessor family
	EM_AVR32 => 185,
	/// STMicroeletronics STM8 8-bit microcontroller
	EM_STM8 => 186,
	/// Tilera TILE64 multicore architecture family
	EM_TILE64 => 187,
	/// Tilera TILEPro multicore architecture family
	EM_TILEPRO => 188,
	/// Xilinx MicroBlaze 32-bit RISC soft processor core
	EM_MICROBLAZE => 189,
	/// NVIDIA CUDA architecture
	EM_CUDA => 190,
	/// Tilera TILE-Gx multicore architecture family
	EM_TILEGX => 191,
	/// CloudShield architecture family
	EM_CLOUDSHIELD => 192,
	/// KIPO-KAIST Core-A 1st generation processor family
	EM_COREA_1ST => 193,
	/// KIPO-KAIST Core-A 2nd generation processor family
	EM_COREA_2ND => 194,
	/// Synopsys ARCompact V2
	EM_ARC_COMPACT2 => 195,
	/// Open8 8-bit RISC soft processor core
	EM_OPEN8 => 196,
	/// Renesas RL78 family
	EM_RL78 => 197,
	/// Broadcom VideoCore V processor
	EM_VIDEOCORE5 => 198,
	/// Renesas 78KOR family
	EM_78KOR => 199,
	/// Freescale 56800EX Digital Signal Controller (DSC)
	EM_56800EX => 200,
	/// Beyond BA1 CPU architecture
	EM_BA1 => 201,
	/// Beyond BA2 CPU architecture
	EM_BA2 => 202,
	/// XMOS xCORE processor family
	EM_XCORE => 203,
	/// Microchip 8-bit PIC(r) family
	EM_MCHP_PIC => 204,
	/// Reserved by Intel
	EM_INTEL205 => 205,
	/// Reserved by Intel
	EM_INTEL206 => 206,
	/// Reserved by Intel
	EM_INTEL207 => 207,
	/// Reserved by Intel
	EM_INTEL208 => 208,
	/// Reserved by Intel
	EM_INTEL209 => 209,
	/// KM211 KM32 32-bit processor
	EM_KM32 => 210,
	/// KM211 KMX32 32-bit processor
	EM_KMX32 => 211,
	/// KM211 KMX16 16-bit processor
	EM_KMX16 => 212,
	/// KM211 KMX8 8-bit processor
	EM_KMX8 => 213,
	/// KM211 KVARC processor
	EM_KVARC => 214,
	/// Paneve CDP architecture family
	EM_CDP => 215,
	/// Cognitive Smart Memory Processor
	EM_COGE => 216,
	/// iCelero CoolEngine
	EM_COOL => 217,
	/// Nanoradio Optimized RISC
	EM_NORC => 218,
	/// CSR Kalimba architecture family
	EM_CSR_KALIMBA => 219,
	/// Zilog Z80
	EM_Z80 => 220,
	/// Controls and Data Services VISIUMcore
	EM_VISIUM => 221,
	/// FTDI Chip FT32
	EM_FT32 => 222,
	/// Moxie processor
	EM_MOXIE => 223,
	/// AMD GPU architecture
	EM_AMDGPU => 224,
	/// RISC-V
	EM_RISCV => 243,
	/// Lanai 32-bit processor
	EM_LANAI => 244,
	/// Linux kernel bpf virtual machine
	EM_BPF => 247,
	/// NEC SX-Aurora VE
	EM_VE => 251,
	/// C-SKY 32-bit processor
	EM_CSKY => 252,
);
