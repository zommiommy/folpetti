//! Conversion of llvm/usr/include/llvm/BinaryFormat/ElfRelocs/*.def
//! and llvm/include/llvm/ExecutionEngine/JITLink/riscv.h

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum RELOC_RISCV {
    R_RISCV_NONE = 0,
    /// A plain 32-bit pointer value relocation
    ///
    /// Fixup expression:
    ///   Fixup <= Target + Addend : uint32
    ///
    R_RISCV_32 = 1,
    /// A plain 64-bit pointer value relocation
    ///
    /// Fixup expression:
    ///   Fixup <- Target + Addend : uint32
    ///
    R_RISCV_64 = 2,
    R_RISCV_RELATIVE = 3,
    R_RISCV_COPY = 4,
    R_RISCV_JUMP_SLOT = 5,
    R_RISCV_TLS_DTPMOD32 = 6,
    R_RISCV_TLS_DTPMOD64 = 7,
    R_RISCV_TLS_DTPREL32 = 8,
    R_RISCV_TLS_DTPREL64 = 9,
    R_RISCV_TLS_TPREL32 = 10,
    R_RISCV_TLS_TPREL64 = 11,
    /// PC-relative branch pointer value relocation
    ///
    /// Fixup expression:
    ///   Fixup <- (Target - Fixup + Addend)
    ///
    R_RISCV_BRANCH = 16,
    /// High 20 bits of PC-relative jump pointer value relocation
    ///
    /// Fixup expression:
    ///   Fixup <- Target - Fixup + Addend
    ///
    R_RISCV_JAL = 17,
    /// PC relative call
    ///
    /// Fixup expression:
    ///   Fixup <- (Target - Fixup + Addend)
    R_RISCV_CALL = 18,
    /// PC relative call by PLT
    ///
    /// Fixup expression:
    ///   Fixup <- (Target - Fixup + Addend)
    R_RISCV_CALL_PLT = 19,
    /// PC relative GOT offset
    ///
    /// Fixup expression:
    ///   Fixup <- (GOT - Fixup + Addend) >> 12
    R_RISCV_GOT_HI20 = 20,
    R_RISCV_TLS_GOT_HI20 = 21,
    R_RISCV_TLS_GD_HI20 = 22,
    /// High 20 bits of PC relative relocation
    ///
    /// Fixup expression:
    ///   Fixup <- (Target - Fixup + Addend + 0x800) >> 12
    R_RISCV_PCREL_HI20 = 23,
    /// Low 12 bits of PC relative relocation, used by I type instruction format
    ///
    /// Fixup expression:
    ///   Fixup <- (Target - Fixup + Addend) & 0xFFF
    R_RISCV_PCREL_LO12_I = 24,
    /// Low 12 bits of PC relative relocation, used by S type instruction format
    ///
    /// Fixup expression:
    ///   Fixup <- (Target - Fixup + Addend) & 0xFFF
    R_RISCV_PCREL_LO12_S = 25,
    /// High 20 bits of 32-bit pointer value relocation
    ///
    /// Fixup expression
    ///   Fixup <- (Target + Addend + 0x800) >> 12
    R_RISCV_HI20 = 26,
    /// Low 12 bits of 32-bit pointer value relocation
    ///
    /// Fixup expression
    ///   Fixup <- (Target + Addend) & 0xFFF
    R_RISCV_LO12_I = 27,
    R_RISCV_LO12_S = 28,
    R_RISCV_TPREL_HI20 = 29,
    R_RISCV_TPREL_LO12_I = 30,
    R_RISCV_TPREL_LO12_S = 31,
    R_RISCV_TPREL_ADD = 32,
    /// 8 bits label addition
    ///
    /// Fixup expression
    ///   Fixup <- (Target - *{1}Fixup + Addend)
    R_RISCV_ADD8 = 33,
    /// 16 bits label addition
    ///
    /// Fixup expression
    ///   Fixup <- (Target - *{2}Fixup + Addend)
    R_RISCV_ADD16 = 34,
    /// 32 bits label addition
    ///
    /// Fixup expression:
    ///   Fixup <- (Target - *{4}Fixup + Addend)
    R_RISCV_ADD32 = 35,
    /// 64 bits label addition
    ///
    /// Fixup expression:
    ///   Fixup <- (Target - *{8}Fixup + Addend)
    R_RISCV_ADD64 = 36,
    /// 8 bits label subtraction
    ///
    /// Fixup expression
    ///   Fixup <- (Target - *{1}Fixup - Addend)
    R_RISCV_SUB8 = 37,
    /// 16 bits label subtraction
    ///
    /// Fixup expression
    ///   Fixup <- (Target - *{2}Fixup - Addend)
    R_RISCV_SUB16 = 38,
    /// 32 bits label subtraction
    ///
    /// Fixup expression
    ///   Fixup <- (Target - *{4}Fixup - Addend)
    R_RISCV_SUB32 = 39,  
    /// 64 bits label subtraction
    ///
    /// Fixup expression
    ///   Fixup <- (Target - *{8}Fixup - Addend)
    R_RISCV_SUB64 = 40,
    R_RISCV_GNU_VTINHERIT = 41,
    R_RISCV_GNU_VTENTRY = 42,
    R_RISCV_ALIGN = 43,
    R_RISCV_RVC_BRANCH = 44,
    R_RISCV_RVC_JUMP = 45,
    R_RISCV_RVC_LUI = 46,
    R_RISCV_RELAX = 51,
    /// 6 bits label subtraction
    ///
    /// Fixup expression
    ///   Fixup <- (Target - *{1}Fixup - Addend)
    R_RISCV_SUB6 = 52,
    /// Local label assignment
    ///
    /// Fixup expression:
    ///   Fixup <- (Target + Addend)
    R_RISCV_SET6 = 53,
    /// Local label assignment
    ///
    /// Fixup expression:
    ///   Fixup <- (Target + Addend)
    R_RISCV_SET8 = 54,
    /// Local label assignment
    ///
    /// Fixup expression:
    ///   Fixup <- (Target + Addend)
    R_RISCV_SET16 = 55,
    /// Local label assignment
    ///
    /// Fixup expression:
    ///   Fixup <- (Target + Addend)
    R_RISCV_SET32 = 56,
    /// 32 bits PC relative relocation
    ///
    /// Fixup expression:
    ///   Fixup <- (Target - Fixup + Addend)
    R_RISCV_32_PCREL = 57,
    R_RISCV_IRELATIVE = 58,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum RELOC_X86_64 {
    R_X86_64_NONE = 0,
    R_X86_64_64 = 1,
    R_X86_64_PC32 = 2,
    R_X86_64_GOT32 = 3,
    R_X86_64_PLT32 = 4,
    R_X86_64_COPY = 5,
    R_X86_64_GLOB_DAT = 6,
    R_X86_64_JUMP_SLOT = 7,
    R_X86_64_RELATIVE = 8,
    R_X86_64_GOTPCREL = 9,
    R_X86_64_32 = 10,
    R_X86_64_32S = 11,
    R_X86_64_16 = 12,
    R_X86_64_PC16 = 13,
    R_X86_64_8 = 14,
    R_X86_64_PC8 = 15,
    R_X86_64_DTPMOD64 = 16,
    R_X86_64_DTPOFF64 = 17,
    R_X86_64_TPOFF64 = 18,
    R_X86_64_TLSGD = 19,
    R_X86_64_TLSLD = 20,
    R_X86_64_DTPOFF32 = 21,
    R_X86_64_GOTTPOFF = 22,
    R_X86_64_TPOFF32 = 23,
    R_X86_64_PC64 = 24,
    R_X86_64_GOTOFF64 = 25,
    R_X86_64_GOTPC32 = 26,
    R_X86_64_GOT64 = 27,
    R_X86_64_GOTPCREL64 = 28,
    R_X86_64_GOTPC64 = 29,
    R_X86_64_GOTPLT64 = 30,
    R_X86_64_PLTOFF64 = 31,
    R_X86_64_SIZE32 = 32,
    R_X86_64_SIZE64 = 33,
    R_X86_64_GOTPC32_TLSDESC = 34,
    R_X86_64_TLSDESC_CALL = 35,
    R_X86_64_TLSDESC = 36,
    R_X86_64_IRELATIVE = 37,
    R_X86_64_GOTPCRELX = 41,
    R_X86_64_REX_GOTPCRELX = 42,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum RELOC_ARM {
    R_ARM_NONE = 0x00,
    R_ARM_PC24 = 0x01,
    R_ARM_ABS32 = 0x02,
    R_ARM_REL32 = 0x03,
    R_ARM_LDR_PC_G0 = 0x04,
    R_ARM_ABS16 = 0x05,
    R_ARM_ABS12 = 0x06,
    R_ARM_THM_ABS5 = 0x07,
    R_ARM_ABS8 = 0x08,
    R_ARM_SBREL32 = 0x09,
    R_ARM_THM_CALL = 0x0a,
    R_ARM_THM_PC8 = 0x0b,
    R_ARM_BREL_ADJ = 0x0c,
    R_ARM_TLS_DESC = 0x0d,
    R_ARM_THM_SWI8 = 0x0e,
    R_ARM_XPC25 = 0x0f,
    R_ARM_THM_XPC22 = 0x10,
    R_ARM_TLS_DTPMOD32 = 0x11,
    R_ARM_TLS_DTPOFF32 = 0x12,
    R_ARM_TLS_TPOFF32 = 0x13,
    R_ARM_COPY = 0x14,
    R_ARM_GLOB_DAT = 0x15,
    R_ARM_JUMP_SLOT = 0x16,
    R_ARM_RELATIVE = 0x17,
    R_ARM_GOTOFF32 = 0x18,
    R_ARM_BASE_PREL = 0x19,
    R_ARM_GOT_BREL = 0x1a,
    R_ARM_PLT32 = 0x1b,
    R_ARM_CALL = 0x1c,
    R_ARM_JUMP24 = 0x1d,
    R_ARM_THM_JUMP24 = 0x1e,
    R_ARM_BASE_ABS = 0x1f,
    R_ARM_ALU_PCREL_7_0 = 0x20,
    R_ARM_ALU_PCREL_15_8 = 0x21,
    R_ARM_ALU_PCREL_23_15 = 0x22,
    R_ARM_LDR_SBREL_11_0_NC = 0x23,
    R_ARM_ALU_SBREL_19_12_NC = 0x24,
    R_ARM_ALU_SBREL_27_20_CK = 0x25,
    R_ARM_TARGET1 = 0x26,
    R_ARM_SBREL31 = 0x27,
    R_ARM_V4BX = 0x28,
    R_ARM_TARGET2 = 0x29,
    R_ARM_PREL31 = 0x2a,
    R_ARM_MOVW_ABS_NC = 0x2b,
    R_ARM_MOVT_ABS = 0x2c,
    R_ARM_MOVW_PREL_NC = 0x2d,
    R_ARM_MOVT_PREL = 0x2e,
    R_ARM_THM_MOVW_ABS_NC = 0x2f,
    R_ARM_THM_MOVT_ABS = 0x30,
    R_ARM_THM_MOVW_PREL_NC = 0x31,
    R_ARM_THM_MOVT_PREL = 0x32,
    R_ARM_THM_JUMP19 = 0x33,
    R_ARM_THM_JUMP6 = 0x34,
    R_ARM_THM_ALU_PREL_11_0 = 0x35,
    R_ARM_THM_PC12 = 0x36,
    R_ARM_ABS32_NOI = 0x37,
    R_ARM_REL32_NOI = 0x38,
    R_ARM_ALU_PC_G0_NC = 0x39,
    R_ARM_ALU_PC_G0 = 0x3a,
    R_ARM_ALU_PC_G1_NC = 0x3b,
    R_ARM_ALU_PC_G1 = 0x3c,
    R_ARM_ALU_PC_G2 = 0x3d,
    R_ARM_LDR_PC_G1 = 0x3e,
    R_ARM_LDR_PC_G2 = 0x3f,
    R_ARM_LDRS_PC_G0 = 0x40,
    R_ARM_LDRS_PC_G1 = 0x41,
    R_ARM_LDRS_PC_G2 = 0x42,
    R_ARM_LDC_PC_G0 = 0x43,
    R_ARM_LDC_PC_G1 = 0x44,
    R_ARM_LDC_PC_G2 = 0x45,
    R_ARM_ALU_SB_G0_NC = 0x46,
    R_ARM_ALU_SB_G0 = 0x47,
    R_ARM_ALU_SB_G1_NC = 0x48,
    R_ARM_ALU_SB_G1 = 0x49,
    R_ARM_ALU_SB_G2 = 0x4a,
    R_ARM_LDR_SB_G0 = 0x4b,
    R_ARM_LDR_SB_G1 = 0x4c,
    R_ARM_LDR_SB_G2 = 0x4d,
    R_ARM_LDRS_SB_G0 = 0x4e,
    R_ARM_LDRS_SB_G1 = 0x4f,
    R_ARM_LDRS_SB_G2 = 0x50,
    R_ARM_LDC_SB_G0 = 0x51,
    R_ARM_LDC_SB_G1 = 0x52,
    R_ARM_LDC_SB_G2 = 0x53,
    R_ARM_MOVW_BREL_NC = 0x54,
    R_ARM_MOVT_BREL = 0x55,
    R_ARM_MOVW_BREL = 0x56,
    R_ARM_THM_MOVW_BREL_NC = 0x57,
    R_ARM_THM_MOVT_BREL = 0x58,
    R_ARM_THM_MOVW_BREL = 0x59,
    R_ARM_TLS_GOTDESC = 0x5a,
    R_ARM_TLS_CALL = 0x5b,
    R_ARM_TLS_DESCSEQ = 0x5c,
    R_ARM_THM_TLS_CALL = 0x5d,
    R_ARM_PLT32_ABS = 0x5e,
    R_ARM_GOT_ABS = 0x5f,
    R_ARM_GOT_PREL = 0x60,
    R_ARM_GOT_BREL12 = 0x61,
    R_ARM_GOTOFF12 = 0x62,
    R_ARM_GOTRELAX = 0x63,
    R_ARM_GNU_VTENTRY = 0x64,
    R_ARM_GNU_VTINHERIT = 0x65,
    R_ARM_THM_JUMP11 = 0x66,
    R_ARM_THM_JUMP8 = 0x67,
    R_ARM_TLS_GD32 = 0x68,
    R_ARM_TLS_LDM32 = 0x69,
    R_ARM_TLS_LDO32 = 0x6a,
    R_ARM_TLS_IE32 = 0x6b,
    R_ARM_TLS_LE32 = 0x6c,
    R_ARM_TLS_LDO12 = 0x6d,
    R_ARM_TLS_LE12 = 0x6e,
    R_ARM_TLS_IE12GP = 0x6f,
    R_ARM_PRIVATE_0 = 0x70,
    R_ARM_PRIVATE_1 = 0x71,
    R_ARM_PRIVATE_2 = 0x72,
    R_ARM_PRIVATE_3 = 0x73,
    R_ARM_PRIVATE_4 = 0x74,
    R_ARM_PRIVATE_5 = 0x75,
    R_ARM_PRIVATE_6 = 0x76,
    R_ARM_PRIVATE_7 = 0x77,
    R_ARM_PRIVATE_8 = 0x78,
    R_ARM_PRIVATE_9 = 0x79,
    R_ARM_PRIVATE_10 = 0x7a,
    R_ARM_PRIVATE_11 = 0x7b,
    R_ARM_PRIVATE_12 = 0x7c,
    R_ARM_PRIVATE_13 = 0x7d,
    R_ARM_PRIVATE_14 = 0x7e,
    R_ARM_PRIVATE_15 = 0x7f,
    R_ARM_ME_TOO = 0x80,
    R_ARM_THM_TLS_DESCSEQ16 = 0x81,
    R_ARM_THM_TLS_DESCSEQ32 = 0x82,
    R_ARM_THM_BF16 = 0x88,
    R_ARM_THM_BF12 = 0x89,
    R_ARM_THM_BF18 = 0x8a,
    R_ARM_IRELATIVE = 0xa0,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum RELOC_AARCH64 {
    R_AARCH64_NONE = 0,
    R_AARCH64_ABS64 = 0x101,
    R_AARCH64_ABS32 = 0x102,
    R_AARCH64_ABS16 = 0x103,
    R_AARCH64_PREL64 = 0x104,
    R_AARCH64_PREL32 = 0x105,
    R_AARCH64_PREL16 = 0x106,
    R_AARCH64_MOVW_UABS_G0 = 0x107,
    R_AARCH64_MOVW_UABS_G0_NC = 0x108,
    R_AARCH64_MOVW_UABS_G1 = 0x109,
    R_AARCH64_MOVW_UABS_G1_NC = 0x10a,
    R_AARCH64_MOVW_UABS_G2 = 0x10b,
    R_AARCH64_MOVW_UABS_G2_NC = 0x10c,
    R_AARCH64_MOVW_UABS_G3 = 0x10d,
    R_AARCH64_MOVW_SABS_G0 = 0x10e,
    R_AARCH64_MOVW_SABS_G1 = 0x10f,
    R_AARCH64_MOVW_SABS_G2 = 0x110,
    R_AARCH64_LD_PREL_LO19 = 0x111,
    R_AARCH64_ADR_PREL_LO21 = 0x112,
    R_AARCH64_ADR_PREL_PG_HI21 = 0x113,
    R_AARCH64_ADR_PREL_PG_HI21_NC = 0x114,
    R_AARCH64_ADD_ABS_LO12_NC = 0x115,
    R_AARCH64_LDST8_ABS_LO12_NC = 0x116,
    R_AARCH64_TSTBR14 = 0x117,
    R_AARCH64_CONDBR19 = 0x118,
    R_AARCH64_JUMP26 = 0x11a,
    R_AARCH64_CALL26 = 0x11b,
    R_AARCH64_LDST16_ABS_LO12_NC = 0x11c,
    R_AARCH64_LDST32_ABS_LO12_NC = 0x11d,
    R_AARCH64_LDST64_ABS_LO12_NC = 0x11e,
    R_AARCH64_MOVW_PREL_G0 = 0x11f,
    R_AARCH64_MOVW_PREL_G0_NC = 0x120,
    R_AARCH64_MOVW_PREL_G1 = 0x121,
    R_AARCH64_MOVW_PREL_G1_NC = 0x122,
    R_AARCH64_MOVW_PREL_G2 = 0x123,
    R_AARCH64_MOVW_PREL_G2_NC = 0x124,
    R_AARCH64_MOVW_PREL_G3 = 0x125,
    R_AARCH64_LDST128_ABS_LO12_NC = 0x12b,
    R_AARCH64_MOVW_GOTOFF_G0 = 0x12c,
    R_AARCH64_MOVW_GOTOFF_G0_NC = 0x12d,
    R_AARCH64_MOVW_GOTOFF_G1 = 0x12e,
    R_AARCH64_MOVW_GOTOFF_G1_NC = 0x12f,
    R_AARCH64_MOVW_GOTOFF_G2 = 0x130,
    R_AARCH64_MOVW_GOTOFF_G2_NC = 0x131,
    R_AARCH64_MOVW_GOTOFF_G3 = 0x132,
    R_AARCH64_GOTREL64 = 0x133,
    R_AARCH64_GOTREL32 = 0x134,
    R_AARCH64_GOT_LD_PREL19 = 0x135,
    R_AARCH64_LD64_GOTOFF_LO15 = 0x136,
    R_AARCH64_ADR_GOT_PAGE = 0x137,
    R_AARCH64_LD64_GOT_LO12_NC = 0x138,
    R_AARCH64_LD64_GOTPAGE_LO15 = 0x139,
    R_AARCH64_PLT32 = 0x13a,
    R_AARCH64_TLSGD_ADR_PREL21 = 0x200,
    R_AARCH64_TLSGD_ADR_PAGE21 = 0x201,
    R_AARCH64_TLSGD_ADD_LO12_NC = 0x202,
    R_AARCH64_TLSGD_MOVW_G1 = 0x203,
    R_AARCH64_TLSGD_MOVW_G0_NC = 0x204,
    R_AARCH64_TLSLD_ADR_PREL21 = 0x205,
    R_AARCH64_TLSLD_ADR_PAGE21 = 0x206,
    R_AARCH64_TLSLD_ADD_LO12_NC = 0x207,
    R_AARCH64_TLSLD_MOVW_G1 = 0x208,
    R_AARCH64_TLSLD_MOVW_G0_NC = 0x209,
    R_AARCH64_TLSLD_LD_PREL19 = 0x20a,
    R_AARCH64_TLSLD_MOVW_DTPREL_G2 = 0x20b,
    R_AARCH64_TLSLD_MOVW_DTPREL_G1 = 0x20c,
    R_AARCH64_TLSLD_MOVW_DTPREL_G1_NC = 0x20d,
    R_AARCH64_TLSLD_MOVW_DTPREL_G0 = 0x20e,
    R_AARCH64_TLSLD_MOVW_DTPREL_G0_NC = 0x20f,
    R_AARCH64_TLSLD_ADD_DTPREL_HI12 = 0x210,
    R_AARCH64_TLSLD_ADD_DTPREL_LO12 = 0x211,
    R_AARCH64_TLSLD_ADD_DTPREL_LO12_NC = 0x212,
    R_AARCH64_TLSLD_LDST8_DTPREL_LO12 = 0x213,
    R_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC = 0x214,
    R_AARCH64_TLSLD_LDST16_DTPREL_LO12 = 0x215,
    R_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC = 0x216,
    R_AARCH64_TLSLD_LDST32_DTPREL_LO12 = 0x217,
    R_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC = 0x218,
    R_AARCH64_TLSLD_LDST64_DTPREL_LO12 = 0x219,
    R_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC = 0x21a,
    R_AARCH64_TLSIE_MOVW_GOTTPREL_G1 = 0x21b,
    R_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC = 0x21c,
    R_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21 = 0x21d,
    R_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC = 0x21e,
    R_AARCH64_TLSIE_LD_GOTTPREL_PREL19 = 0x21f,
    R_AARCH64_TLSLE_MOVW_TPREL_G2 = 0x220,
    R_AARCH64_TLSLE_MOVW_TPREL_G1 = 0x221,
    R_AARCH64_TLSLE_MOVW_TPREL_G1_NC = 0x222,
    R_AARCH64_TLSLE_MOVW_TPREL_G0 = 0x223,
    R_AARCH64_TLSLE_MOVW_TPREL_G0_NC = 0x224,
    R_AARCH64_TLSLE_ADD_TPREL_HI12 = 0x225,
    R_AARCH64_TLSLE_ADD_TPREL_LO12 = 0x226,
    R_AARCH64_TLSLE_ADD_TPREL_LO12_NC = 0x227,
    R_AARCH64_TLSLE_LDST8_TPREL_LO12 = 0x228,
    R_AARCH64_TLSLE_LDST8_TPREL_LO12_NC = 0x229,
    R_AARCH64_TLSLE_LDST16_TPREL_LO12 = 0x22a,
    R_AARCH64_TLSLE_LDST16_TPREL_LO12_NC = 0x22b,
    R_AARCH64_TLSLE_LDST32_TPREL_LO12 = 0x22c,
    R_AARCH64_TLSLE_LDST32_TPREL_LO12_NC = 0x22d,
    R_AARCH64_TLSLE_LDST64_TPREL_LO12 = 0x22e,
    R_AARCH64_TLSLE_LDST64_TPREL_LO12_NC = 0x22f,
    R_AARCH64_TLSDESC_LD_PREL19 = 0x230,
    R_AARCH64_TLSDESC_ADR_PREL21 = 0x231,
    R_AARCH64_TLSDESC_ADR_PAGE21 = 0x232,
    R_AARCH64_TLSDESC_LD64_LO12 = 0x233,
    R_AARCH64_TLSDESC_ADD_LO12 = 0x234,
    R_AARCH64_TLSDESC_OFF_G1 = 0x235,
    R_AARCH64_TLSDESC_OFF_G0_NC = 0x236,
    R_AARCH64_TLSDESC_LDR = 0x237,
    R_AARCH64_TLSDESC_ADD = 0x238,
    R_AARCH64_TLSDESC_CALL = 0x239,
    R_AARCH64_TLSLE_LDST128_TPREL_LO12 = 0x23a,
    R_AARCH64_TLSLE_LDST128_TPREL_LO12_NC = 0x23b,
    R_AARCH64_TLSLD_LDST128_DTPREL_LO12 = 0x23c,
    R_AARCH64_TLSLD_LDST128_DTPREL_LO12_NC = 0x23d,
    
    // Dynamic relocations start
    R_AARCH64_COPY = 0x400,
    R_AARCH64_GLOB_DAT = 0x401,
    R_AARCH64_JUMP_SLOT = 0x402,
    R_AARCH64_RELATIVE = 0x403,
    
    // 0x404 and 0x405 are now R_AARCH64_TLS_IMPDEF1 and R_AARCH64_TLS_IMPDEF2
    // We follow GNU and define TLS_IMPDEF1 as TLS_DTPMOD64 and TLS_IMPDEF2 as
    // TLS_DTPREL64
    R_AARCH64_TLS_DTPMOD64 = 0x404,
    R_AARCH64_TLS_DTPREL64 = 0x405,
    R_AARCH64_TLS_TPREL64 = 0x406,
    R_AARCH64_TLSDESC = 0x407,
    R_AARCH64_IRELATIVE = 0x408,
    
    // R_AARCH64_P32_NONE = 0,
    R_AARCH64_P32_ABS32 = 0x001,
    R_AARCH64_P32_ABS16 = 0x002,
    R_AARCH64_P32_PREL32 = 0x003,
    R_AARCH64_P32_PREL16 = 0x004,
    R_AARCH64_P32_MOVW_UABS_G0 = 0x005,
    R_AARCH64_P32_MOVW_UABS_G0_NC = 0x006,
    R_AARCH64_P32_MOVW_UABS_G1 = 0x007,
    R_AARCH64_P32_MOVW_SABS_G0 = 0x008,
    R_AARCH64_P32_LD_PREL_LO19 = 0x009,
    R_AARCH64_P32_ADR_PREL_LO21 = 0x00a,
    R_AARCH64_P32_ADR_PREL_PG_HI21 = 0x00b,
    R_AARCH64_P32_ADD_ABS_LO12_NC = 0x00c,
    R_AARCH64_P32_LDST8_ABS_LO12_NC = 0x00d,
    R_AARCH64_P32_LDST16_ABS_LO12_NC = 0x00e,
    R_AARCH64_P32_LDST32_ABS_LO12_NC = 0x00f,
    R_AARCH64_P32_LDST64_ABS_LO12_NC = 0x010,
    R_AARCH64_P32_LDST128_ABS_LO12_NC = 0x011,
    R_AARCH64_P32_TSTBR14 = 0x012,
    R_AARCH64_P32_CONDBR19 = 0x013,
    R_AARCH64_P32_JUMP26 = 0x014,
    R_AARCH64_P32_CALL26 = 0x015,
    R_AARCH64_P32_MOVW_PREL_G0 = 0x016,
    R_AARCH64_P32_MOVW_PREL_G0_NC = 0x017,
    R_AARCH64_P32_MOVW_PREL_G1 = 0x018,
    R_AARCH64_P32_GOT_LD_PREL19 = 0x019,
    R_AARCH64_P32_ADR_GOT_PAGE = 0x01a,
    R_AARCH64_P32_LD32_GOT_LO12_NC = 0x01b,
    R_AARCH64_P32_LD32_GOTPAGE_LO14 = 0x01c,
    R_AARCH64_P32_PLT32 = 0x01d,
    R_AARCH64_P32_TLSGD_ADR_PREL21 = 0x050,
    R_AARCH64_P32_TLSGD_ADR_PAGE21 = 0x051,
    R_AARCH64_P32_TLSGD_ADD_LO12_NC = 0x052,
    R_AARCH64_P32_TLSLD_ADR_PREL21 = 0x053,
    R_AARCH64_P32_TLSLD_ADR_PAGE21 = 0x054,
    R_AARCH64_P32_TLSLD_ADD_LO12_NC = 0x055,
    R_AARCH64_P32_TLSLD_LD_PREL19 = 0x056,
    R_AARCH64_P32_TLSLD_MOVW_DTPREL_G1 = 0x057,
    R_AARCH64_P32_TLSLD_MOVW_DTPREL_G0 = 0x058,
    R_AARCH64_P32_TLSLD_MOVW_DTPREL_G0_NC = 0x059,
    R_AARCH64_P32_TLSLD_ADD_DTPREL_HI12 = 0x05a,
    R_AARCH64_P32_TLSLD_ADD_DTPREL_LO12 = 0x05b,
    R_AARCH64_P32_TLSLD_ADD_DTPREL_LO12_NC = 0x05c,
    R_AARCH64_P32_TLSLD_LDST8_DTPREL_LO12 = 0x05d,
    R_AARCH64_P32_TLSLD_LDST8_DTPREL_LO12_NC = 0x05e,
    R_AARCH64_P32_TLSLD_LDST16_DTPREL_LO12 = 0x05f,
    R_AARCH64_P32_TLSLD_LDST16_DTPREL_LO12_NC = 0x060,
    R_AARCH64_P32_TLSLD_LDST32_DTPREL_LO12 = 0x061,
    R_AARCH64_P32_TLSLD_LDST32_DTPREL_LO12_NC = 0x062,
    R_AARCH64_P32_TLSLD_LDST64_DTPREL_LO12 = 0x063,
    R_AARCH64_P32_TLSLD_LDST64_DTPREL_LO12_NC = 0x064,
    R_AARCH64_P32_TLSLD_LDST128_DTPREL_LO12 = 0x065,
    R_AARCH64_P32_TLSLD_LDST128_DTPREL_LO12_NC = 0x066,
    R_AARCH64_P32_TLSIE_ADR_GOTTPREL_PAGE21 = 0x067,
    R_AARCH64_P32_TLSIE_LD32_GOTTPREL_LO12_NC = 0x068,
    R_AARCH64_P32_TLSIE_LD_GOTTPREL_PREL19 = 0x069,
    R_AARCH64_P32_TLSLE_MOVW_TPREL_G1 = 0x06a,
    R_AARCH64_P32_TLSLE_MOVW_TPREL_G0 = 0x06b,
    R_AARCH64_P32_TLSLE_MOVW_TPREL_G0_NC = 0x06c,
    R_AARCH64_P32_TLSLE_ADD_TPREL_HI12 = 0x06d,
    R_AARCH64_P32_TLSLE_ADD_TPREL_LO12 = 0x06e,
    R_AARCH64_P32_TLSLE_ADD_TPREL_LO12_NC = 0x06f,
    R_AARCH64_P32_TLSLE_LDST8_TPREL_LO12 = 0x070,
    R_AARCH64_P32_TLSLE_LDST8_TPREL_LO12_NC = 0x071,
    R_AARCH64_P32_TLSLE_LDST16_TPREL_LO12 = 0x072,
    R_AARCH64_P32_TLSLE_LDST16_TPREL_LO12_NC = 0x073,
    R_AARCH64_P32_TLSLE_LDST32_TPREL_LO12 = 0x074,
    R_AARCH64_P32_TLSLE_LDST32_TPREL_LO12_NC = 0x075,
    R_AARCH64_P32_TLSLE_LDST64_TPREL_LO12 = 0x076,
    R_AARCH64_P32_TLSLE_LDST64_TPREL_LO12_NC = 0x077,
    R_AARCH64_P32_TLSLE_LDST128_TPREL_LO12 = 0x078,
    R_AARCH64_P32_TLSLE_LDST128_TPREL_LO12_NC = 0x079,
    R_AARCH64_P32_TLSDESC_LD_PREL19 = 0x07a,
    R_AARCH64_P32_TLSDESC_ADR_PREL21 = 0x07b,
    R_AARCH64_P32_TLSDESC_ADR_PAGE21 = 0x07c,
    R_AARCH64_P32_TLSDESC_LD32_LO12 = 0x07d,
    R_AARCH64_P32_TLSDESC_ADD_LO12 = 0x07e,
    R_AARCH64_P32_TLSDESC_CALL = 0x07f,
    
    // Dynamic relocations start
    R_AARCH64_P32_COPY = 0x0b4,
    R_AARCH64_P32_GLOB_DAT = 0x0b5,
    R_AARCH64_P32_JUMP_SLOT = 0x0b6,
    R_AARCH64_P32_RELATIVE = 0x0b7,
    R_AARCH64_P32_TLS_DTPREL = 0x0b8,
    R_AARCH64_P32_TLS_DTPMOD = 0x0b9,
    R_AARCH64_P32_TLS_TPREL = 0x0ba,
    R_AARCH64_P32_TLSDESC = 0x0bb,
    R_AARCH64_P32_IRELATIVE = 0x0bc,
}

