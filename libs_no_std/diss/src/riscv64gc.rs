//! https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf
//! https://msyksphinz-self.github.io/riscv-isadoc/html/index.html
//! https://github.com/gamozolabs/fuzz_with_emus/blob/master/src/emulator.rs
//! 
//! This should handle anything that targets `riscv64gc-unknown-linux-gnu`
use crate::utils::*;


/// 64-bit RISC-V registers
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Register {
    Zero = 0,
    /// Return Address, Caller Saved
    Ra,
    /// Stack Pointer, Callee Saved
    Sp,
    /// Global Pointer
    Gp,
    /// Thread Pointer
    Tp,
    /// Temp / alternate link register, Caller Saved
    T0,
    /// Temp, Caller Saved
    T1,
    /// Temp, Caller Saved
    T2,
    /// Saved registers / Frame Pointer, Callee Saved
    S0,
    /// Saved registers, Callee Saved
    S1,
    /// Function Arguments / return values, Caller Saved
    A0,
    /// Function Arguments / return values, Caller Saved
    A1,
    /// Function Arguments, Caller Saved
    A2,
    /// Function Arguments, Caller Saved
    A3,
    /// Function Arguments, Caller Saved
    A4,
    /// Function Arguments, Caller Saved
    A5,
    /// Function Arguments, Caller Saved
    A6,
    /// Function Arguments, Caller Saved
    A7,
    /// Saved register, Callee Saved
    S2,
    /// Saved registers, Callee Saved
    S3,
    /// Saved registers, Callee Saved
    S4,
    /// Saved registers, Callee Saved
    S5,
    /// Saved registers, Callee Saved
    S6,
    /// Saved registers, Callee Saved
    S7,
    /// Saved registers, Callee Saved
    S8,
    /// Saved registers, Callee Saved
    S9,
    /// Saved registers, Callee Saved
    S10,
    /// Saved registers, Callee Saved
    S11,
    /// Temp, Caller Saved
    T3,
    /// Temp, Caller Saved
    T4,
    /// Temp, Caller Saved
    T5,
    /// Temp, Caller Saved
    T6,
    /// Program Counter
    Pc,
}

impl Register {
    fn from_prime(val: u16) -> Register {
        match val {
            0b000 => Register::S0,
            0b001 => Register::S1,
            0b010 => Register::A0,
            0b011 => Register::A1,
            0b100 => Register::A2,
            0b101 => Register::A3,
            0b110 => Register::A4,
            0b111 => Register::A5,
            _ => panic!("Invalid prime register"),
        }
    } 
}

impl From<u32> for Register {
    fn from(val: u32) -> Self {
        // TODO!: do it properly
        assert!(val < 33);
        unsafe {
            core::ptr::read_unaligned(&(val as usize) as
                                      *const usize as *const Register)
        }
    }
}
/// 64-bit RISC-V float registers
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum FloatRegister {
    /// FP temp, Caller Saved
    FT0,
    /// FP temp, Caller Saved
    FT1,
    /// FP temp, Caller Saved
    FT2,
    /// FP temp, Caller Saved
    FT3, 
    /// FP temp, Caller Saved
    FT4,
    /// FP temp, Caller Saved
    FT5, 
    /// FP temp, Caller Saved
    FT6, 
    /// FP temp, Caller Saved
    FT7, 
    /// FP saved registers, Callee Saved
    FS0,
    /// FP saved registers, Callee Saved
    FS1,
    /// FP Arguments / return values, Caller Saved
    FA0,
    /// FP Arguments / return values, Caller Saved
    FA1,
    /// FP Arguments, Caller Saved
    FA2,
    /// FP Arguments, Caller Saved
    FA3,
    /// FP Arguments, Caller Saved
    FA4,
    /// FP Arguments, Caller Saved
    FA5,
    /// FP Arguments, Caller Saved
    FA6,
    /// FP Arguments, Caller Saved
    FA7,
    /// FP saved registers, Callee Saved
    FS2,
    /// FP saved registers, Callee Saved
    FS3,
    /// FP saved registers, Callee Saved
    FS4,
    /// FP saved registers, Callee Saved
    FS5,
    /// FP saved registers, Callee Saved
    FS6,
    /// FP saved registers, Callee Saved
    FS7,
    /// FP saved registers, Callee Saved
    FS8,
    /// FP saved registers, Callee Saved
    FS9,
    /// FP saved registers, Callee Saved
    FS10,
    /// FP saved registers, Callee Saved
    FS11,
    /// FP temp, Caller Saved
    FT8,
    /// FP temp, Caller Saved
    FT9,
    /// FP temp, Caller Saved
    FT10,
    /// FP temp, Caller Saved
    FT11,
    /// Float Control Status Register
    FCSR,
}

impl FloatRegister {
    fn from_prime(val: u16) -> FloatRegister {
        match val {
            0b000 => FloatRegister::FS0,
            0b001 => FloatRegister::FS1,
            0b010 => FloatRegister::FA0,
            0b011 => FloatRegister::FA1,
            0b100 => FloatRegister::FA2,
            0b101 => FloatRegister::FA3,
            0b110 => FloatRegister::FA4,
            0b111 => FloatRegister::FA5,
            _ => panic!("Invalid prime float register"),
        }
    } 
}

impl From<u32> for FloatRegister {
    fn from(val: u32) -> Self {
        // TODO!: do it properly
        assert!(val < 33);
        unsafe {
            core::ptr::read_unaligned(&(val as usize) as
                                      *const usize as *const FloatRegister)
        }
    }
}

/// 64-bit RISC-V float rounding modes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FloatRoundingMode {
    /// Round to Nearest, ties to Even
    RNE,
    /// Round towards Zero
    RTZ,
    /// Round Down (torwards -inf)
    RDN,
    /// Round Up (towards +inf)
    RUP,
    /// Round to nearest, ties to Max Magnitude
    RMM,
    /// In instruction's `rm` field, selects dynamic rounding mode;
    /// In Rounding Mode register, reserved
    DYN,
}

impl From<u32> for FloatRoundingMode {
    fn from(val: u32) -> Self {
        match val {
            0b000 => FloatRoundingMode::RNE,
            0b001 => FloatRoundingMode::RTZ,
            0b010 => FloatRoundingMode::RDN,
            0b011 => FloatRoundingMode::RUP,
            0b100 => FloatRoundingMode::RMM,
            0b111 => FloatRoundingMode::DYN,
            _ => unreachable!("Invalid rounding mode"),
        }
    }
}

/// An R-type instruction
#[derive(Debug)]
struct Rtype {
    funct7: u32,
    rs2:    u32,
    rs1:    u32,
    funct3: u32,
    rd:     u32,
}

impl From<u32> for Rtype {
    fn from(inst: u32) -> Self {
        debug_assert_eq!(inst & 0b11, 0b11);
        Rtype {
            funct7: (inst >> 25) & 0b1111111,
            rs2:    (inst >> 20) & 0b11111,
            rs1:    (inst >> 15) & 0b11111,
            funct3: (inst >> 12) & 0b111,
            rd:     (inst >>  7) & 0b11111,
        }
    }
}

/// An R4-type instruction
#[derive(Debug)]
struct R4type {
    funct2: u32,
    rs3:    u32,
    rs2:    u32,
    rs1:    u32,
    funct3: u32,
    rd:     u32,
}

impl From<u32> for R4type {
    fn from(inst: u32) -> Self {
        debug_assert_eq!(inst & 0b11, 0b11);
        R4type {
            funct2: (inst >> 25) & 0b11,
            rs3:    (inst >> 27) & 0b11111,
            rs2:    (inst >> 20) & 0b11111,
            rs1:    (inst >> 15) & 0b11111,
            funct3: (inst >> 12) & 0b111,
            rd:     (inst >>  7) & 0b11111,
        }
    }
}

/// An S-type instruction
#[derive(Debug)]
struct Stype {
    imm:    i32,
    rs2:    u32,
    rs1:    u32,
    funct3: u32,
}

impl From<u32> for Stype {
    fn from(inst: u32) -> Self {
        debug_assert_eq!(inst & 0b11, 0b11);
        let imm115 = (inst >> 25) & 0b1111111;
        let imm40  = (inst >>  7) & 0b11111;

        let imm = (imm115 << 5) | imm40;
        let imm = ((imm as i32) << 20) >> 20;

        Stype {
            imm:    imm,
            rs2:    (inst >> 20) & 0b11111,
            rs1:    (inst >> 15) & 0b11111,
            funct3: (inst >> 12) & 0b111,
        }
    }
}

/// A J-type instruction
#[derive(Debug)]
struct Jtype {
    imm: i32,
    rd:  u32,
}

impl From<u32> for Jtype {
    fn from(inst: u32) -> Self {
        debug_assert_eq!(inst & 0b11, 0b11);
        let imm20   = (inst >> 31) & 1;
        let imm101  = (inst >> 21) & 0b1111111111;
        let imm11   = (inst >> 20) & 1;
        let imm1912 = (inst >> 12) & 0b11111111;

        let imm = (imm20 << 20) | (imm1912 << 12) | (imm11 << 11) |
            (imm101 << 1);
        let imm = ((imm as i32) << 11) >> 11;

        Jtype {
            imm: imm,
            rd:  (inst >> 7) & 0b11111,
        }
    }
}

/// A B-type instruction
#[derive(Debug)]
struct Btype {
    imm:    i32,
    rs2:    u32,
    rs1:    u32,
    funct3: u32,
}

impl From<u32> for Btype {
    fn from(inst: u32) -> Self {
        debug_assert_eq!(inst & 0b11, 0b11);
        let imm12  = (inst >> 31) & 1;
        let imm105 = (inst >> 25) & 0b111111;
        let imm41  = (inst >>  8) & 0b1111;
        let imm11  = (inst >>  7) & 1;

        let imm = (imm12 << 12) | (imm11 << 11) |(imm105 << 5) | (imm41 << 1);
        let imm = ((imm as i32) << 19) >> 19;

        Btype {
            imm:    imm,
            rs2:    (inst >> 20) & 0b11111,
            rs1:    (inst >> 15) & 0b11111,
            funct3: (inst >> 12) & 0b111,
        }
    }
}

/// An I-type instruction
#[derive(Debug)]
struct Itype {
    imm:    i32,
    rs1:    u32,
    funct3: u32,
    rd:     u32,
}

impl From<u32> for Itype {
    fn from(inst: u32) -> Self {
        debug_assert_eq!(inst & 0b11, 0b11);
        let imm = (inst as i32) >> 20;
        Itype {
            imm:    imm,
            rs1:    (inst >> 15) & 0b11111,
            funct3: (inst >> 12) & 0b111,
            rd:     (inst >>  7) & 0b11111,
        }
    }
}

#[derive(Debug)]
struct Utype {
    imm: i32,
    rd:  u32,
}

impl From<u32> for Utype {
    fn from(inst: u32) -> Self {
        debug_assert_eq!(inst & 0b11, 0b11);
        Utype {
            imm: (inst & !0xfff) as i32,
            rd:  (inst >> 7) & 0b11111,
        }
    }
}

#[derive(Debug)]
struct CRtype {
    funct4: u16,
    rd_rs1: u16,
    rs2:    u16,
}

impl From<u16> for CRtype {
    fn from(inst: u16) -> Self {
        debug_assert_eq!(inst & 0b11, 0b00);
        CRtype {
            funct4: (inst >> 12) & 0b1111,
            rd_rs1: (inst >>  7) & 0b11111,
            rs2:    (inst >>  2) & 0b11111,
        }
    }
}

#[derive(Debug)]
struct CItype {
    funct3: u16,
    imm2:   u16,
    rd_rs1: u16,
    imm1:   u16,
}

impl From<u16> for CItype {
    fn from(inst: u16) -> Self {
        debug_assert_eq!(inst & 0b11, 0b00);
        CItype {
            funct3: (inst >> 13) & 0b111,
            imm2:   (inst >> 12) & 0b1,
            rd_rs1: (inst >>  7) & 0b11111,
            imm1:   (inst >>  2) & 0b11111,
        }
    }
}

#[derive(Debug)]
struct CSStype {
    funct3: u16,
    imm:    u16,
    rs2:    u16,
}

impl From<u16> for CSStype {
    fn from(inst: u16) -> Self {
        debug_assert_eq!(inst & 0b11, 0b00);
        CSStype {
            funct3: (inst >> 13) & 0b111,
            imm:    (inst >>  7) & 0b111111,
            rs2:    (inst >>  2) & 0b11111,
        }
    }
}

#[derive(Debug)]
struct CIWtype {
    funct3:   u16,
    imm:      u16,
    rd_prime: u16,
}

impl From<u16> for CIWtype {
    fn from(inst: u16) -> Self {
        debug_assert_eq!(inst & 0b11, 0b00);
        CIWtype {
            funct3:   (inst >> 13) & 0b111,
            imm:      (inst >>  5) & 0b11111111,
            rd_prime: (inst >>  2) & 0b111,
        }
    }
}

#[derive(Debug)]
struct CLtype {
    funct3:    u16,
    imm2:      u16,
    rs1_prime: u16,
    imm1:      u16,
    rd_prime:  u16,
}

impl From<u16> for CLtype {
    fn from(inst: u16) -> Self {
        debug_assert_eq!(inst & 0b11, 0b00);
        CLtype {
            funct3:    (inst >> 13) & 0b111,
            imm2:      (inst >>  10) & 0b111,
            rs1_prime: (inst >>  7) & 0b111,
            imm1:      (inst >>  5) & 0b11,
            rd_prime:  (inst >>  2) & 0b111,
        }
    }
}

#[derive(Debug)]
struct CStype {
    funct3:    u16,
    imm2:      u16,
    rs1_prime: u16,
    imm1:      u16,
    rs2_prime:  u16,
}

impl From<u16> for CStype {
    fn from(inst: u16) -> Self {
        debug_assert_eq!(inst & 0b11, 0b00);
        CStype {
            funct3:    (inst >> 13) & 0b111,
            imm2:      (inst >> 10) & 0b111,
            rs1_prime: (inst >>  7) & 0b111,
            imm1:      (inst >>  5) & 0b11,
            rs2_prime: (inst >>  2) & 0b111,
        }
    }
}

#[derive(Debug)]
struct CAtype {
    funct6:       u16,
    rd_rs1_prime: u16,
    funct2:       u16,
    rs2_prime:    u16,
}

impl From<u16> for CAtype {
    fn from(inst: u16) -> Self {
        debug_assert_eq!(inst & 0b11, 0b00);
        CAtype {
            funct6:       (inst >> 10) & 0b111111,
            rd_rs1_prime: (inst >>  7) & 0b111,
            funct2:       (inst >>  5) & 0b11,
            rs2_prime:    (inst >>  2) & 0b111,
        }
    }
}

#[derive(Debug)]
struct CBtype {
    funct3:    u16,
    offset2:   u16,
    rs1_prime: u16,
    offset1:   u16,
}

impl From<u16> for CBtype {
    fn from(inst: u16) -> Self {
        debug_assert_eq!(inst & 0b11, 0b00);
        CBtype {
            funct3:    (inst >> 13) & 0b111,
            offset2:   (inst >> 10) & 0b111,
            rs1_prime: (inst >>  7) & 0b111,
            offset1:   (inst >>  2) & 0b11111,
        }
    }
}

#[derive(Debug)]
struct CJtype {
    funct3:      u16,
    jump_target: u16,
}

impl From<u16> for CJtype {
    fn from(inst: u16) -> Self {
        debug_assert_eq!(inst & 0b11, 0b00);
        CJtype {
            funct3:      (inst >> 13) & 0b111,
            jump_target: (inst >> 2) & 0b1111111111,
        }
    }
}

/// Helper function to build compact integers
fn compose_imms_53_76(imm1: u16, imm2: u16) -> u16 {
    (imm1 << 6) | (imm2 << 3)
}

/// Helper function to build compact integers
fn compose_imms_53_2_or_6(imm1: u16, imm2: u16) -> u16 {
    ((imm1 & 0b1) << 6) | (imm2 << 3) | (imm1 & 0b10) 
}

/// Instructions skipped: `uret, srtet, mret, wfi, sfence.vma`
/// Extensions Skipped: `RV32A, RV64A` do I need atomic right now?
/// G = IMAFD, Zicsr, Zifencei
/// 
/// Compact instructions will receive already translated registers
/// so I can implement it once.
pub trait RV64GUser<T> {
    type Error;

    /// # Load Upper Immediate (RV32I)
    /// 
    /// Build 32-bit constants and uses the U-type format. LUI places the 
    /// U-immediate value in the top 20 bits of the destination register rd, 
    /// filling in the lowest 12 bits with zeros.
    /// 
    /// `x[rd] = sext(immediate[31:12] << 12)`
    fn lui(&mut self, rd: Register, imm: i32) -> Result<T, Self::Error>;

    /// # Add Upper Immediate to PC (RV32I)
    /// 
    /// Build pc-relative addresses and uses the U-type format. AUIPC forms a 
    /// 32-bit offset from the 20-bit U-immediate, filling in the lowest 12 bits 
    /// with zeros, adds this offset to the pc, then places the result in 
    /// register rd.
    /// 
    /// `x[rd] = pc + sext(immediate[31:12] << 12)`
    fn auipc(&mut self, rd: Register, imm: u64) -> Result<T, Self::Error>;

    /// # Add Immediate (RV32I)
    /// 
    /// Adds the sign-extended 12-bit immediate to register rs1. Arithmetic 
    /// overflow is ignored and the result is simply the low XLEN bits of the 
    /// result. ADDI rd, rs1, 0 is used to implement the MV rd, rs1 assembler 
    /// pseudo-instruction.
    /// 
    /// `x[rd] = x[rs1] + sext(immediate)`
    fn addi(&mut self, rd: Register, rs1: Register, imm: u64) 
        -> Result<T, Self::Error>;

    /// # Set Less Than Immediate (RV32I)
    /// 
    /// Place the value 1 in register rd if register rs1 is less than the 
    /// signextended immediate when both are treated as signed numbers, else 0 
    /// is written to rd.
    /// 
    /// `x[rd] = x[rs1] <s sext(immediate)`
    fn slti(&mut self, rd: Register, rs1: Register, imm: u64) 
        -> Result<T, Self::Error>;

    /// # Set Less Than Immediate Unsigned (RV32I)
    /// 
    /// Place the value 1 in register rd if register rs1 is less than the 
    /// immediate when both are treated as unsigned numbers, else 0 is written 
    /// to rd.
    /// 
    /// `x[rd] = x[rs1] <u sext(immediate)`
    fn sltiu(&mut self, rd: Register, rs1: Register, imm: u64)
        -> Result<T, Self::Error>;

    /// # Xor Immediate (RV32I)
    /// 
    /// Performs bitwise XOR on register rs1 and the sign-extended 12-bit 
    /// immediate and place the result in rd
    /// Note, “XORI rd, rs1, -1” performs a bitwise logical inversion of 
    /// register rs1(assembler pseudo-instruction NOT rd, rs)
    /// 
    /// `x[rd] = x[rs1] ^ sext(immediate)`
    fn xori(&mut self, rd: Register, rs1: Register, imm: u64) 
        -> Result<T, Self::Error>;

    /// # Or Immediate (RV32I)
    /// 
    /// Performs bitwise OR on register rs1 and the sign-extended 12-bit 
    /// immediate and place the result in rd
    /// 
    /// `x[rd] = x[rs1] | sext(immediate)`
    fn ori(&mut self, rd: Register, rs1: Register, imm: u64) 
        -> Result<T, Self::Error>;

    /// # And Immediate (RV32I)
    /// 
    /// Performs bitwise AND on register rs1 and the sign-extended 12-bit 
    /// immediate and place the result in rd
    /// 
    /// `x[rd] = x[rs1] & sext(immediate)`
    fn andi(&mut self, rd: Register, rs1: Register, imm: u64) 
        -> Result<T, Self::Error>;

    /// # Shift Left Logical Immediate (RV32I)
    /// 
    /// Performs logical left shift on the value in register rs1 by the shift 
    /// amount held in the lower 5 bits of the immediate.
    /// In RV64, bit-25 is used to shamt[5].
    /// 
    /// `x[rd] = x[rs1] << shamt`
    fn slli(&mut self, rd: Register, rs1: Register, shamt: i32) 
        -> Result<T, Self::Error>;

    /// # Shift Right Logical Immediate (RV32I)
    /// 
    /// Performs logical right shift on the value in register rs1 by the shift 
    /// amount held in the lower 5 bits of the immediate
    /// In RV64, bit-25 is used to shamt[5].
    /// 
    /// `x[rd] = x[rs1] >>u shamt`
    fn srli(&mut self, rd: Register, rs1: Register, shamt: i32) 
        -> Result<T, Self::Error>;

    /// # Shift Right Arithmetical Immediate (RV32I)
    /// 
    /// Performs arithmetic right shift on the value in register rs1 by the 
    /// shift amount held in the lower 5 bits of the immediate
    /// In RV64, bit-25 is used to shamt[5].
    /// 
    /// `x[rd] = x[rs1] >>s shamt`
    fn srai(&mut self, rd: Register, rs1: Register, shamt: i32) 
        -> Result<T, Self::Error>;

    /// # Add (RV32I)
    /// 
    /// Adds the registers rs1 and rs2 and stores the result in rd.
    /// Arithmetic overflow is ignored and the result is simply the low XLEN 
    /// bits of the result.
    /// 
    /// `x[rd] = x[rs1] + x[rs2]`
    fn add(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Sub (RV32I)
    /// 
    /// Subs the register rs2 from rs1 and stores the result in rd.
    /// Arithmetic overflow is ignored and the result is simply the low XLEN 
    /// bits of the result.
    /// 
    /// `x[rd] = x[rs1] - x[rs2]`
    fn sub(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;
    
    /// # Shift Logical Left (RV32I)
    /// 
    /// Performs logical left shift on the value in register rs1 by the shift 
    /// amount held in the lower 5 bits of register rs2.
    /// 
    /// `x[rd] = x[rs1] << x[rs2]`
    fn sll(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Set Less Than (RV32I)
    /// 
    /// Place the value 1 in register rd if register rs1 is less than register 
    /// rs2 when both are treated as signed numbers, else 0 is written to rd.
    /// 
    /// `x[rd] = x[rs1] <s x[rs2]`
    fn slt(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Set Less Than Unsigned (RV32I)
    /// 
    /// Place the value 1 in register rd if register rs1 is less than register 
    /// rs2 when both are treated as unsigned numbers, else 0 is written to rd.
    /// 
    /// `x[rd] = x[rs1] <u x[rs2]`
    fn sltu(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Xor (RV32I)
    /// 
    /// Performs bitwise XOR on registers rs1 and rs2 and place the result in rd
    /// 
    /// `x[rd] = x[rs1] ^ x[rs2]`
    fn xor(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Shift Right Logical (RV32I)
    /// 
    /// Logical right shift on the value in register rs1 by the shift amount 
    /// held in the lower 5 bits of register rs2
    /// 
    /// `x[rd] = x[rs1] >>u x[rs2]`
    fn srl(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Shift Right Arithmetical (RV32I)
    /// 
    /// Performs arithmetic right shift on the value in register rs1 by the 
    /// shift amount held in the lower 5 bits of register rs2
    /// 
    /// `x[rd] = x[rs1] >>s x[rs2]`
    fn sra(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Or (RV32I)
    /// 
    /// Performs bitwise OR on registers rs1 and rs2 and place the result in rd
    /// 
    /// `x[rd] = x[rs1] | x[rs2]`
    fn or(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # And (RV32I)
    /// 
    /// Performs bitwise AND on registers rs1 and rs2 and place the result in rd
    /// 
    /// `x[rd] = x[rs1] & x[rs2]`
    fn and(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Fence (RV32I)
    /// TODO!: check args
    /// 
    /// Used to order device I/O and memory accesses as viewed by other RISC-V 
    /// harts and external devices or coprocessors.
    /// Any combination of device input (I), device output (O), memory reads (R)
    /// , and memory writes (W) may be ordered with respect to any combination 
    /// of the same.
    /// Informally, no other RISC-V hart or external device can observe any 
    /// operation in the successor set following a FENCE before any operation in
    /// the predecessor set preceding the FENCE.
    fn fence(&mut self) -> Result<T, Self::Error>;

    /// # Fence Instructions (RV32Zifencei)
    /// TODO!: check args
    /// 
    /// Provides explicit synchronization between writes to instruction memory 
    /// and instruction fetches on the same hart.
    fn fence_i(&mut self) -> Result<T, Self::Error>;

    /// # Atomic Read / Write CSR (RV32Zicsr)
    /// 
    /// Atomically swaps values in the CSRs and integer registers.
    /// CSRRW reads the old value of the CSR, zero-extends the value to XLEN 
    /// bits, then writes it to integer register rd.
    /// The initial value in rs1 is written to the CSR.
    /// If rd=x0, then the instruction shall not read the CSR and shall not 
    /// cause any of the side effects that might occur on a CSR read.
    /// 
    /// `t = CSRs[csr]; CSRs[csr] = x[rs1]; x[rd] = t`
    fn csrrw(&mut self, rd: Register, rs1: Register, offset: u32) 
        -> Result<T, Self::Error>;

    /// # Atomic read and set bits in CSR (RV32Zicsr)
    /// 
    /// Reads the value of the CSR, zero-extends the value to XLEN bits, and 
    /// writes it to integer register rd.
    /// The initial value in integer register rs1 is treated as a bit mask that 
    /// specifies bit positions to be set in the CSR.
    /// Any bit that is high in rs1 will cause the corresponding bit to be set 
    /// in the CSR, if that CSR bit is writable.
    /// Other bits in the CSR are unaffected (though CSRs might have side 
    /// effects when written).
    /// 
    /// `t = CSRs[csr]; CSRs[csr] = t | x[rs1]; x[rd] = t`
    fn csrrs(&mut self, rd: Register, rs1: Register, offset: u32) 
        -> Result<T, Self::Error>;

    /// # Atomic read and clear bits in CSR (RV32Zicsr)
    /// 
    /// Reads the value of the CSR, zero-extends the value to XLEN bits, and 
    /// writes it to integer register rd.
    /// The initial value in integer register rs1 is treated as a bit mask that 
    /// specifies bit positions to be cleared in the CSR.
    /// Any bit that is high in rs1 will cause the corresponding bit to be 
    /// cleared in the CSR, if that CSR bit is writable.
    /// Other bits in the CSR are unaffected.
    /// 
    /// `t = CSRs[csr]; CSRs[csr] = t &∼x[rs1]; x[rd] = t`
    fn csrrc(&mut self, rd: Register, rs1: Register, offset: u32) 
        -> Result<T, Self::Error>;

    /// # Update CSR Immediate (RV32Zicsr)
    /// 
    /// Update the CSR using an XLEN-bit value obtained by zero-extending a 
    /// 5-bit unsigned immediate (uimm[4:0]) field encoded in the rs1 field.
    /// 
    /// `x[rd] = CSRs[csr]; CSRs[csr] = zimm`
    fn csrrwi(&mut self, rd: Register, zimm: u8, offset: u32) 
        -> Result<T, Self::Error>;

    /// # Set CSR Immediate (RV32Zicsr)
    /// 
    /// Set CSR bit using an XLEN-bit value obtained by zero-extending a 5-bit 
    /// unsigned immediate (uimm[4:0]) field encoded in the rs1 field.
    /// 
    /// `t = CSRs[csr]; CSRs[csr] = t | zimm; x[rd] = t`
    fn csrrsi(&mut self, rd: Register, zimm: u8, offset: u32) 
        -> Result<T, Self::Error>;

    /// # Clear CSR Immediate (RV32Zicsr)
    /// 
    /// Clear CSR bit using an XLEN-bit value obtained by zero-extending a 5-bit 
    /// unsigned immediate (uimm[4:0]) field encoded in the rs1 field.
    /// 
    /// `t = CSRs[csr]; CSRs[csr] = t &∼zimm; x[rd] = t`
    fn csrrci(&mut self, rd: Register, zimm: u8, offset: u32) 
        -> Result<T, Self::Error>;

    /// # ECall (RV32I)
    /// 
    /// Make a request to the supporting execution environment.
    /// When executed in U-mode, S-mode, or M-mode, it generates an 
    /// environment-call-from-U-mode exception, environment-call-from-S-mode 
    /// exception, or environment-call-from-M-mode exception, respectively, and 
    /// performs no other operation.
    fn ecall(&mut self) -> Result<T, Self::Error>;

    /// # EBreak (RV32I)
    /// 
    /// Used by debuggers to cause control to be transferred back to a debugging 
    /// environment.
    /// It generates a breakpoint exception and performs no other operation.
    fn ebreak(&mut self) -> Result<T, Self::Error>;

    /// # Load Byte (RV32I)
    /// 
    /// Loads a 8-bit value from memory and sign-extends this to XLEN bits 
    /// before storing it in register rd.
    /// 
    /// `x[rd] = sext(M[x[rs1] + sext(offset)][7:0])`
    fn lb(&mut self, rd: Register, imm: u64) -> Result<T, Self::Error>;

    /// # Load Half word (RV32I)
    /// 
    /// Loads a 16-bit value from memory and sign-extends this to XLEN bits 
    /// before storing it in register rd.
    /// 
    /// `x[rd] = sext(M[x[rs1] + sext(offset)][15:0])`
    fn lh(&mut self, rd: Register, imm: u64) -> Result<T, Self::Error>;

    /// # Load Word (RV32I)
    /// 
    /// Loads a 32-bit value from memory and sign-extends this to XLEN bits 
    /// before storing it in register rd.
    /// 
    /// `x[rd] = sext(M[x[rs1] + sext(offset)][31:0])`
    fn lw(&mut self, rd: Register, imm: u64) -> Result<T, Self::Error>;

    /// # Load Double word (RV64I)
    /// 
    /// Loads a 64-bit value from memory into register rd for RV64I.
    /// 
    /// `x[rd] = M[x[rs1] + sext(offset)][63:0]`
    fn ld(&mut self, rd: Register, imm: u64) -> Result<T, Self::Error>;

    /// # Load Byte Unsigned (RV32I)
    /// 
    /// Loads a 8-bit value from memory and zero-extends this to XLEN bits 
    /// before storing it in register rd.
    /// 
    /// `x[rd] = M[x[rs1] + sext(offset)][7:0]`
    fn lbu(&mut self, rd: Register, imm: u64) -> Result<T, Self::Error>;

    /// # Load Half word Unsigned (RV32I)
    /// 
    /// Loads a 16-bit value from memory and zero-extends this to XLEN bits 
    /// before storing it in register rd.
    /// 
    /// `x[rd] = M[x[rs1] + sext(offset)][15:0]`
    fn lhu(&mut self, rd: Register, imm: u64) -> Result<T, Self::Error>;

    /// # Load Word Unsigned (RV64I)
    /// 
    /// Loads a 32-bit value from memory and zero-extends this to 64 bits before 
    /// storing it in register rd.
    /// 
    /// `x[rd] = M[x[rs1] + sext(offset)][31:0]`
    fn lwu(&mut self, rd: Register, imm: u64) -> Result<T, Self::Error>;

    /// # Store Byte (RV32I)
    /// 
    /// Store 8-bit, values from the low bits of register rs2 to memory.
    /// 
    /// `M[x[rs1] + sext(offset)] = x[rs2][7:0]`
    fn sb(&mut self, rs1: Register, rs2: Register, imm: u64) 
        -> Result<T, Self::Error>;

    /// # Store Half word (RV32I)
    /// 
    /// Store 16-bit, values from the low bits of register rs2 to memory.
    /// 
    /// `M[x[rs1] + sext(offset)] = x[rs2][15:0]`
    fn sh(&mut self, rs1: Register, rs2: Register, imm: u64) 
        -> Result<T, Self::Error>;

    /// # Store Half word (RV32I)
    /// 
    /// Store 32-bit, values from the low bits of register rs2 to memory.
    /// 
    /// `M[x[rs1] + sext(offset)] = x[rs2][31:0]`
    fn sw(&mut self, rs1: Register, rs2: Register, imm: u64) 
        -> Result<T, Self::Error>;

    /// # Store Double word (RV64I)
    /// 
    /// Store 64-bit, values from register rs2 to memory.
    /// 
    /// `M[x[rs1] + sext(offset)] = x[rs2][63:0]`
    fn sd(&mut self, rs1: Register, rs2: Register, imm: u64) 
        -> Result<T, Self::Error>;

    /// # Jump and link(RV32I)
    /// 
    /// Jump to address and place return address in rd.
    /// 
    /// `x[rd] = pc+4; pc += sext(offset)`
    fn jal(&mut self, imm: u64) -> Result<T, Self::Error>;

    /// # Jump and link in relative (RV32I)
    /// 
    /// Jump to address and place return address in rd.
    /// 
    /// `t =pc+4; pc=(x[rs1]+sext(offset))&∼1; x[rd]=t`
    fn jalr(&mut self, rd: Register, imm: u64) -> Result<T, Self::Error>;

    /// # Branch Equal (RV32I)
    /// 
    /// Take the branch if registers rs1 and rs2 are equal.
    /// 
    /// `if (rs1 == rs2) pc += sext(offset)`
    fn beq(&mut self, rs1: Register, rs2: Register, imm: u64) 
        -> Result<T, Self::Error>;

    /// # Branch Not Equal (RV32I)
    /// 
    /// Take the branch if registers rs1 and rs2 are not equal.
    /// 
    /// `if (rs1 != rs2) pc += sext(offset)`
    fn bne(&mut self, rs1: Register, rs2: Register, imm: u64) 
        -> Result<T, Self::Error>;

    /// # Branch Less Than (RV32I)
    /// 
    /// Take the branch if registers rs1 is less than rs2, using signed 
    /// comparison.
    /// 
    /// `if (rs1 <s rs2) pc += sext(offset)`
    fn blt(&mut self, rs1: Register, rs2: Register, imm: u64) 
        -> Result<T, Self::Error>;

    /// # Branch Greater Equal (RV32I)
    /// 
    /// Take the branch if registers rs1 is greater than rs2, using signed 
    /// comparison.
    /// 
    /// `if (rs1 >=s rs2) pc += sext(offset)`
    fn bge(&mut self, rs1: Register, rs2: Register, imm: u64) 
        -> Result<T, Self::Error>;

    /// # Branch Less Than Unsigned (RV32I)
    /// 
    /// Take the branch if registers rs1 is less than rs2, using unsigned 
    /// comparison.
    /// 
    /// `if (rs1 >u rs2) pc += sext(offset)`
    fn bltu(&mut self, rs1: Register, rs2: Register, imm: u64) 
        -> Result<T, Self::Error>;

    /// # Branch Greater Equal Unsigned (RV32I)
    /// 
    /// Take the branch if registers rs1 is greater than rs2, using unsigned 
    /// comparison.
    /// 
    /// `if (rs1 >=u rs2) pc += sext(offset)`
    fn bgeu(&mut self, rs1: Register, rs2: Register, imm: u64) 
        -> Result<T, Self::Error>;

    /// # Add Immediate Word (RV64I)
    /// 
    /// Adds the sign-extended 12-bit immediate to register rs1 and produces the 
    /// proper sign-extension of a 32-bit result in rd.
    /// Overflows are ignored and the result is the low 32 bits of the result 
    /// sign-extended to 64 bits.
    /// Note, ADDIW rd, rs1, 0 writes the sign-extension of the lower 32 bits of 
    /// register rs1 into register rd (assembler pseudoinstruction SEXT.W).
    /// 
    /// `x[rd] = sext((x[rs1] + sext(immediate))[31:0])`
    fn addiw(&mut self, rd: Register, rsq: Register, imm: u32) 
        -> Result<T, Self::Error>;
        
    /// # Shift Left Logical Immediate Word (RV64I)
    /// 
    /// Performs logical left shift on the 32-bit of value in register rs1 by 
    /// the shift amount held in the lower 5 bits of the immediate.
    /// Encodings with $imm[5] neq 0$ are reserved.
    /// 
    /// `x[rd] = sext((x[rs1] << shamt)[31:0])`
    fn slliw(&mut self, rd: Register, rsq: Register, shamt: i32) 
        -> Result<T, Self::Error>;

    /// # Shift Right Logical Immediate Word (RV64I)
    /// 
    /// Performs logical right shift on the 32-bit of value in register rs1 by 
    /// the shift amount held in the lower 5 bits of the immediate.
    /// Encodings with $imm[5] neq 0$ are reserved.
    /// 
    /// `x[rd] = sext(x[rs1][31:0] >>u shamt)`
    fn srliw(&mut self, rd: Register, rsq: Register, shamt: i32) 
        -> Result<T, Self::Error>;

    /// # Shift Right Arithmetical Immediate Word (RV64I)
    /// 
    /// Performs arithmetic right shift on the 32-bit of value in register rs1 
    /// by the shift amount held in the lower 5 bits of the immediate.
    /// Encodings with $imm[5] neq 0$ are reserved.
    /// 
    /// `x[rd] = sext(x[rs1][31:0] >>s shamt)`
    fn sraiw(&mut self, rd: Register, rsq: Register, shamt: i32) 
        -> Result<T, Self::Error>;

    /// # Add Word (RV64I)
    /// 
    /// Adds the 32-bit of registers rs1 and 32-bit of register rs2 and stores 
    /// the result in rd.
    /// Arithmetic overflow is ignored and the low 32-bits of the result is 
    /// sign-extended to 64-bits and written to the destination register.
    /// 
    /// `x[rd] = sext((x[rs1] + x[rs2])[31:0])`
    fn addw(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;
        
    /// # Sub Word (RV64I)
    /// 
    /// Subtract the 32-bit of registers rs1 and 32-bit of register rs2 and 
    /// stores the result in rd.
    /// Arithmetic overflow is ignored and the low 32-bits of the result is 
    /// sign-extended to 64-bits and written to the destination register.
    /// 
    /// `x[rd] = sext((x[rs1] - x[rs2])[31:0])`
    fn subw(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;
    
    /// # Shift Left Logical Word (RV64I)
    /// 
    /// Performs logical left shift on the low 32-bits value in register rs1 by 
    /// the shift amount held in the lower 5 bits of register rs2 and produce 
    /// 32-bit results and written to the destination register rd.
    /// 
    /// `x[rd] = sext((x[rs1] << x[rs2][4:0])[31:0])`
    fn sllw(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Shift Right Logical Word (RV64I)
    /// 
    /// Performs logical right shift on the low 32-bits value in register rs1 by 
    /// the shift amount held in the lower 5 bits of register rs2 and produce 
    /// 32-bit results and written to the destination register rd.
    /// 
    /// `x[rd] = sext(x[rs1][31:0] >>u x[rs2][4:0])`
    fn srlw(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Shift Right Arithmetical Word (RV64I)
    /// 
    /// Performs arithmetic right shift on the low 32-bits value in register rs1 
    /// by the shift amount held in the lower 5 bits of register rs2 and produce 
    /// 32-bit results and written to the destination register rd.
    /// 
    /// `x[rd] = sext(x[rs1][31:0] >>s x[rs2][4:0])`
    fn sraw(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;
    
    /// # Multiply (RV32M)
    /// 
    /// performs an XLEN-bit × XLEN-bit multiplication of signed rs1 by signed 
    /// rs2 and places the lower XLEN bits in the destination register.
    /// 
    /// `x[rd] = x[rs1] × x[rs2]`
    fn mul(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;
    
    /// # Multiply Higher (RV32M)
    /// 
    /// performs an XLEN-bit × XLEN-bit multiplication of signed rs1 by signed 
    /// rs2 and places the upper XLEN bits in the destination register.
    /// 
    /// `x[rd] = (x[rs1] s×s x[rs2]) >>s XLEN`
    fn mulh(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;
    
    /// # Multiply Higher Signed x Unsigned (RV32M)
    /// 
    /// performs an XLEN-bit × XLEN-bit multiplication of signed rs1 by unsigned 
    /// rs2 and places the upper XLEN bits in the destination register.
    /// 
    /// `x[rd] = (x[rs1] s × x[rs2]) >>s XLEN`
    fn mulhsu(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Multiply Higher Unsigned x Unsigned (RV32M)
    /// 
    /// performs an XLEN-bit × XLEN-bit multiplication of unsigned rs1 by 
    /// unsigned rs2 and places the upper XLEN bits in the destination register.
    /// 
    /// `x[rd] = (x[rs1] u × x[rs2]) >>u XLEN`
    fn mulhu(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Divide (RV32M)
    /// 
    /// perform an XLEN bits by XLEN bits signed integer division of rs1 by rs2, 
    /// rounding towards zero.
    /// 
    /// `x[rd] = x[rs1] /s x[rs2]`
    fn div(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Divide Unsigned (RV32M)
    /// 
    /// perform an XLEN bits by XLEN bits unsigned integer division of rs1 by 
    /// rs2, rounding towards zero.
    /// 
    /// `x[rd] = x[rs1] /u x[rs2]`
    fn divu(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Reminder (RV32M)
    /// 
    /// perform an XLEN bits by XLEN bits signed integer reminder of rs1 by rs2.
    /// 
    /// `x[rd] = x[rs1] %s x[rs2]`
    fn rem(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Reminder Unsigned (RV32M)
    /// 
    /// perform an XLEN bits by XLEN bits unsigned integer reminder of rs1 by 
    /// rs2.
    /// 
    /// `x[rd] = x[rs1] %u x[rs2]`
    fn remu(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Multiply Word (RV64M)
    /// 
    /// performs an 32-bit × 32-bit multiplication of signed rs1 by signed 
    /// rs2 and places the lower 32 bits in the destination register.
    /// 
    /// `x[rd] = sext((x[rs1] × x[rs2])[31:0])`
    fn mulw(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Divide Word (RV64M)
    /// 
    /// perform an 32 bits by 32 bits signed integer division of rs1 by rs2.
    /// 
    /// `x[rd] = sext(x[rs1][31:0] /s x[rs2][31:0]`
    fn divw(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Divide Unsigned Word (RV64M)
    /// 
    /// perform an 32 bits by 32 bits unsigned integer division of rs1 by rs2.
    /// 
    /// `x[rd] = sext(x[rs1][31:0] /u x[rs2][31:0])`
    fn divuw(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;
    
    /// # Reminder Word(RV64M)
    /// 
    /// perform an 32 bits by 32 bits signed integer reminder of rs1 by rs2.
    /// 
    /// `x[rd] = sext(x[rs1][31:0] %s x[rs2][31:0])`
    fn remw(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Reminder Unsigned Word(RV64M)
    /// 
    /// perform an 32 bits by 32 bits unsigned integer reminder of rs1 by rs2.
    /// 
    /// `x[rd] = sext(x[rs1][31:0] %u x[rs2][31:0])`
    fn remuw(&mut self, rd: Register, rs1: Register, rs2: Register) 
        -> Result<T, Self::Error>;

    /// # Fused Multiply Addition Single Precision (RV32F)
    /// 
    /// Perform single-precision fused multiply addition.
    /// 
    /// `f[rd] = f[rs1]×f[rs2]+f[rs3]`
    fn fmadd_s(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, 
        rs2: FloatRegister, rs3: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Fused Multiply Subtraction Single Precision (RV32F)
    /// 
    /// Perform single-precision fused multiply subtraction.
    /// 
    /// `f[rd] = f[rs1]×f[rs2]-f[rs3]`
    fn fmsub_s(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, 
        rs2: FloatRegister, rs3: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Fused Negated Multiply Subtraction Single Precision (RV32F)
    /// 
    /// Perform single-precision fused negated multiply subtraction.
    /// 
    /// `f[rd] = -f[rs1]×f[rs2-f[rs3]`
    fn fnmsub_s(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, 
        rs2: FloatRegister, rs3: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Fused Negated Multiply Addition Single Precision (RV32F)
    /// 
    /// Perform single-precision fused negated multiply addition.
    /// 
    /// `f[rd] = -f[rs1]×f[rs2+f[rs3]`
    fn fnmadd_s(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, 
        rs2: FloatRegister, rs3: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Addition Single Precision (RV32F)
    /// 
    /// Perform single-precision floating-point addition.
    /// 
    /// `f[rd] = f[rs1] + f[rs2]`
    fn fadd_s(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Subtraction Single Precision (RV32F)
    /// 
    /// Perform single-precision floating-point substraction.
    /// 
    /// `f[rd] = f[rs1] - f[rs2]`
    fn fsub_s(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Multiplication Single Precision (RV32F)
    /// 
    /// Perform single-precision floating-point multiplication.
    /// 
    /// `f[rd] = f[rs1] x f[rs2]`
    fn fmul_s(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Division Single Precision (RV32F)
    /// 
    /// Perform single-precision floating-point division.
    /// 
    /// `f[rd] = f[rs1] / f[rs2]`
    fn fdiv_s(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Square Root Single Precision (RV32F)
    /// 
    /// Perform single-precision square root.
    /// 
    /// `f[rd] = sqrt(f[rs1])`
    fn fsqrt_s(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rm: FloatRoundingMode,
    ) 
        -> Result<T, Self::Error>;

    /// # Float Sign Single Precision (RV32F)
    /// 
    /// Produce a result that takes all bits except the sign bit from rs1.
    /// The result’s sign bit is rs2’s sign bit.
    /// 
    /// `f[rd] = {f[rs2][31], f[rs1][30:0]}`
    fn fsgnj_s(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
    ) -> Result<T, Self::Error>;

    /// # Float Sign Negated Single Precision (RV32F)
    /// 
    /// Produce a result that takes all bits except the sign bit from rs1.
    /// The result’s sign bit is opposite of rs2’s sign bit.
    /// 
    /// `f[rd] = {~f[rs2][31], f[rs1][30:0]}`
    fn fsgnjn_s(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
    ) -> Result<T, Self::Error>;

    /// # Float Sign Xor Single Precision (RV32F)
    /// 
    /// Produce a result that takes all bits except the sign bit from rs1.
    /// The result’s sign bit is XOR of sign bit of rs1 and rs2.
    /// 
    /// `f[rd] = {f[rs1][31] ^ f[rs2][31], f[rs1][30:0]}`
    fn fsgnjx_s(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
    ) -> Result<T, Self::Error>;

    /// # Float Min Single Precision (RV32F)
    /// 
    /// Write the smaller of single precision data in rs1 and rs2 to rd.
    /// 
    /// `f[rd] = min(f[rs1], f[rs2])`
    fn fmin_s(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
    ) -> Result<T, Self::Error>;

    /// # Float Max Single Precision (RV32F)
    /// 
    /// Write the larger of single precision data in rs1 and rs2 to rd.
    /// 
    /// `f[rd] = max(f[rs1], f[rs2])`
    fn fmax_s(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
    ) -> Result<T, Self::Error>;

    /// # Float Convert Word Single Precision (RV32F)
    /// 
    /// Convert a floating-point number in floating-point register rs1 to a 
    /// signed 32-bit in integer register rd.
    /// 
    /// `x[rd] = sext(s32_{f32}(f[rs1]))`
    fn fcvt_w_s(&mut self, 
        rd: Register, rs1: FloatRegister, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Convert Word Unsigned Single Precision (RV32F)
    /// 
    /// Convert a floating-point number in floating-point register rs1 to a 
    /// signed 32-bit in unsigned integer register rd.
    /// 
    /// `x[rd] = sext(u32_{f32}(f[rs1]))`
    fn fcvt_wu_s(&mut self, 
        rd: Register, rs1: FloatRegister, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Move Register Word (RV32F)
    /// 
    /// Move the single-precision value in floating-point register rs1 
    /// represented in IEEE 754-2008 encoding to the lower 32 bits of integer 
    /// register rd.
    /// 
    /// `x[rd] = sext(f[rs1][31:0])`
    fn fmv_x_w(&mut self, 
        rd: Register, rs1: FloatRegister, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Eq Single precision (RV32F)
    /// 
    /// Performs a quiet equal comparison between floating-point registers rs1 
    /// and rs2 and record the Boolean result in integer register rd.
    /// Only signaling NaN inputs cause an Invalid Operation exception.
    /// The result is 0 if either operand is NaN.
    /// 
    /// `x[rd] = f[rs1] == f[rs2]`
    fn feq_s(&mut self, 
        rd: Register, rs1: FloatRegister, rs2: FloatRegister,
    ) -> Result<T, Self::Error>;

    /// # Float Less Than Single precision (RV32F)
    /// 
    /// Performs a quiet less comparison between floating-point registers rs1 
    /// and rs2 and record the Boolean result in integer register rd.
    /// Only signaling NaN inputs cause an Invalid Operation exception.
    /// The result is 0 if either operand is NaN.
    /// 
    /// `x[rd] = f[rs1] <==> f[rs2]`
    fn flt_s(&mut self, 
        rd: Register, rs1: FloatRegister, rs2: FloatRegister,
    ) -> Result<T, Self::Error>;

    /// # Float Less Equal Single precision (RV32F)
    /// 
    /// Performs a quiet less or equal comparison between floating-point 
    /// registers rs1 and rs2 and record the Boolean result in integer register 
    /// rd.
    /// Only signaling NaN inputs cause an Invalid Operation exception.
    /// The result is 0 if either operand is NaN.
    /// 
    /// `x[rd] = f[rs1] <= f[rs2]`
    fn fle_s(&mut self, 
        rd: Register, rs1: FloatRegister, rs2: FloatRegister,
    ) -> Result<T, Self::Error>;

    /// # Float classify Single precision (RV32F)
    /// 
    /// Examines the value in floating-point register rs1 and writes to integer 
    /// register rd a 10-bit mask that indicates the class of the floating-point 
    /// number. The format of the mask is described in [classify table]_.
    /// The corresponding bit in rd will be set if the property is true and 
    /// clear otherwise.
    /// All other bits in rd are cleared. Note that exactly one bit in rd will 
    /// be set.
    /// 
    /// `x[rd] = classifys(f[rs1])`
    fn fclass_s(&mut self, rd: Register, rs1: FloatRegister) 
        -> Result<T, Self::Error>;
    
    /// # Float Convert Single Precision to Word (RV32F)
    /// 
    /// Converts a 32-bit signed integer, in integer register rs1 into a 
    /// floating-point number in floating-point register rd.
    /// 
    /// `f[rd] = f32_{s32}(x[rs1])`
    fn fcvt_s_w(&mut self, 
        rd: FloatRegister, rs1: Register, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Convert Single Precision to Word Unsigned (RV32F)
    /// 
    /// Converts a 32-bit unsigned integer, in integer register rs1 into a 
    /// floating-point number in floating-point register rd.
    /// 
    /// `f[rd] = f32_{u32}(x[rs1])`
    fn fcvt_s_wu(&mut self, 
        rd: FloatRegister, rs1: Register, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Move Word Register (RV32F)
    /// 
    /// Move the single-precision value encoded in IEEE 754-2008 standard 
    /// encoding from the lower 32 bits of integer register rs1 to the 
    /// floating-point register rd.
    /// 
    /// `f[rd] = x[rs1][31:0]`
    fn fmv_w_x(&mut self, rd: FloatRegister, rs1: Register) 
        -> Result<T, Self::Error>;

    /// # Fused Multiply Addition Double Precision (RV32D)
    /// 
    /// Perform double-precision fused multiply addition.
    /// 
    /// `f[rd] = f[rs1]×f[rs2]+f[rs3]`
    fn fmadd_d(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, 
        rs2: FloatRegister, rs3: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Fused Multiply Subtraction Double Precision (RV32D)
    /// 
    /// Perform double-precision fused multiply subtraction.
    /// 
    /// `f[rd] = f[rs1]×f[rs2]-f[rs3]`
    fn fmsub_d(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, 
        rs2: FloatRegister, rs3: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Fused Negated Multiply Subtraction Double Precision (RV32D)
    /// 
    /// Perform double-precision fused negated multiply subtraction.
    /// 
    /// `f[rd] = -f[rs1]×f[rs2-f[rs3]`
    fn fnmsub_d(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, 
        rs2: FloatRegister, rs3: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Fused Negated Multiply Addition Double Precision (RV32D)
    /// 
    /// Perform double-precision fused negated multiply addition.
    /// 
    /// `f[rd] = -f[rs1]×f[rs2+f[rs3]`
    fn fnmadd_d(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, 
        rs2: FloatRegister, rs3: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Addition Double Precision (RV32D)
    /// 
    /// Perform double-precision floating-point addition.
    /// 
    /// `f[rd] = f[rs1] + f[rs2]`
    fn fadd_d(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Subtraction Double Precision (RV32D)
    /// 
    /// Perform double-precision floating-point substraction.
    /// 
    /// `f[rd] = f[rs1] - f[rs2]`
    fn fsub_d(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Multiplication Double Precision (RV32F)
    /// 
    /// Perform double-precision floating-point multiplication.
    /// 
    /// `f[rd] = f[rs1] x f[rs2]`
    fn fmul_d(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Division Double Precision (RV32F)
    /// 
    /// Perform double-precision floating-point division.
    /// 
    /// `f[rd] = f[rs1] / f[rs2]`
    fn fdiv_d(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Square Root Double Precision (RV32D)
    /// 
    /// Perform double-precision square root.
    /// 
    /// `f[rd] = sqrt(f[rs1])`
    fn fsqrt_d(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Sign Double Precision (RV32D)
    /// 
    /// Produce a result that takes all bits except the sign bit from rs1.
    /// The result’s sign bit is rs2’s sign bit.
    /// 
    /// `f[rd] = {f[rs2][63], f[rs1][62:0]}`
    fn fsgnj_d(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
    ) -> Result<T, Self::Error>;

    /// # Float Sign Negated Double Precision (RV32D)
    /// 
    /// Produce a result that takes all bits except the sign bit from rs1.
    /// The result’s sign bit is opposite of rs2’s sign bit.
    /// 
    /// `f[rd] = {~f[rs2][63], f[rs1][62:0]}`
    fn fsgnjn_d(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
    ) -> Result<T, Self::Error>;

    /// # Float Sign Xor Double Precision (RV32D)
    /// 
    /// Produce a result that takes all bits except the sign bit from rs1.
    /// The result’s sign bit is XOR of sign bit of rs1 and rs2.
    /// 
    /// `f[rd] = {f[rs1][63] ^ f[rs2][63], f[rs1][62:0]}`
    fn fsgnjx_d(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
    ) -> Result<T, Self::Error>;

    /// # Float Min Double Precision (RV32D)
    /// 
    /// Write the smaller of double precision data in rs1 and rs2 to rd.
    /// 
    /// `f[rd] = min(f[rs1], f[rs2])`
    fn fmin_d(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
    ) -> Result<T, Self::Error>;

    /// # Float Max Double Precision (RV32D)
    /// 
    /// Write the larger of double precision data in rs1 and rs2 to rd.
    /// 
    /// `f[rd] = max(f[rs1], f[rs2])`
    fn fmax_d(&mut self, 
        rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister,
    ) -> Result<T, Self::Error>;

    /// # Float Convert Double Precision to Single Precision (RV32D)
    /// 
    /// Converts double floating-point register in rs1 into a floating-point 
    /// number in floating-point register rd.
    /// 
    /// `f[rd] = f32_{f64}(f[rs1])`
    fn fcvt_s_d(&mut self, rd: FloatRegister, rs1: FloatRegister) 
        -> Result<T, Self::Error>;

    /// # Float Convert Single Precision to Double Precision (RV32D)
    /// 
    /// Converts single floating-point register in rs1 into a double 
    /// floating-point number in floating-point register rd.
    /// 
    /// `f[rd] = f64_{f32}(f[rs1])`
    fn fcvt_d_s(&mut self, rd: FloatRegister, rs1: FloatRegister) 
        -> Result<T, Self::Error>;

    /// # Float Eq Double precision (RV32D)
    /// 
    /// Performs a quiet equal comparison between floating-point registers rs1 
    /// and rs2 and record the Boolean result in integer register rd.
    /// Only signaling NaN inputs cause an Invalid Operation exception.
    /// The result is 0 if either operand is NaN.
    /// 
    /// `x[rd] = f[rs1] == f[rs2]`
    fn feq_d(&mut self, 
        rd: Register, rs1: FloatRegister, rs2: FloatRegister,
    ) -> Result<T, Self::Error>;

    /// # Float Less Than Double precision (RV32D)
    /// 
    /// Performs a quiet less comparison between floating-point registers rs1 
    /// and rs2 and record the Boolean result in integer register rd.
    /// Only signaling NaN inputs cause an Invalid Operation exception.
    /// The result is 0 if either operand is NaN.
    /// 
    /// `x[rd] = f[rs1] <==> f[rs2]`
    fn flt_d(&mut self, 
        rd: Register, rs1: FloatRegister, rs2: FloatRegister,
    ) -> Result<T, Self::Error>;

    /// # Float Less Equal Double precision (RV32D)
    /// 
    /// Performs a quiet less or equal comparison between floating-point 
    /// registers rs1 and rs2 and record the Boolean result in integer register 
    /// rd.
    /// Only signaling NaN inputs cause an Invalid Operation exception.
    /// The result is 0 if either operand is NaN.
    /// 
    /// `x[rd] = f[rs1] <= f[rs2]`
    fn fle_d(&mut self, 
        rd: Register, rs1: FloatRegister, rs2: FloatRegister,
    ) -> Result<T, Self::Error>;

    /// # Float classify Double precision (RV32D)
    /// 
    /// Examines the value in floating-point register rs1 and writes to integer 
    /// register rd a 10-bit mask that indicates the class of the floating-point 
    /// number.
    /// The format of the mask is described in table [classify table]_.
    /// The corresponding bit in rd will be set if the property is true and 
    /// clear otherwise.
    /// All other bits in rd are cleared. Note that exactly one bit in rd will 
    /// be set.
    /// 
    /// `x[rd] = classifys(f[rs1])`
    fn fclass_d(&mut self, rd: Register, rs1: FloatRegister) 
        -> Result<T, Self::Error>;
    
    /// # Float Convert Word to Double Precision (RV32D)
    /// 
    /// Converts a double-precision floating-point number in floating-point 
    /// register rs1 to a signed 32-bit integer, in integer register rd.
    /// 
    /// `x[rd] = sext(s32_{f64}(f[rs1]))`
    fn fcvt_w_d(&mut self, 
        rd: Register, rs1: FloatRegister, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;
    
    /// # Float Convert Word Unsigned to Double Precision (RV32D)
    /// 
    /// Converts a double-precision floating-point number in floating-point 
    /// register rs1 to a unsigned 32-bit integer, in integer register rd.
    /// 
    /// `x[rd] = sext(u32f64(f[rs1]))`
    fn fcvt_wu_d(&mut self, 
        rd: Register, rs1: FloatRegister, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Convert Double Precision to Word (RV32D)
    /// 
    /// Converts a 32-bit signed integer, in integer register rs1 into a 
    /// double-precision floating-point number in floating-point register rd.
    /// 
    /// `x[rd] = sext(s32_{f64}(f[rs1]))`
    fn fcvt_d_w(&mut self, 
        rd: FloatRegister, rs1: Register, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Convert Double Precision to Word Unsigned (RV32D)
    /// 
    /// Converts a 32-bit unsigned integer, in integer register rs1 into a 
    /// double-precision floating-point number in floating-point register rd.
    /// 
    /// `f[rd] = f64_{u32}(x[rs1])`
    fn fcvt_d_wu(&mut self, 
        rd: FloatRegister, rs1: Register, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Load Single Precision (RV32F)
    /// 
    /// Load a single-precision floating-point value from memory into 
    /// floating-point register rd.
    /// 
    /// `f[rd] = M[x[rs1] + sext(offset)][31:0]`
    fn flw(&mut self, rd: FloatRegister, rs1: Register, imm: u32) 
        -> Result<T, Self::Error>;
    
    /// # Float Store Single Precision (RV32F)
    /// 
    /// Store a single-precision value from floating-point register rs2 to 
    /// memory.
    /// 
    /// `M[x[rs1] + sext(offset)] = f[rs2][31:0]`
    fn fsw(&mut self, rs1: Register, rs2: FloatRegister, offset: u32) 
        -> Result<T, Self::Error>;

    /// # Float Load Double Precision (RV32D)
    /// 
    /// Load a double-precision floating-point value from memory into 
    /// floating-point register rd.
    /// 
    /// `f[rd] = M[x[rs1] + sext(offset)][63:0]`
    fn fld(&mut self, rd: FloatRegister, rs1: Register, offset: u32) 
        -> Result<T, Self::Error>;
    
    /// # Float Store Double Precision (RV32D)
    /// 
    /// Store a double-precision value from the floating-point registers to 
    /// memory.
    /// 
    /// `M[x[rs1] + sext(offset)] = f[rs2][63:0]`
    fn fsd(&mut self, rs1: Register, rs2: FloatRegister, offset: u32) 
        -> Result<T, Self::Error>;

    /// # Float Convert ? (RV64F)
    /// 
    /// ?
    /// 
    /// `x[rd] = s64_{f32}(f[rs1])`
    fn fcvt_l_s(&mut self, 
        rd: Register, rs1: FloatRegister, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Convert ? (RV64F)
    /// 
    /// ?
    /// 
    /// `x[rd] = s64_{f32}(f[rs1])`
    fn fcvt_lu_s(&mut self, 
        rd: FloatRegister, rs1: Register, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Convert ? (RV64F)
    /// 
    /// ?
    /// 
    /// `f[rd] = f32_{s64}(x[rs1])`
    fn fcvt_s_l(&mut self, 
        rd: FloatRegister, rs1: Register, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Convert ? (RV64F)
    /// 
    /// ?
    /// 
    /// `f[rd] = f32_{u64}(x[rs1])`
    fn fcvt_s_lu(&mut self, 
        rd: FloatRegister, rs1: Register, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Convert ? (RV64D)
    /// 
    /// ?
    /// 
    /// `x[rd] = s64_{f64}(f[rs1])`
    fn fcvt_l_d(&mut self, 
        rd: Register, rs1: FloatRegister, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Convert ? (RV64D)
    /// 
    /// ?
    /// 
    /// `x[rd] = u64_{f64}(f[rs1])`
    fn fcvt_lu_d(&mut self, 
        rd: Register, rs1: FloatRegister, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Move ? (RV64D)
    /// 
    /// ?
    /// 
    /// `x[rd] = f[rs1][63:0]`
    fn fmv_x_d(&mut self, 
        rd: Register, rs1: FloatRegister, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Convert ? (RV64D)
    /// 
    /// ?
    /// 
    /// `f[rd] = f64_{s64}(x[rs1])`
    fn fcvt_d_l(&mut self, 
        rd: FloatRegister, rs1: Register, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Convert ? (RV64D)
    /// 
    /// ?
    /// 
    /// `f[rd] = f64_{u64}(x[rs1])`
    fn fcvt_d_lu(&mut self, 
        rd: FloatRegister, rs1: Register, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;

    /// # Float Move ? (RV64D)
    /// 
    /// ?
    /// 
    /// `f[rd] = x[rs1][63:0]`
    fn fmv_d_x(&mut self, 
        rd: FloatRegister, rs1: Register, rm: FloatRoundingMode,
    ) -> Result<T, Self::Error>;
    
    /// # Compact Add Imm (RV32C)
    /// 
    /// Add a zero-extended non-zero immediate, scaled by 4, to the stack 
    /// pointer, x2, and writes the result to rd'.
    /// This instruction is used to generate pointers to stack-allocated 
    /// variables, and expands to addi rd', x2, nzuimm[9:2].
    /// 
    /// `x[8+rd'] = x[2] + uimm`
    /// Translated:
    /// `x[rd] = x[2] + uimm`
    fn c_addi4spn(&mut self, rd: Register, uimm: u16) 
        -> Result<T, Self::Error>;
    
    /// # Compact Float Load Double (RV64C)
    /// 
    /// Load a double-precision floating-point value from memory into 
    /// floating-point register rd'.
    /// It computes an effective address by adding the zero-extended offset, 
    /// scaled by 8, to the base address in register rs1'.
    /// 
    /// `f[8+rd'] = M[x[8+rs1'] + uimm][63:0]`
    /// Translated:
    /// `f[rd] = M[x[rs1] + uimm][63:0]`
    fn c_fld(&mut self, rd: Register, rs1: Register, imm: u16) 
        -> Result<T, Self::Error>;

    /// # Compact Load Word (RV32C)
    /// 
    /// Load a 32-bit value from memory into register rd'. It computes an 
    /// effective address by adding the zero-extended offset, scaled by 4, to 
    /// the base address in register rs1'.
    /// 
    /// `x[8+rd'] = sext(M[x[8+rs1'] + uimm][31:0])`
    /// Translated:
    /// `x[rd] = sext(M[x[rs1] + uimm][31:0])`
    fn c_lw(&mut self, rd: Register, rs1: Register, uimm: u16) 
        -> Result<T, Self::Error>;

    /// # Compact Float Load Word (RV32C)
    /// 
    /// Load a single-precision floating-point value from memory into 
    /// floating-point register rd'.
    /// It computes an effective address by adding the zero-extended offset, 
    /// scaled by 4, to the base address in register rs1'.
    /// 
    /// `f[8+rd'] = M[x[8+rs1'] + uimm][31:0]`
    /// Translated:
    /// `f[rd] = M[x[rs1] + uimm][31:0]`
    fn c_flw(&mut self, rd: FloatRegister, rs1: Register, uimm: u16) 
        -> Result<T, Self::Error>;

    /// # Compact Float Load Word (RV64C)
    /// 
    /// Load a 64-bit value from memory into register rd'.
    /// It computes an effective address by adding the zero-extended offset, 
    /// scaled by 8, to the base address in register rs1'.
    /// 
    /// `x[8+rd'] = M[x[8+rs1'] + uimm][63:0]`
    /// Translated:
    /// `x[rd] = M[x[rs1] + uimm][63:0]`
    fn c_ld(&mut self, rd: Register, rs1: FloatRegister, uimm: u16) 
        -> Result<T, Self::Error>;

    /// # Compact Float Store Double (RV64C)
    /// 
    /// Store a double-precision floating-point value in floating-point register 
    /// rs2' to memory.
    /// It computes an effective address by adding the zeroextended offset, 
    /// scaled by 8, to the base address in register rs1'.
    /// 
    /// `M[x[8+rs1'] + uimm][63:0] = f[8+rs2']`
    /// Translated:
    /// `M[x[rs1] + uimm][63:0] = f[rs2]`
    fn c_fsd(&mut self, rs1: Register, rs2: FloatRegister, uimm: u16) 
        -> Result<T, Self::Error>;

    /// # Compact Store Word (RV32C)
    /// 
    /// Store a 32-bit value in register rs2' to memory.
    /// It computes an effective address by adding the zero-extended offset, 
    /// scaled by 4, to the base address in register rs1'.
    /// 
    /// `M[x[8+rs1'] + uimm][31:0] = x[8+rs2']`
    /// Translated:
    /// `M[x[rs1] + uimm][31:0] = x[rs2]`
    fn c_sw(&mut self, rs1: Register, rs2: Register, uimm: u16) 
        -> Result<T, Self::Error>;
    
    /// # Compact Float Store Word (RV32C)
    /// 
    /// Store a single-precision floating-point value in floatingpoint register 
    /// rs2' to memory.
    /// It computes an effective address by adding the zero-extended offset, 
    /// scaled by 4, to the base address in register rs1'.
    /// 
    /// `M[x[8+rs1'] + uimm][31:0] = f[8+rs2']`
    /// Translated:
    /// `M[x[rs1] + uimm][31:0] = f[rs2]`
    fn c_fsw(&mut self, rs1: Register, rs2: FloatRegister, uimm: u8) 
        -> Result<T, Self::Error>;

    /// # Compact Store Double (RV64C)
    /// 
    /// Store a 64-bit value in register rs2' to memory.
    /// It computes an effective address by adding the zero-extended offset, 
    /// scaled by 8, to the base address in register rs1'.
    /// 
    /// `M[x[8+rs1'] + uimm][63:0] = x[8+rs2']`
    /// Translated:
    /// `M[x[rs1] + uimm][63:0] = x[rs2]`
    fn c_sd(&mut self, rs1: Register, rs2: FloatRegister, uimm: u16) 
        -> Result<T, Self::Error>;

    /// # Compact NOP (RV32C)
    /// 
    /// Does not change any user-visible state, except for advancing the pc.
    /// 
    fn c_nop(&mut self) -> Result<T, Self::Error>;

    /// # Compact Add Immediate (RV32C)
    /// 
    /// Add the non-zero sign-extended 6-bit immediate to the value in register 
    /// rd then writes the result to rd.
    /// 
    /// `x[rd] = x[rd] + sext(imm)`
    fn c_addi(&mut self, rd: Register, imm: i8) -> Result<T, Self::Error>;

    /// # Compact Jump and Link (RV32C)
    /// 
    /// Jump to address and place return address in rd.
    /// 
    /// `x[1] = pc+2; pc += sext(offset)`
    fn c_jal(&mut self, imm: u16) -> Result<T, Self::Error>;

    /// # Compact Add Immediate Word (RV64C)
    /// 
    /// Add the non-zero sign-extended 6-bit immediate to the value in register 
    /// rd then produce 32-bit result, then sign-extends result to 64 bits.
    /// 
    /// `x[rd] = sext((x[rd] + sext(imm))[31:0])`
    fn c_addiw(&mut self, rd: Register, imm: i8) -> Result<T, Self::Error>;

    /// # Compact Load Immediate (RV32C)
    /// 
    /// Load the sign-extended 6-bit immediate, imm, into register rd.
    /// C.LI is only valid when rd!=x0.
    /// 
    /// `x[rd] = sext(imm)`
    fn c_li(&mut self, rd: Register, imm: i8) -> Result<T, Self::Error>;

    /// # Compact ? (RV32C)
    /// 
    /// Add the non-zero sign-extended 6-bit immediate to the value in the stack 
    /// pointer (sp=x2), where the immediate is scaled to represent multiples of 
    /// 16 in the range (-512,496).
    /// 
    /// `x[2] = x[2] + sext(imm)`
    fn c_addi16sp(&mut self, imm: i8) -> Result<T, Self::Error>;

    /// # Compact Load Unsigned Immediate (RV32C)
    /// 
    /// ?
    /// 
    /// `x[rd] = sext(imm[17:12] << 12)`
    fn c_lui(&mut self, rd: Register, imm: i8) -> Result<T, Self::Error>;

    /// # Compact Shift Right Logical Immediate (RV32C)
    /// 
    /// Perform a logical right shift of the value in register rd' then writes 
    /// the result to rd'.
    /// The shift amount is encoded in the shamt field, where shamt[5] must be 
    /// zero for RV32C.
    /// 
    /// `x[8+rd'] = x[8+rd'] >>u uimm`
    /// Translated:
    /// `x[rd] = x[rd] >>u uimm`
    fn c_srli(&mut self, rd: Register, uimm: u8) -> Result<T, Self::Error>;

    /// # Compact Shift Right Arithmetical Immediate (RV32C)
    /// 
    /// Perform a arithmetic right shift of the value in register rd' then 
    /// writes the result to rd'.
    /// The shift amount is encoded in the shamt field, where shamt[5] must be 
    /// zero for RV32C.
    /// 
    /// `x[8+rd'] = x[8+rd'] >>s uimm`
    /// Translated:
    /// `x[rd] = x[rd] >>s uimm`
    fn c_srai(&mut self, rd: Register, uimm: u8) -> Result<T, Self::Error>;

    /// # Compact And Immediate (RV32C)
    /// 
    /// Compute the bitwise AND of of the value in register rd' and the 
    /// sign-extended 6-bit immediate, then writes the result to rd'.
    /// 
    /// `x[8+rd'] = x[8+rd'] & sext(imm)`
    /// Translated:
    /// `x[rd] = x[rd] & sext(imm)`
    fn c_andi(&mut self, rd: Register, uimm: u8) -> Result<T, Self::Error>;

    /// # Compact Sub (RV32C)
    /// 
    /// Subtract the value in register rs2' from the value in register rd', then 
    /// writes the result to register rd'.
    /// 
    /// `x[8+rd'] = x[8+rd'] - x[8+rs2']`
    /// Translated:
    /// `x[rd] = x[rd] - x[rs2]`
    fn c_sub(&mut self, rd: Register, rs2: Register) -> Result<T, Self::Error>;

    /// # Compact Xor (RV32C)
    /// 
    /// Compute the bitwise XOR of the values in registers rd' and rs2', then 
    /// writes the result to register rd'.
    /// 
    /// `x[8+rd'] = x[8+rd'] ^ x[8+rs2']`
    /// Translated:
    /// `x[rd] = x[rd] ^ x[rs2]`
    fn c_xor(&mut self, rd: Register, rs2: Register) -> Result<T, Self::Error>;

    /// # Compact Or (RV32C)
    /// 
    /// Compute the bitwise OR of the values in registers rd' and rs2', then 
    /// writes the result to register rd'.
    /// 
    /// `x[8+rd'] = x[8+rd'] | x[8+rs2']`
    /// Translated:
    /// `x[rd] = x[rd] | x[rs2]`
    fn c_or(&mut self, rd: Register, rs2: Register) -> Result<T, Self::Error>;

    /// # Compact And (RV32C)
    /// 
    /// Compute the bitwise AND of the values in registers rd' and rs2', then 
    /// writes the result to register rd'.
    /// 
    /// `x[8+rd'] = x[8+rd'] & x[8+rs2']`
    /// Translated:
    /// `x[rd] = x[rd] & x[rs2]`
    fn c_and(&mut self, rd: Register, rs2: Register) -> Result<T, Self::Error>;

    /// # Compact Sub Word (RV64C)
    /// 
    /// Subtract the value in register rs2' from the value in register rd', then 
    /// sign-extends the lower 32 bits of the difference before writing the 
    /// result to register rd'.
    /// 
    /// `x[8+rd'] = sext((x[8+rd'] - x[8+rs2'])[31:0])`
    /// Translated:
    /// `x[rd] = sext((x[rd] - x[rs2])[31:0])`
    fn c_subw(&mut self, rd: Register, rs2: Register) -> Result<T, Self::Error>;

    /// # Compact Add Word (RV64C)
    /// 
    /// Add the value in register rs2' from the value in register rd', then 
    /// sign-extends the lower 32 bits of the difference before writing the 
    /// result to register rd'.
    /// 
    /// `x[8+rd'] = sext((x[8+rd'] + x[8+rs2'])[31:0])`
    /// Translated:
    /// `x[rd] = sext((x[rd] + x[rs2])[31:0])`
    fn c_addw(&mut self, rd: Register, rs2: Register) -> Result<T, Self::Error>;

    /// # Compact Jump (RV32C)
    /// 
    /// Unconditional control transfer.
    /// 
    /// `pc += sext(offset)`
    fn c_j(&mut self, imm: u16) -> Result<T, Self::Error>;

    /// # Compact Jump (RV64C)
    /// 
    /// Take the branch if the value in register rs1' is zero.
    /// 
    /// `if (x[8+rs1'] == 0) pc += sext(offset)`
    /// Translated:
    /// `if (x[rs1] == 0) pc += sext(offset)`
    fn c_beqz(&mut self, rs1: Register, offset: u16) -> Result<T, Self::Error>;

    /// # Compact Jump (RV64C)
    /// 
    /// Take the branch if the value in register rs1' is not zero.
    /// 
    /// `if (x[8+rs1'] != 0) pc += sext(offset)`
    /// Translated:
    /// `if (x[rs1] != 0) pc += sext(offset)`
    fn c_bnez(&mut self, rs1: Register, offset: u16) -> Result<T, Self::Error>;

    /// # Compact Shift Left Logical Immediate (RV32C)
    /// 
    /// Perform a logical left shift of the value in register rd then writes the 
    /// result to rd.
    /// The shift amount is encoded in the shamt field, where shamt[5] must be 
    /// zero for RV32C.
    /// 
    /// `x[rd] = x[rd] << uimm`
    fn c_slli(&mut self, rd: Register, uimm: u8) -> Result<T, Self::Error>;

    /// # Compact Float Load Double from Stack (RV32C)
    /// 
    /// Load a double-precision floating-point value from memory into 
    /// floating-point register rd.
    /// It computes its effective address by adding the zero-extended offset, 
    /// scaled by 8, to the stack pointer, x2.
    /// 
    /// `f[rd] = M[x[2] + uimm][63:0]`
    fn c_fldsp(&mut self, rd: FloatRegister, uimm: u8) -> Result<T, Self::Error>;

    /// # Compact Load Word from Stack (RV32C)
    /// 
    /// Load a 32-bit value from memory into register rd. It computes an 
    /// effective address by adding the zero-extended offset, scaled by 4, to 
    /// the stack pointer, x2.
    /// 
    /// `x[rd] = sext(M[x[2] + uimm][31:0])`
    fn c_lwsp(&mut self, rd: Register, uimm: u8) -> Result<T, Self::Error>;

    /// # Compact Float Load Word from Stack (RV32C)
    /// 
    /// Load a single-precision floating-point value from memory into 
    /// floating-point register rd.
    /// It computes its effective address by adding the zero-extended offset, 
    /// scaled by 4, to the stack pointer, x2.
    /// 
    /// `f[rd] = M[x[2] + uimm][31:0]`
    fn c_flwsp(&mut self, rd: Register, uimm: u8) -> Result<T, Self::Error>;

    /// # Compact Load Double Word from Stack (RV64C)
    /// 
    /// Load a 64-bit value from memory into register rd.
    /// It computes its effective address by adding the zero-extended offset, 
    /// scaled by 8, to the stack pointer, x2.
    /// 
    /// `x[rd] = M[x[2] + uimm][63:0]`
    fn c_ldsp(&mut self, rd: Register, uimm: u8) -> Result<T, Self::Error>;

    /// # Compact Jump Register (RV32C)
    /// 
    /// Performs an unconditional control transfer to the address in register 
    /// rs1.
    /// 
    /// `pc = x[rs1]`
    fn c_jr(&mut self, rs1: Register) -> Result<T, Self::Error>;

    /// # Compact Move (RV32C)
    /// 
    /// Copy the value in register rs2 into register rd.
    /// 
    /// `x[rd] = x[rs2]`
    fn c_mv(&mut self, rs1: Register, rs2: Register) -> Result<T, Self::Error>;

    /// # Compact Ebreak (RV32C)
    /// 
    /// Cause control to be transferred back to the debugging environment.
    fn c_ebreak(&mut self) -> Result<T, Self::Error>;

    /// # Compact Jump And Link Register (RV32C)
    /// 
    /// Jump to address and place return address in rd.
    /// 
    /// `t = pc+2; pc = x[rs1]; x[1] = t`
    fn c_jalr(&mut self, rs1: Register) -> Result<T, Self::Error>;

    /// # Compact Add (RV32C)
    /// 
    /// Add the values in registers rd and rs2 and writes the result to register rd.
    /// 
    /// `x[rd] = x[rd] + x[rs2]`
    fn c_add(&mut self, rd: Register, rs2: Register) -> Result<T, Self::Error>;

    /// # Compact Float Store Double from Stack (RV32C)
    /// 
    /// Store a double-precision floating-point value in floating-point register 
    /// rs2 to memory.
    /// It computes an effective address by adding the zeroextended offset, 
    /// scaled by 8, to the stack pointer, x2.
    /// 
    /// `M[x[2] + uimm][63:0] = f[rs2]`
    fn c_fsdsp(&mut self, rd: Register, rs2: Register) -> Result<T, Self::Error>;

    /// # Compact Store Word Stack (RV32C)
    /// 
    /// Store a 32-bit value in register rs2 to memory.
    /// It computes an effective address by adding the zero-extended offset, 
    /// scaled by 4, to the stack pointer, x2.
    /// 
    /// `M[x[2] + uimm][31:0] = x[rs2]`
    fn c_swsp(&mut self, rs2: Register, uimm: u8) -> Result<T, Self::Error>;

    /// # Compact Float Store Word Stack (RV32C)
    /// 
    /// Store a single-precision floating-point value in floating-point register 
    /// rs2 to memory.
    /// It computes an effective address by adding the zero-extended offset, 
    /// scaled by 4, to the stack pointer, x2.
    /// 
    /// `M[x[2] + uimm][31:0] = f[rs2]`
    fn c_fswsp(&mut self, rs2: FloatRegister, uimm: u8) -> Result<T, Self::Error>;

    /// # Compact Store Dobule Word Stack (RV64C)
    /// 
    /// Store a 64-bit value in register rs2 to memory.
    /// It computes an effective address by adding the zero-extended offset, 
    /// scaled by 8, to the stack pointer, x2.
    /// 
    /// `M[x[2] + uimm][63:0] = x[rs2]`
    fn c_sdsp(&mut self, rs2: FloatRegister, uimm: u8) -> Result<T, Self::Error>;
}

/// Disassemble a u32 instruction and call the visitor `User` with the relative
/// Instruction
pub fn diss_riscv64gc<T, User: RV64GUser<T>>(user: &mut User, inst: u32) 
    -> Result<T, User::Error> {

    // decode the instruction length prefix
    match inst & 0b11 {
        // 4 bytes instruction
        0b11 => diss_riscv64gc_4b_inst(user, inst),
        // 2 bytes instruction (Compact) quadrant 0
        0b00 => {
            let inst = inst as u16;
            let funct3 = (inst >> 12) & 0b111;
            match funct3 {
                0b000 => {
                    let CIWtype{
                        funct3, imm, rd_prime
                    } = CIWtype::from(inst);
                    if imm == 0 {
                        panic!("Illegal instruction");
                    }
                    user.c_addi4spn(Register::from_prime(rd_prime), imm)
                }
                0b001 => {
                    let CLtype {
                        funct3, imm2, rs1_prime, imm1, rd_prime,
                    } = CLtype::from(inst);
                    user.c_fld(
                        Register::from_prime(rd_prime),
                        Register::from_prime(rs1_prime),
                        compose_imms_53_76(imm1, imm2),
                    )
                }
                0b010 => {
                    let CLtype {
                        funct3, imm2, rs1_prime, imm1, rd_prime,
                    } = CLtype::from(inst);
                    user.c_lw(
                        Register::from_prime(rd_prime),
                        Register::from_prime(rs1_prime),
                        compose_imms_53_2_or_6(imm1, imm2),
                    )
                }
                0b011 => {
                    let CLtype {
                        funct3, imm2, rs1_prime, imm1, rd_prime,
                    } = CLtype::from(inst);
                    user.c_ld(
                        Register::from_prime(rd_prime),
                        FloatRegister::from_prime(rs1_prime),
                        compose_imms_53_76(imm1, imm2),
                    )
                }
                0b100 => unimplemented!("Reserved Compact Instruction"),
                0b101 => {
                    let CStype {
                        funct3, imm2, rs1_prime, imm1, rs2_prime,
                    } = CStype::from(inst);
                    user.c_fsd(
                        Register::from_prime(rs1_prime),
                        FloatRegister::from_prime(rs2_prime),
                        compose_imms_53_76(imm1, imm2),
                    )
                }
                0b110 => {
                    let CStype {
                        funct3, imm2, rs1_prime, imm1, rs2_prime,
                    } = CStype::from(inst);
                    user.c_sw(
                        Register::from_prime(rs1_prime),
                        Register::from_prime(rs2_prime),
                        compose_imms_53_2_or_6(imm1, imm2),
                    )
                }
                0b111 => {
                    let CStype {
                        funct3, imm2, rs1_prime, imm1, rs2_prime,
                    } = CStype::from(inst);
                    user.c_sd(
                        Register::from_prime(rs1_prime),
                        FloatRegister::from_prime(rs2_prime),
                        compose_imms_53_2_or_6(imm1, imm2),
                    )
                }
                _ => unreachable!(),
            }
        },
        // 2 bytes instruction (Compact) quadrant 1
        0b01 => {
            let inst = inst as u16;
            let funct3 = (inst >> 12) & 0b111;
            unimplemented!("TODO")
        },
        // 2 bytes instruction (Compact) quadrant 2
        0b10 => {
            let inst = inst as u16;
            let funct3 = (inst >> 12) & 0b111;
            unimplemented!("TODO")
        },
        _ => unreachable!(),
    }
}

fn diss_riscv64gc_4b_inst<T, User: RV64GUser<T>>(user: &mut User, inst: u32) 
    -> Result<T, User::Error> {
    // Extract the opcode from the instruction
    let opcode = inst & 0b1111111;

    match opcode {
        0b0110111 => {
            let Utype{imm, rd} = Utype::from(inst);
            user.lui(rd.into(), imm)
        }
        0b0010111 => {
            let Utype{imm, rd} = Utype::from(inst);
            user.auipc(rd.into(), imm as i64 as u64)
        }
        0b1101111 => {
            let Jtype{imm, rd} = Jtype::from(inst);
            user.jal(imm as i64 as u64)
        }
        0b1100111 => {
            let Itype{
                imm, rs1, funct3, rd,
            } = Itype::from(inst);
            match funct3 {
                0b000 => {
                    user.jalr(rd.into(), imm as i64 as u64)
                }
                _ => unimplemented!("Unexpected 0b1100111"),
            }
        }
        0b1100011 => {
            let Btype {
                imm, rs2, rs1, funct3,
            } = Btype::from(inst);
            match funct3 {
                0b000 => user.beq( rs1.into(), rs2.into(), imm as i64 as u64),
                0b001 => user.bne( rs1.into(), rs2.into(), imm as i64 as u64),
                0b100 => user.blt( rs1.into(), rs2.into(), imm as i64 as u64),
                0b101 => user.bge( rs1.into(), rs2.into(), imm as i64 as u64),
                0b110 => user.bltu(rs1.into(), rs2.into(), imm as i64 as u64),
                0b111 => user.bgeu(rs1.into(), rs2.into(), imm as i64 as u64),
                _ => unimplemented!("Unexpected 0b1100011"),
            }
        }
        0b0000111 => {
            let Itype{
                imm, rs1, funct3, rd,
            } = Itype::from(inst);

            match funct3 {
                0b010 => user.flw(rd.into(), rs1.into(), imm),
                0b011 => user.fld(rd.into(), rs1.into(), imm),
                _ => unimplemented!("Unexpected 0b0000111"),
            }
        }
        0b0100111 => {
            let Stype {
                imm, rs2, rs1, funct3,
            } = Stype::from(inst);

            match funct3 {
                0b010 => user.fsw(rs1.into(), rs2.into(), imm),
                0b011 => user.fsd(rs1.into(), rs2.into(), imm),
                _ => unimplemented!("Unexpected 0b0000111"),
            }
        }
        0b1000011 => {
            let R4type {
                funct2, rs3, rs2, rs1, funct3, rd,
            } = R4type::from(inst);
            match funct2 {
                00 => user.fmadd_s(
                    rd.into(), rs1.into(), rs2.into(), 
                    rs3.into(), funct3.into(),
                ),
                01 => user.fmadd_d(
                    rd.into(), rs1.into(), rs2.into(), 
                    rs3.into(), funct3.into(),
                ),
                _ => unimplemented!("Unexpected 0b1000011"),
            }
        }
        0b1000111 => {
            let R4type {
                funct2, rs3, rs2, rs1, funct3, rd,
            } = R4type::from(inst);
            match funct2 {
                00 => user.fmsub_s(
                    rd.into(), rs1.into(), rs2.into(), 
                    rs3.into(), funct3.into(),
                ),
                01 => user.fmsub_d(
                    rd.into(), rs1.into(), rs2.into(), 
                    rs3.into(), funct3.into(),
                ),
                _ => unimplemented!("Unexpected 0b1000111"),
            }
        }
        0b1001011 => {
            let R4type {
                funct2, rs3, rs2, rs1, funct3, rd,
            } = R4type::from(inst);
            match funct2 {
                00 => user.fnmsub_s(
                    rd.into(), rs1.into(), rs2.into(),
                    rs3.into(), funct3.into(),
                ),
                01 => user.fnmsub_d(
                    rd.into(), rs1.into(), rs2.into(),
                    rs3.into(), funct3.into(),
                ),
                _ => unimplemented!("Unexpected 0b1001011"),
            }
        }
        0b1001111 => {
            let R4type {
                funct2, rs3, rs2, rs1, funct3, rd,
            } = R4type::from(inst);
            match funct2 {
                00 => user.fnmadd_s(
                    rd.into(), rs1.into(), rs2.into(), 
                    rs3.into(), funct3.into(),
                ),
                01 => user.fnmadd_d(
                    rd.into(), rs1.into(), rs2.into(), 
                    rs3.into(), funct3.into(),
                ),
                _ => unimplemented!("Unexpected 0b1001111"),
            }
        }
        0b1010011 => {
            let Rtype{
                funct7, rs2, rs1, funct3, rd,
            } = Rtype::from(inst);
            match funct7 {
                0b0000000 => user.fadd_s(
                    rd.into(), rs1.into(), 
                    rs2.into(), funct3.into(),
                ),
                0b0000001 => user.fadd_d(
                    rd.into(), rs1.into(), 
                    rs2.into(), funct3.into(),
                ),
                0b0000100 => user.fsub_s(
                    rd.into(), rs1.into(), 
                    rs2.into(), funct3.into(),
                ),
                0b0000100 => user.fsub_d(
                    rd.into(), rs1.into(), 
                    rs2.into(), funct3.into(),
                ),
                0b0001000 => user.fmul_s(
                    rd.into(), rs1.into(), 
                    rs2.into(), funct3.into(),
                ),
                0b0001001 => user.fmul_d(
                    rd.into(), rs1.into(), 
                    rs2.into(), funct3.into(),
                ),
                0b0001100 => user.fdiv_s(
                    rd.into(), rs1.into(), 
                    rs2.into(), funct3.into(),
                ),
                0b0001101 => user.fdiv_d(
                    rd.into(), rs1.into(), 
                    rs2.into(), funct3.into(),
                ),
                0b0101100 => {
                    assert_eq!(rs2, 0b00000);
                    user.fsqrt_s(
                        rd.into(), rs1.into(), 
                        funct3.into(),
                    )
                },
                0b0101101 => {
                    assert_eq!(rs2, 0b00000);
                    user.fsqrt_d(
                        rd.into(), rs1.into(), 
                        funct3.into(),
                    )
                },
                0b0010000 => {
                    match funct3 {
                        0b000 => user.fsgnj_s(
                            rd.into(), rs1.into(), rs2.into()
                        ),
                        0b001 => user.fsgnjn_s(
                            rd.into(), rs1.into(), rs2.into()
                        ),
                        0b010 => user.fsgnjx_s(
                            rd.into(), rs1.into(), rs2.into()
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b0010000"),
                    }
                }
                0b0010001 => {
                    match funct3 {
                        0b000 => user.fsgnj_d(
                            rd.into(), rs1.into(), rs2.into()
                        ),
                        0b001 => user.fsgnjn_d(
                            rd.into(), rs1.into(), rs2.into()
                        ),
                        0b010 => user.fsgnjx_d(
                            rd.into(), rs1.into(), rs2.into()
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b0010001"),
                    }
                }
                0b0010100 => {
                    match funct3 {
                        0b000 => user.fmin_s(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        0b001 => user.fmax_s(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b0010100"),
                    }
                }
                0b0010101 => {
                    match funct3 {
                        0b000 => user.fmin_d(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        0b001 => user.fmax_d(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b0010100"),
                    }
                }
                0b0100000 => {
                    match rs2 {
                        0b00001 => user.fcvt_s_d(
                            rd.into(), rs1.into()
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b0100000"),
                    }
                }
                0b0100001 => {
                    match rs2 {
                        0b00000 => user.fcvt_d_s(
                            rd.into(), rs1.into()
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b0100001"),
                    }
                }
                0b1100000 => {
                    match rs2 {
                        0b00000 => user.fcvt_w_s(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00001 => user.fcvt_wu_s(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00010 => user.fcvt_l_s(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00011 => user.fcvt_lu_s(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1100000"),
                    }
                }
                0b1100001 => {
                    match rs2 {
                        0b00000 => user.fcvt_w_d(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00001 => user.fcvt_wu_d(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00010 => user.fcvt_l_d(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00011 => user.fcvt_lu_d(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1100000"),
                    }
                }
                0b1110000 => {
                    match (rs2, funct3) {
                        (0b00000, 0b000) => user.fmv_x_w(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        (0b00000, 0b001) => user.fclass_s(
                            rd.into(), rs1.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1110000"),
                    }
                }
                0b1110001 => {
                    match (rs2, funct3) {
                        (0b00000, 0b000) => user.fmv_x_d(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        (0b00000, 0b001) => user.fclass_d(
                            rd.into(), rs1.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1110001"),
                    }
                }
                0b1010000 => {
                    match funct3 {
                        0b010 => user.feq_s(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        0b001 => user.flt_s(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        0b000 => user.fle_s(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1010000"),
                    }
                }
                0b1010001 => {
                    match funct3 {
                        0b010 => user.feq_d(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        0b001 => user.flt_d(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        0b000 => user.fle_d(
                            rd.into(), rs1.into(), rs2.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1010000"),
                    }
                }
                0b1101000 => {
                    match rs2 {
                        0b00000 => user.fcvt_s_w(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00001 => user.fcvt_s_wu(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00010 => user.fcvt_s_l(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00011 => user.fcvt_s_lu(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1101000"),
                    }
                }
                0b1101001 => {
                    match rs2 {
                        0b00000 => user.fcvt_d_w(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00001 => user.fcvt_d_wu(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00010 => user.fcvt_d_l(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        0b00011 => user.fcvt_d_lu(
                            rd.into(), rs1.into(),
                            funct3.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1101000"),
                    }
                }
                0b1111000 => {
                    match (rs2, funct3) {
                        (0b00000, 0b000) => user.fmv_w_x(
                            rd.into(), rs1.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1111000"),
                    }
                }
                0b1111001 => {
                    match (rs2, funct3) {
                        (0b00000, 0b000) => user.fmv_d_x(
                            rd.into(), rs1.into(), funct3.into(),
                        ),
                        _ => unimplemented!("Unexpected 0b1010011 - 0b1111001"),
                    }
                }
            }
        }
        0b0000011 => {
            let Itype{
                imm, rs1, funct3, rd,
            } = Itype::from(inst);

            match funct3 {
                0b000 => user.lb( rd.into(), imm as i64 as u64),
                0b001 => user.lh( rd.into(), imm as i64 as u64),
                0b010 => user.lw( rd.into(), imm as i64 as u64),
                0b011 => user.ld( rd.into(), imm as i64 as u64),
                0b100 => user.lbu(rd.into(), imm as i64 as u64),
                0b101 => user.lhu(rd.into(), imm as i64 as u64),
                0b110 => user.lwu(rd.into(), imm as i64 as u64),
                _ => unimplemented!("Unexpected 0b0000011"),
            }
        }
        0b0100011 => {
            let Stype {
                imm, rs2, rs1, funct3,
            } = Stype::from(inst);

            match funct3 {
                0b000 => user.sb(rs1.into(), rs2.into(), imm as i64 as u64),
                0b001 => user.sh(rs1.into(), rs2.into(), imm as i64 as u64),
                0b010 => user.sw(rs1.into(), rs2.into(), imm as i64 as u64),
                0b011 => user.sd(rs1.into(), rs2.into(), imm as i64 as u64),
                _ => unimplemented!("Unexpected 0b0100011"),
            }
        }
        0b0010011 => {
            let Itype{
                imm, rs1, funct3, rd,
            } = Itype::from(inst);

            match funct3 {
                0b000 => user.addi( rd.into(), rs1.into(), imm as i64 as u64),
                0b010 => user.slti( rd.into(), rs1.into(), imm as i64 as u64),
                0b011 => user.sltiu(rd.into(), rs1.into(), imm as i64 as u64),
                0b100 => user.xori( rd.into(), rs1.into(), imm as i64 as u64),
                0b110 => user.ori(  rd.into(), rs1.into(), imm as i64 as u64),
                0b111 => user.andi( rd.into(), rs1.into(), imm as i64 as u64),
                0b001 => {
                    let mode = (imm >> 6) & 0b111111;
                    let shamt = imm & 0b111111;
                    
                    match mode {
                        0b000000 => user.slli(rd.into(), rs1.into(), shamt),
                        _ => unreachable!(),
                    }
                }
                0b101 => {
                    let mode = (imm >> 6) & 0b111111;
                    let shamt = imm & 0b111111;
                    
                    match mode {
                        0b000000 => user.srli(rd.into(), rs1.into(), shamt),
                        0b010000 => user.srai(rd.into(), rs1.into(), shamt),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        0b0110011 => {
            let Rtype{
                funct7, rs2, rs1, funct3, rd,
            } = Rtype::from(inst);

            match (funct7, funct3) {
                (0b0000000, 0b000) => user.add(   rd.into(), rs1.into(), rs2.into()),
                (0b0100000, 0b000) => user.sub(   rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b001) => user.sll(   rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b010) => user.slt(   rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b011) => user.sltu(  rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b100) => user.xor(   rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b101) => user.srl(   rd.into(), rs1.into(), rs2.into()),
                (0b0100000, 0b101) => user.sra(   rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b110) => user.or(    rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b111) => user.and(   rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b000) => user.mul(   rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b001) => user.mulh(  rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b010) => user.mulhsu(rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b011) => user.mulhu( rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b100) => user.div(   rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b101) => user.divu(  rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b110) => user.rem(   rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b111) => user.remu(  rd.into(), rs1.into(), rs2.into()),
                _ => unreachable!(),
            }
        }
        0b0111011 => {
            let Rtype{
                funct7, rs2, rs1, funct3, rd,
            } = Rtype::from(inst);

            match (funct7, funct3) {
                (0b0000000, 0b000) => user.addw( rd.into(), rs1.into(), rs2.into()),
                (0b0100000, 0b000) => user.subw( rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b001) => user.sllw( rd.into(), rs1.into(), rs2.into()),
                (0b0000000, 0b101) => user.srlw( rd.into(), rs1.into(), rs2.into()),
                (0b0100000, 0b101) => user.sraw( rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b000) => user.mulw( rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b100) => user.divw( rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b101) => user.divuw(rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b110) => user.remw( rd.into(), rs1.into(), rs2.into()),
                (0b0000001, 0b111) => user.remuw(rd.into(), rs1.into(), rs2.into()),
                _ => unreachable!(),
            }
        }
        0b0001111 => {
            let Itype{
                imm, rs1, funct3, rd,
            } = Itype::from(inst);

            match funct3 {
                0b000 => user.fence(),
                0b001 => user.fence_i(),
                _ => unreachable!(),
            }
        }
        0b1110011 => {
            let Itype{
                imm, rs1, funct3, rd,
            } = Itype::from(inst);

            match funct3 {
                0b000 => {
                    if inst == 0b00000000000000000000000001110011 {
                        user.ecall()
                    } else if inst == 0b00000000000100000000000001110011 {
                        user.ebreak()
                    } else {
                        unreachable!();
                    }
                }
                0b001 => user.csrrw( rd.into(), rs1.into(), imm),
                0b010 => user.csrrs( rd.into(), rs1.into(), imm),
                0b011 => user.csrrc( rd.into(), rs1.into(), imm),
                0b101 => user.csrrwi(rd.into(), rs1.into(), imm),
                0b110 => user.csrrsi(rd.into(), rs1.into(), imm),
                0b111 => user.csrrci(rd.into(), rs1.into(), imm),
            }

        }
        0b0011011 => {
            let Itype{
                imm, rs1, funct3, rd,
            } = Itype::from(inst);

            match funct3 {
                0b000 => user.addiw(rd.into(), rs1.into(), imm as u32),
                0b001 => {
                    let mode = (imm >> 5) & 0b1111111;
                    let shamt = imm & 0b11111;
                    
                    match mode {
                        0b0000000 => user.slliw(rd.into(), rs1.into(), shamt),
                        _ => unreachable!(),
                    }
                }
                0b101 => {
                    let mode = (imm >> 5) & 0b1111111;
                    let shamt = imm & 0b11111;

                    match mode {
                        0b0000000 => user.srliw(rd.into(), rs1.into(), shamt),
                        0b0100000 => user.sraiw(rd.into(), rs1.into(), shamt),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unimplemented!("Unhandled opcode {:#09b}\n", opcode),
    }
}