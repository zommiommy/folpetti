use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegA64 {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    Fp, // X29
    Lr, // x30
}

impl From<u32> for RegA64 {
    fn from(value: u32) -> Self {
        use RegA64::*;
        match value { 
            0 => X0,
            1 => X1,
            2 => X2,
            3 => X3,
            4 => X4,
            5 => X5,
            6 => X6,
            7 => X7,
            8 => X8,
            9 => X9,
           10 => X10,
           11 => X11,
           12 => X12,
           13 => X13,
           14 => X14,
           15 => X15,
           16 => X16,
           17 => X17,
           18 => X18,
           19 => X19,
           20 => X20,
           21 => X21,
           22 => X22,
           23 => X23,
           24 => X24,
           25 => X25,
           26 => X26,
           27 => X27,
           28 => X28,
           29 => Fp,
           30 => Lr,
           _ => {
               panic!("invalid register decoding {}", value);
           }
        }
    }
}

pub enum ErrorDissArmV8aA64<E> {
    UnimplementedInstruction(u32),
    UnallocatedInstruction(u32),
    UserError(E),
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Conditional menemonics for instructions.
/// From the [reference manual](https://developer.arm.com/documentation/ddi0406/cb/Application-Level-Architecture/Instruction-Details/Conditional-execution?lang=en#Chdcgdjb)
pub enum Cond {
    /// Equal, Z == 1
    Eq = 0b0000,
    /// Not equal, Z == 0
    Ne = 0b0001,
    /// Carry set (also called Hs), C == 1
    Cs = 0b0010,
    /// Carry clear (also called Lo), C == 0
    Cc = 0b0011,
    /// Minus, negative, N == 1
    Mi = 0b0100,
    /// Plus positive or zero, N == 0
    Pl = 0b0101,
    /// Overflow, V == 1
    Vs = 0b0110,
    /// No overflow, V == 0
    Vc = 0b0111,
    /// Unsigned higher, C == 1 and Z == 0
    Hi = 0b1000,
    /// Unsigned lower or same, C == 0 or Z == 1
    Ls = 0b1001,
    /// Signed Greater than or equal, N == V
    Ge = 0b1010,
    /// Signed less than, N != V
    Lt = 0b1011,
    /// Signed grater than, Z == 0 and N == V
    Gt = 0b1100,
    /// Signed less than or equal, Z == 1 or N != V
    Le = 0b1101,
    /// None, Always, unconditional
    Al = 0b1110,
    /// Instruction that can only be executed unconditionally
    Unconditional = 0b1111,
}

impl From<u32> for Cond {
    fn from(value: u32) -> Cond {
        match value {
             0 => Cond::Eq,
             1 => Cond::Ne,
             2 => Cond::Cs,
             3 => Cond::Cc,
             4 => Cond::Mi,
             5 => Cond::Pl,
             6 => Cond::Vs,
             7 => Cond::Vc,
             8 => Cond::Hi,
             9 => Cond::Ls,
            10 => Cond::Ge,
            11 => Cond::Lt,
            12 => Cond::Gt,
            13 => Cond::Le,
            14 => Cond::Al,
            15 => Cond::Unconditional,
            _ => panic!("Invalid condition value {}", value),
        }
    }
}

pub trait ArmV8aA64User {
    type Error;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/ADR--Form-PC-relative-address-?lang=en)
    /// Form PC-relative address adds an immediate value to the PC value to form 
    /// a PC-relative address, and writes the result to the destination 
    /// register.
    fn adr(&mut self, rd: RegA64, imm: u32) -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/ADRP--Form-PC-relative-address-to-4KB-page-?lang=en)
    /// Form PC-relative address to 4KB page adds an immediate value that is 
    /// shifted left by 12 bits, to the PC value to form a PC-relative address, 
    /// with the bottom 12 bits masked out, and writes the result to the 
    /// destination register.
    fn adrp(&mut self, rd: RegA64, imm: u32) -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/ADD--immediate---Add--immediate--?lang=en)
    /// Add (immediate) adds a register value and an optionally-shifted 
    /// immediate value, and writes the result to the destination register.
    /// This instruction is used by the alias MOV (to/from SP).
    fn add_imm_32(&mut self, rd: RegA64, rn: RegA64, imm: u32) 
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::add_imm_32`]
    fn add_imm_64(&mut self, rd: RegA64, rn: RegA64, imm: u32) 
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/ADDS--immediate---Add--immediate---setting-flags-?lang=en)
    /// Add (immediate), setting flags, adds a register value and an 
    /// optionally-shifted immediate value, and writes the result to the 
    /// destination register. It updates the condition flags based on the 
    /// result.
    /// This instruction is used by the alias CMN (immediate).
    fn adds_imm_32(&mut self, rd: RegA64, rn: RegA64, imm: u32) 
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::adds_imm_32`]
    fn adds_imm_64(&mut self, rd: RegA64, rn: RegA64, imm: u32) 
        -> Result<(), Self::Error>;
    
    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/SUB--immediate---Subtract--immediate--?lang=en)
    /// Subtract (immediate) subtracts an optionally-shifted immediate value 
    /// from a register value, and writes the result to the destination 
    /// register.
    fn sub_imm_32(&mut self, rd: RegA64, rn: RegA64, imm: u32) 
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::sub_imm_32`]
    fn sub_imm_64(&mut self, rd: RegA64, rn: RegA64, imm: u32) 
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/SUBS--immediate---Subtract--immediate---setting-flags-?lang=en)
    /// Subtract (immediate), setting flags, subtracts an optionally-shifted 
    /// immediate value from a register value, and writes the result to the 
    /// destination register. It updates the condition flags based on the 
    /// result.
    /// This instruction is used by the alias CMP (immediate).
    fn subs_imm_32(&mut self, rd: RegA64, rn: RegA64, imm: u32) 
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::subs_imm_32`]
    fn subs_imm_64(&mut self, rd: RegA64, rn: RegA64, imm: u32) 
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/AND--immediate---Bitwise-AND--immediate--?lang=en)
    /// Bitwise AND (immediate) performs a bitwise AND of a register value and 
    /// an immediate value, and writes the result to the destination register.
    fn and_imm_32(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::and_imm_32`]
    fn and_imm_64(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/ORR--immediate---Bitwise-OR--immediate--?lang=en)
    /// Bitwise OR (immediate) performs a bitwise (inclusive) OR of a register value and an immediate register value, and writes the result to the destination register.
    /// This instruction is used by the alias MOV (bitmask immediate).
    fn orr_imm_32(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::orr_imm_32`]
    fn orr_imm_64(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/EOR--immediate---Bitwise-Exclusive-OR--immediate--?lang=en)
    /// Bitwise Exclusive OR (immediate) performs a bitwise Exclusive OR of a 
    /// register value and an immediate value, and writes the result to the 
    /// destination register.
    fn eor_imm_32(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::eor_imm_32`]
    fn eor_imm_64(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/ANDS--immediate---Bitwise-AND--immediate---setting-flags-?lang=en)
    /// Bitwise AND (immediate), setting flags, performs a bitwise AND of a 
    /// register value and an immediate value, and writes the result to the
    ///  destination register. It updates the condition flags based on the 
    /// result. This instruction is used by the alias TST (immediate).
    fn ands_imm_32(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::ands_imm_32`]
    fn ands_imm_64(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/MOVN--Move-wide-with-NOT-?lang=en)
    /// Move wide with NOT moves the inverse of an optionally-shifted 16-bit 
    /// immediate value to a register.
    /// This instruction is used by the alias MOV (inverted wide immediate).
    fn movn_32(&mut self, rd: RegA64, imm16: u32, hw: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::movn_32`]
    fn movn_64(&mut self, rd: RegA64, imm16: u32, hw: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/MOVZ--Move-wide-with-zero-?lang=en)
    /// Move wide with zero moves an optionally-shifted 16-bit immediate value 
    /// to a register. This instruction is used by the alias MOV 
    /// (wide immediate).
    fn movz_32(&mut self, rd: RegA64, imm16: u32, hw: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::movz_32`]
    fn movz_64(&mut self, rd: RegA64, imm16: u32, hw: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/MOVK--Move-wide-with-keep-?lang=en)
    /// Move wide with keep moves an optionally-shifted 16-bit immediate value 
    /// into a register, keeping other bits unchanged.
    fn movk_32(&mut self, rd: RegA64, imm16: u32, hw: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::movk_32`]
    fn movk_64(&mut self, rd: RegA64, imm16: u32, hw: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/SBFM--Signed-Bitfield-Move-?lang=en)
    /// Signed Bitfield Move is usually accessed via one of its aliases, which 
    /// are always preferred for disassembly.
    /// 
    /// If <imms> is greater than or equal to <immr>, this copies a bitfield of 
    /// (<imms>-<immr>+1) bits starting from bit position <immr> in the source 
    /// register to the least significant bits of the destination register.
    ///
    /// If <imms> is less than <immr>, this copies a bitfield of (<imms>+1) bits 
    /// from the least significant bits of the source register to bit position 
    /// (regsize-<immr>) of the destination register, where regsize is the 
    /// destination register size of 32 or 64 bits.
    /// 
    /// In both cases the destination bits below the bitfield are set to zero, 
    /// and the bits above the bitfield are set to a copy of the most 
    /// significant bit of the bitfield.
    /// 
    /// This instruction is used by the aliases ASR (immediate), SBFIZ, SBFX, 
    /// SXTB, SXTH, and SXTW.
    fn sbfm_32(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::sbfm_32`]
    fn sbfm_64(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/BFM--Bitfield-Move-?lang=en)
    /// Bitfield Move is usually accessed via one of its aliases, which are 
    /// always preferred for disassembly.
    /// 
    /// If <imms> is greater than or equal to <immr>, this copies a bitfield of 
    /// (<imms>-<immr>+1) bits starting from bit position <immr> in the source 
    /// register to the least significant bits of the destination register.
    ///
    /// If <imms> is less than <immr>, this copies a bitfield of (<imms>+1) bits 
    /// from the least significant bits of the source register to bit position 
    /// (regsize-<immr>) of the destination register, where regsize is the 
    /// destination register size of 32 or 64 bits.
    ///
    /// In both cases the other bits of the destination register remain 
    /// unchanged.
    ///
    /// This instruction is used by the aliases BFC, BFI, and BFXIL.
    fn bfm_32(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::bfm_32`]
    fn bfm_64(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/UBFM--Unsigned-Bitfield-Move-?lang=en)
    /// Unsigned Bitfield Move is usually accessed via one of its aliases, 
    /// which are always preferred for disassembly.
    ///
    /// If <imms> is greater than or equal to <immr>, this copies a bitfield of 
    /// (<imms>-<immr>+1) bits starting from bit position <immr> in the source 
    /// register to the least significant bits of the destination register.
    ///
    /// If <imms> is less than <immr>, this copies a bitfield of (<imms>+1) bits
    /// from the least significant bits of the source register to bit position 
    /// (regsize-<immr>) of the destination register, where regsize is the 
    /// destination register size of 32 or 64 bits.
    ///
    /// In both cases the destination bits below and above the bitfield are set 
    /// to zero.
    ///
    /// This instruction is used by the aliases LSL (immediate), LSR 
    /// (immediate), UBFIZ, UBFX, UXTB, and UXTH.
    fn ubfm_32(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::ubfm_32`]
    fn ubfm_64(&mut self, rd: RegA64, rn: RegA64, imms: u32, immr: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/EXTR--Extract-register-?lang=en)
    /// Extract register extracts a register from a pair of registers.
    ///
    /// This instruction is used by the alias ROR (immediate).
    fn extr_32(&mut self, rd: RegA64, rn: RegA64, imms: u32, rm: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::extr_32`]
    fn extr_64(&mut self, rd: RegA64, rn: RegA64, imms: u32, rm: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/UDIV--Unsigned-Divide-?lang=en)
    /// Unsigned Divide divides an unsigned integer register value by another 
    /// unsigned integer register value, and writes the result to the 
    /// destination register. The condition flags are not affected.
    fn udiv_32(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::udiv_32`]
    fn udiv_64(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/SDIV--Signed-Divide-?lang=en)
    /// Signed Divide divides a signed integer register value by another signed 
    /// integer register value, and writes the result to the destination 
    /// register. The condition flags are not affected.
    fn sdiv_32(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::sdiv_32`]
    fn sdiv_64(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/LSLV--Logical-Shift-Left-Variable-?lang=en)
    /// Logical Shift Left Variable shifts a register value left by a variable 
    /// number of bits, shifting in zeros, and writes the result to the 
    /// destination register. The remainder obtained by dividing the second 
    /// source register by the data size defines the number of bits by which 
    /// the first source register is left-shifted.
    /// 
    /// This instruction is used by the alias LSL (register).
    fn lslv_32(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::lslv_32`]
    fn lslv_64(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/LSRV--Logical-Shift-Right-Variable-?lang=en)
    /// Logical Shift Right Variable shifts a register value right by a variable 
    /// number of bits, shifting in zeros, and writes the result to the 
    /// destination register. The remainder obtained by dividing the second 
    /// source register by the data size defines the number of bits by which 
    /// the first source register is right-shifted.
    /// 
    /// This instruction is used by the alias LSR (register).
    fn lsrv_32(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::lsrv_32`]
    fn lsrv_64(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/ASRV--Arithmetic-Shift-Right-Variable-?lang=en)
    /// Arithmetic Shift Right Variable shifts a register value right by a 
    /// variable number of bits, shifting in copies of its sign bit, and writes 
    /// the result to the destination register. The remainder obtained by 
    /// dividing the second source register by the data size defines the number 
    /// of bits by which the first source register is right-shifted.
    /// 
    /// This instruction is used by the alias ASR (register).
    fn asrv_32(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::asrv_32`]
    fn asrv_64(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
     -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/RORV--Rotate-Right-Variable-?lang=en)
    /// Rotate Right Variable provides the value of the contents of a register 
    /// rotated by a variable number of bits. The bits that are rotated off the 
    /// right end are inserted into the vacated bit positions on the left. The 
    /// remainder obtained by dividing the second source register by the data 
    /// size defines the number of bits by which the first source register is 
    /// right-shifted.
    /// 
    /// This instruction is used by the alias ROR (register).
    fn rorv_32(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::rorv_32`]
    fn rorv_64(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/CRC32B--CRC32H--CRC32W--CRC32X--CRC32-checksum-?lang=en#CRC32B_32C_dp_2src)
    /// CRC32 checksum performs a cyclic redundancy check (CRC) calculation on a 
    /// value held in a general-purpose register. It takes an input CRC value in 
    /// the first source operand, performs a CRC on the input value in the 
    /// second source operand, and returns the output CRC value. The second 
    /// source operand can be 8, 16, 32, or 64 bits. To align with common usage, 
    /// the bit order of the values is reversed as part of the operation, and 
    /// the polynomial 0x04C11DB7 is used for the CRC calculation.
    ///
    /// In an Armv8.0 implementation, this is an optional instruction. From 
    /// Armv8.1, it is mandatory for all implementations to implement this 
    /// instruction.
    fn crc32b(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::crc32b`]
    fn crc32h(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::crc32b`]
    fn crc32w(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::crc32b`]
    fn crc32cb(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::crc32b`]
    fn crc32ch(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::crc32b`]
    fn crc32cw(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::crc32b`]
    fn crc32x(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::crc32b`]
    fn crc32cx(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    
    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/SUBP--Subtract-Pointer-?lang=en)
    /// Subtract Pointer subtracts the 56-bit address held in the second source 
    /// register from the 56-bit address held in the first source register, 
    /// sign-extends the result to 64-bits, and writes the result to the 
    /// destination register.
    fn subp(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    
    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/SUBPS--Subtract-Pointer--setting-Flags-?lang=en)
    /// Subtract Pointer, setting Flags subtracts the 56-bit address held in the 
    /// second source register from the 56-bit address held in the first source 
    /// register, sign-extends the result to 64-bits, and writes the result to 
    /// the destination register. It updates the condition flags based on the 
    /// result of the subtraction.
    /// 
    /// This instruction is used by the alias CMPP.
    fn subps(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/IRG--Insert-Random-Tag-?lang=en)
    /// Insert Random Tag inserts a random Logical Address Tag into the address 
    /// in the first source register, and writes the result to the destination 
    /// register. Any tags specified in the optional second source register or 
    /// in GCR_EL1.Exclude are excluded from the selection of the random Logical 
    /// Address Tag.
    fn irg(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/GMI--Tag-Mask-Insert-?lang=en)
    /// Tag Mask Insert inserts the tag in the first source register into the 
    /// excluded set specified in the second source register, writing the new 
    /// excluded set to the destination register.
    fn gmi(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
        
    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/PACGA--Pointer-Authentication-Code--using-Generic-key-?lang=en)
    /// Pointer Authentication Code, using Generic key. This instruction 
    /// computes the pointer authentication code for an address in the first 
    /// source register, using a modifier in the second source register, and the 
    /// Generic key. The computed pointer authentication code is returned in the 
    /// upper 32 bits of the destination register.
    fn pacga(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/RBIT--Reverse-Bits-?lang=en)
    /// Reverse Bits reverses the bit order in a register.
    fn rbit_32(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::rbit_32`]
    fn rbit_64(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/REV16--Reverse-bytes-in-16-bit-halfwords-?lang=en)
    /// Reverse bytes in 16-bit halfwords reverses the byte order in each 16-bit 
    /// halfword of a register.
    fn rev16_32(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::rev16_32`]
    fn rev16_64(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/REV--Reverse-Bytes-?lang=en)
    /// Reverse Bytes reverses the byte order in a register.
    /// 
    /// This instruction is used by the pseudo-instruction REV64.
    fn rev_32(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::rev_32`]
    fn rev_64(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/REV32--Reverse-bytes-in-32-bit-words-?lang=en)
    /// Reverse bytes in 32-bit words reverses the byte order in each 32-bit 
    /// word of a register.
    fn rev32(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/CLZ--Count-Leading-Zeros-?lang=en)
    /// Count Leading Zeros counts the number of binary zero bits before the 
    /// first binary one bit in the value of the source register, and writes the 
    /// result to the destination register.
    fn clz_32(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::clz_32`]
    fn clz_64(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/CLS--Count-Leading-Sign-bits-?lang=en)
    /// Count Leading Sign bits counts the number of leading bits of the source 
    /// register that have the same value as the most significant bit of the 
    /// register, and writes the result to the destination register. This count 
    /// does not include the most significant bit of the source register.
    fn cls_32(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::cls_32`]
    fn cls_64(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/PACIA--PACIA1716--PACIASP--PACIAZ--PACIZA--Pointer-Authentication-Code-for-Instruction-address--using-key-A-?lang=en#PACIA_64P_dp_1src)
    /// Pointer Authentication Code for Instruction address, using key A. This 
    /// instruction computes and inserts a pointer authentication code for an 
    /// instruction address, using a modifier and key A.
    /// 
    /// The address is:
    ///  - In the general-purpose register that is specified by <Xd> for PACIA 
    ///     and PACIZA.
    ///  - In X17, for PACIA1716.
    ///  - In X30, for PACIASP and PACIAZ.
    /// The modifier is:
    ///  - In the general-purpose register or stack pointer that is specified 
    ///     by <Xn|SP> for PACIA.
    ///  - The value zero, for PACIZA and PACIAZ.
    ///  - In X16, for PACIA1716.
    ///  - In SP, for PACIASP.
    /// It has encodings from 2 classes: Integer and System
    fn pacia(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::pacia`]
    fn paciza(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/PACIB--PACIB1716--PACIBSP--PACIBZ--PACIZB--Pointer-Authentication-Code-for-Instruction-address--using-key-B-?lang=en#PACIB_64P_dp_1src)
    /// Pointer Authentication Code for Instruction address, using key B. This 
    /// instruction computes and inserts a pointer authentication code for an 
    /// instruction address, using a modifier and key B.
    /// 
    /// The address is:
    ///  - In the general-purpose register that is specified by <Xd> for PACIB 
    ///     and PACIZB.
    ///  - In X17, for PACIB1716.
    ///  - In X30, for PACIBSP and PACIBZ.
    /// 
    /// The modifier is:
    ///  - In the general-purpose register or stack pointer that is specified 
    ///     by <Xn|SP> for PACIB.
    ///  - The value zero, for PACIZB and PACIBZ.
    ///  - In X16, for PACIB1716.
    ///  - In SP, for PACIBSP.
    /// It has encodings from 2 classes: Integer and System
    fn pacib(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::pacib`]
    fn pacizb(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/PACDA--PACDZA--Pointer-Authentication-Code-for-Data-address--using-key-A-?lang=en#PACDA_64P_dp_1src)
    /// Pointer Authentication Code for Data address, using key A. This 
    /// instruction computes and inserts a pointer authentication code for a 
    /// data address, using a modifier and key A.
    /// 
    /// The address is in the general-purpose register that is specified by 
    /// <Xd>.
    /// 
    /// The modifier is:
    ///  - In the general-purpose register or stack pointer that is specified by 
    ///  - <Xn|SP> for PACDA.
    ///  - The value zero, for PACDZA.
    fn pacda(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::pacda`]
    fn pacdza(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/PACDB--PACDZB--Pointer-Authentication-Code-for-Data-address--using-key-B-?lang=en#PACDB_64P_dp_1src)
    /// Pointer Authentication Code for Data address, using key B. This 
    /// instruction computes and inserts a pointer authentication code for a 
    /// data address, using a modifier and key B.
    /// 
    /// The address is in the general-purpose register that is specified by <Xd>.
    /// 
    /// The modifier is:
    ///  - In the general-purpose register or stack pointer that is specified 
    ///     by <Xn|SP> for PACDB.
    ///  - The value zero, for PACDZB.
    fn pacdb(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::pacdb`]
    fn pacdzb(&mut self, rd: RegA64, rn: RegA64)
    -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/AUTIA--AUTIA1716--AUTIASP--AUTIAZ--AUTIZA--Authenticate-Instruction-address--using-key-A-?lang=en#AUTIA_64P_dp_1src)
    /// Authenticate Instruction address, using key A. This instruction 
    /// authenticates an instruction address, using a modifier and key A.
    /// 
    /// The address is:
    ///  - In the general-purpose register that is specified by <Xd> for AUTIA 
    ///     and AUTIZA.
    ///  - In X17, for AUTIA1716.
    ///  - In X30, for AUTIASP and AUTIAZ.
    /// 
    /// The modifier is:
    ///  - In the general-purpose register or stack pointer that is specified by
    ///     <Xn|SP> for AUTIA.
    ///  - The value zero, for AUTIZA and AUTIAZ.
    ///  - In X16, for AUTIA1716.
    ///  - In SP, for AUTIASP.
    /// 
    /// If the authentication passes, the upper bits of the address are restored 
    /// to enable subsequent use of the address. If the authentication fails, 
    /// the upper bits are corrupted and any subsequent use of the address 
    /// results in a Translation fault.
    /// 
    /// It has encodings from 2 classes: Integer and System
    fn autia(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::autia`]
    fn autiza(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/AUTIB--AUTIB1716--AUTIBSP--AUTIBZ--AUTIZB--Authenticate-Instruction-address--using-key-B-?lang=en#AUTIB_64P_dp_1src)
    /// Authenticate Instruction address, using key B. This instruction 
    /// authenticates an instruction address, using a modifier and key B.
    /// 
    /// The address is:
    ///  - In the general-purpose register that is specified by <Xd> for AUTIB 
    ///     and AUTIZB.
    ///  - In X17, for AUTIB1716.
    ///  - In X30, for AUTIBSP and AUTIBZ.
    /// 
    /// The modifier is:
    ///  - In the general-purpose register or stack pointer that is specified by 
    ///     <Xn|SP> for AUTIB.
    ///  - The value zero, for AUTIZB and AUTIBZ.
    ///  - In X16, for AUTIB1716.
    ///  - In SP, for AUTIBSP.
    /// 
    /// If the authentication passes, the upper bits of the address are restored 
    /// to enable subsequent use of the address. If the authentication fails, 
    /// the upper bits are corrupted and any subsequent use of the address 
    /// results in a Translation fault.
    /// 
    /// It has encodings from 2 classes: Integer and System
    fn autib(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::autib`]
    fn autizb(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/AUTDA--AUTDZA--Authenticate-Data-address--using-key-A-?lang=en#AUTDA_64P_dp_1src)
    /// Authenticate Data address, using key A. This instruction authenticates 
    /// a data address, using a modifier and key A.
    /// 
    /// The address is in the general-purpose register that is specified by 
    ///     <Xd>.
    /// 
    /// The modifier is:
    ///  - In the general-purpose register or stack pointer that is specified by 
    ///     <Xn|SP> for AUTDA.
    ///  - The value zero, for AUTDZA.
    /// If the authentication passes, the upper bits of the address are restored 
    /// to enable subsequent use of the address. If the authentication fails, 
    /// the upper bits are corrupted and any subsequent use of the address 
    /// results in a Translation fault.
    fn autda(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::autda`]
    fn autdza(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/AUTDB--AUTDZB--Authenticate-Data-address--using-key-B-?lang=en#AUTDB_64P_dp_1src)
    /// Authenticate Data address, using key B. This instruction authenticates 
    /// a data address, using a modifier and key B.
    /// 
    /// The address is in the general-purpose register that is specified by 
    /// <Xd>.
    /// 
    /// The modifier is:
    ///  - In the general-purpose register or stack pointer that is specified 
    ///     by <Xn|SP> for AUTDB.
    ///  - The value zero, for AUTDZB.
    /// If the authentication passes, the upper bits of the address are restored 
    /// to enable subsequent use of the address. If the authentication fails, 
    /// the upper bits are corrupted and any subsequent use of the address 
    /// results in a Translation fault.
    fn autdb(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::autdb`]
    fn autdzb(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/XPACD--XPACI--XPACLRI--Strip-Pointer-Authentication-Code-?lang=en#XPACI_64Z_dp_1src)
    /// Strip Pointer Authentication Code. This instruction removes the pointer 
    /// authentication code from an address. The address is in the specified 
    /// general-purpose register for XPACI and XPACD, and is in LR for XPACLRI.
    /// 
    /// The XPACD instruction is used for data addresses, and XPACI and XPACLRI 
    /// are used for instruction addresses.
    /// 
    /// It has encodings from 2 classes: Integer and System
    fn xpaci(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/XPACD--XPACI--XPACLRI--Strip-Pointer-Authentication-Code-?lang=en#XPACD_64Z_dp_1src)
    /// Strip Pointer Authentication Code. This instruction removes the pointer 
    /// authentication code from an address. The address is in the specified 
    /// general-purpose register for XPACI and XPACD, and is in LR for XPACLRI.
    /// 
    /// The XPACD instruction is used for data addresses, and XPACI and XPACLRI 
    /// are used for instruction addresses.
    /// 
    /// It has encodings from 2 classes: Integer and System
    fn xpacd(&mut self, rd: RegA64, rn: RegA64)
        -> Result<(), Self::Error>;
    
    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/AND--shifted-register---Bitwise-AND--shifted-register--?lang=en)
    /// Bitwise AND (shifted register) performs a bitwise AND of a register 
    /// value and an optionally-shifted register value, and writes the result 
    /// to the destination register.
    fn and_32(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::and_32`]
    fn and_64(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/ORR--shifted-register---Bitwise-OR--shifted-register--?lang=en)
    /// Bitwise OR (shifted register) performs a bitwise (inclusive) OR of a 
    /// register value and an optionally-shifted register value, and writes the 
    /// result to the destination register.
    /// This instruction is used by the alias MOV (register). 
    fn orr_32(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::orr_32`]
    fn orr_64(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/ORN--shifted-register---Bitwise-OR-NOT--shifted-register--?lang=en)
    /// Bitwise OR NOT (shifted register) performs a bitwise (inclusive) OR of 
    /// a register value and the complement of an optionally-shifted register 
    /// value, and writes the result to the destination register.
    /// This instruction is used by the alias MVN.
    fn orn_32(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::orn_32`]
    fn orn_64(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/EOR--shifted-register---Bitwise-Exclusive-OR--shifted-register--?lang=en)
    /// Bitwise Exclusive OR (shifted register) performs a bitwise Exclusive OR 
    /// of a register value and an optionally-shifted register value, and writes
    /// the result to the destination register.
    fn eor_32(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::eor_32`]
    fn eor_64(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/EON--shifted-register---Bitwise-Exclusive-OR-NOT--shifted-register--?lang=en)
    /// Bitwise Exclusive OR NOT (shifted register) performs a bitwise Exclusive
    /// OR NOT of a register value and an optionally-shifted register value, and 
    /// writes the result to the destination register.
    /// 
    fn eon_32(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::eon_32`]
    fn eon_64(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/ANDS--shifted-register---Bitwise-AND--shifted-register---setting-flags-?lang=en)
    /// Bitwise AND (shifted register), setting flags, performs a bitwise AND of 
    /// a register value and an optionally-shifted register value, and writes 
    /// the result to the destination register. It updates the condition flags 
    /// based on the result.
    /// This instruction is used by the alias TST (shifted register).
    fn ands_32(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::ands_32`]
    fn ands_64(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/BIC--shifted-register---Bitwise-Bit-Clear--shifted-register--?lang=en)
    /// Bitwise Bit Clear (shifted register) performs a bitwise AND of a 
    /// register value and the complement of an optionally-shifted register 
    /// value, and writes the result to the destination register.
    fn bic_32(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::bic_32`]
    fn bic_64(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/BICS--shifted-register---Bitwise-Bit-Clear--shifted-register---setting-flags-?lang=en)
    /// Bitwise Bit Clear (shifted register), setting flags, performs a bitwise 
    /// AND of a register value and the complement of an optionally-shifted 
    /// register value, and writes the result to the destination register. It 
    /// updates the condition flags based on the result.
    fn bics_32(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::bics_32`]
    fn bics_64(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/ADD--shifted-register---Add--shifted-register--?lang=en)
    /// Add (shifted register) adds a register value and an optionally-shifted 
    /// register value, and writes the result to the destination register.
    fn add_32(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::add_32`]
    fn add_64(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/ADDS--shifted-register---Add--shifted-register---setting-flags-?lang=en)
    /// Add (shifted register), setting flags, adds a register value and an 
    /// optionally-shifted register value, and writes the result to the 
    /// destination register. It updates the condition flags based on the 
    /// result.
    /// 
    /// This instruction is used by the alias CMN (shifted register).
    fn adds_32(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::adds_32`]
    fn adds_64(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/SUB--shifted-register---Subtract--shifted-register--?lang=en)
    /// Subtract (shifted register) subtracts an optionally-shifted register 
    /// value from a register value, and writes the result to the destination 
    /// register.
    /// 
    /// This instruction is used by the alias NEG (shifted register).
    fn sub_32(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::sub_32`]
    fn sub_64(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/SUBS--shifted-register---Subtract--shifted-register---setting-flags-?lang=en)
    /// Subtract (shifted register), setting flags, subtracts an 
    /// optionally-shifted register value from a register value, and writes the 
    /// result to the destination register. It updates the condition flags based 
    /// on the result.
    /// 
    /// This instruction is used by the aliases CMP (shifted register), and NEGS.
    fn subs_32(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::subs_32`]
    fn subs_64(&mut self, rd: RegA64, rn: RegA64, imm6: u32, rm: RegA64, shift: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/ADD--extended-register---Add--extended-register--?lang=en)
    /// Add (extended register) adds a register value and a sign or 
    /// zero-extended register value, followed by an optional left shift amount,
    /// and writes the result to the destination register. The argument that is 
    /// extended from the <Rm> register can be a byte, halfword, word, or 
    /// doubleword.
    fn add_ext_32(&mut self, rd: RegA64, rn: RegA64, imm3: u32, option: u32, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::add_ext_32`]
    fn add_ext_64(&mut self, rd: RegA64, rn: RegA64, imm3: u32, option: u32, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/ADDS--extended-register---Add--extended-register---setting-flags-?lang=en)
    /// Add (extended register), setting flags, adds a register value and a sign 
    /// or zero-extended register value, followed by an optional left shift 
    /// amount, and writes the result to the destination register. The argument 
    /// that is extended from the <Rm> register can be a byte, halfword, word, 
    /// or doubleword. It updates the condition flags based on the result.
    /// 
    /// This instruction is used by the alias CMN (extended register).
    fn adds_ext_32(&mut self, rd: RegA64, rn: RegA64, imm3: u32, option: u32, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::adds_ext_32`]
    fn adds_ext_64(&mut self, rd: RegA64, rn: RegA64, imm3: u32, option: u32, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/SUB--extended-register---Subtract--extended-register--?lang=en)
    /// Subtract (extended register) subtracts a sign or zero-extended register 
    /// value, followed by an optional left shift amount, from a register value, 
    /// and writes the result to the destination register. The argument that is 
    /// extended from the <Rm> register can be a byte, halfword, word, or 
    /// doubleword.
    fn sub_ext_32(&mut self, rd: RegA64, rn: RegA64, imm3: u32, option: u32, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::sub_ext_32`]
    fn sub_ext_64(&mut self, rd: RegA64, rn: RegA64, imm3: u32, option: u32, rm: RegA64)
        -> Result<(), Self::Error>;
    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/SUBS--extended-register---Subtract--extended-register---setting-flags-?lang=en)
    /// Subtract (extended register), setting flags, subtracts a sign or 
    /// zero-extended register value, followed by an optional left shift amount, 
    /// from a register value, and writes the result to the destination 
    /// register. The argument that is extended from the <Rm> register can be a 
    /// byte, halfword, word, or doubleword. It updates the condition flags 
    /// based on the result.
    /// 
    /// This instruction is used by the alias CMP (extended register).
    fn subs_ext_32(&mut self, rd: RegA64, rn: RegA64, imm3: u32, option: u32, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::subs_ext_32`]
    fn subs_ext_64(&mut self, rd: RegA64, rn: RegA64, imm3: u32, option: u32, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/ADC--Add-with-Carry-?lang=en)
    /// Add with Carry adds two register values and the Carry flag value, and 
    /// writes the result to the destination register.
    fn adc_32(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::adc_32`]
    fn adc_64(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    
    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/ADCS--Add-with-Carry--setting-flags-?lang=en)
    /// Add with Carry, setting flags, adds two register values and the Carry 
    /// flag value, and writes the result to the destination register. It 
    /// updates the condition flags based on the result.
    fn adcs_32(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::adcs_32`]
    fn adcs_64(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/SBC--Subtract-with-Carry-?lang=en)
    /// Subtract with Carry subtracts a register value and the value of NOT 
    /// (Carry flag) from a register value, and writes the result to the 
    /// destination register.
    /// 
    /// This instruction is used by the alias NGC.
    fn sbc_32(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::sbc_32`]
    fn sbc_64(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/SBCS--Subtract-with-Carry--setting-flags-?lang=en)
    /// Subtract with Carry, setting flags, subtracts a register value and the 
    /// value of NOT (Carry flag) from a register value, and writes the result 
    /// to the destination register. It updates the condition flags based on the 
    /// result.
    /// 
    /// This instruction is used by the alias NGCS.
    fn sbcs_32(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::sbcs_32`]
    fn sbcs_64(&mut self, rd: RegA64, rn: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/RMIF--Rotate--Mask-Insert-Flags-?lang=en)
    /// Performs a rotation right of a value held in a general purpose register 
    /// by an immediate value, and then inserts a selection of the bottom four 
    /// bits of the result of the rotation into the PSTATE flags, under the 
    /// control of a second immediate mask.
    fn rmif(&mut self, mask: u32, o2: u32, rn: RegA64, imm6: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/SETF8--SETF16--Evaluation-of-8-or-16-bit-flag-values-?lang=en#SETF16_only_setf)
    /// Set the PSTATE.NZV flags based on the value in the specified 
    /// general-purpose register. SETF8 treats the value as an 8 bit value, and
    /// SETF16 treats the value as an 16 bit value.
    /// 
    /// The PSTATE.C flag is not affected by these instructions.
    fn setf8(&mut self, rn: RegA64) -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::setf8`]
    fn setf16(&mut self, rn: RegA64) -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/CCMN--register---Conditional-Compare-Negative--register--?lang=en)
    /// Conditional Compare Negative (register) sets the value of the condition 
    /// flags to the result of the comparison of a register value and the 
    /// inverse of another register value if the condition is TRUE, and an 
    /// immediate value otherwise.
    fn ccmn_32(&mut self, nzcv: u32, rn: RegA64, cond: Cond, rm: RegA64) 
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::ccmn_32`]
    fn ccmn_64(&mut self, nzcv: u32, rn: RegA64, cond: Cond, rm: RegA64) 
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/CCMP--register---Conditional-Compare--register--?lang=en)
    /// Conditional Compare (register) sets the value of the condition flags to 
    /// the result of the comparison of two registers if the condition is TRUE, 
    /// and an immediate value otherwise.
    fn ccmp_32(&mut self, nzcv: u32, rn: RegA64, cond: Cond, rm: RegA64) 
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::ccmp_32`]
    fn ccmp_64(&mut self, nzcv: u32, rn: RegA64, cond: Cond, rm: RegA64) 
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/CCMN--immediate---Conditional-Compare-Negative--immediate--?lang=en)
    /// Conditional Compare Negative (immediate) sets the value of the condition 
    /// flags to the result of the comparison of a register value and a negated 
    /// immediate value if the condition is TRUE, and an immediate value 
    /// otherwise.
    fn ccmn_imm_32(&mut self, nzcv: u32, rn: RegA64, cond: Cond, imm5: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::ccmn_imm_32`]
    fn ccmn_imm_64(&mut self, nzcv: u32, rn: RegA64, cond: Cond, imm5: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/CCMP--immediate---Conditional-Compare--immediate--?lang=en)
    /// Conditional Compare (immediate) sets the value of the condition flags to 
    /// the result of the comparison of a register value and an immediate value 
    /// if the condition is TRUE, and an immediate value otherwise.
    fn ccmp_imm_32(&mut self, nzcv: u32, rn: RegA64, cond: Cond, imm5: u32)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::ccmp_imm_32`]
    fn ccmp_imm_64(&mut self, nzcv: u32, rn: RegA64, cond: Cond, imm5: u32)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/CSEL--Conditional-Select-?lang=en)
    /// If the condition is true, Conditional Select writes the value of the 
    /// first source register to the destination register. If the condition is 
    /// false, it writes the value of the second source register to the 
    /// destination register.
    fn csel_32(&mut self, rd: RegA64, rn: RegA64, cond: Cond, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::csel_32`]
    fn csel_64(&mut self, rd: RegA64, rn: RegA64, cond: Cond, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/CSINC--Conditional-Select-Increment-?lang=en)
    /// Conditional Select Increment returns, in the destination register, the
    /// value of the first source register if the condition is TRUE, and 
    /// otherwise returns the value of the second source register incremented 
    /// by 1.
    /// 
    /// This instruction is used by the aliases CINC, and CSET.
    fn csinc_32(&mut self, rd: RegA64, rn: RegA64, cond: Cond, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::csinc_32`]
    fn csinc_64(&mut self, rd: RegA64, rn: RegA64, cond: Cond, rm: RegA64)
        -> Result<(), Self::Error>;
    
    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/CSINV--Conditional-Select-Invert-?lang=en)
    /// Conditional Select Invert returns, in the destination register, the 
    /// value of the first source register if the condition is TRUE, and 
    /// otherwise returns the bitwise inversion value of the second source 
    /// register.
    /// 
    /// This instruction is used by the aliases CINV, and CSETM.
    fn csinv_32(&mut self, rd: RegA64, rn: RegA64, cond: Cond, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::csinv_32`]
    fn csinv_64(&mut self, rd: RegA64, rn: RegA64, cond: Cond, rm: RegA64)
        -> Result<(), Self::Error>;
    
    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/CSNEG--Conditional-Select-Negation-?lang=en)
    /// Conditional Select Negation returns, in the destination register, the value of the first source register if the condition is TRUE, and otherwise returns the negated value of the second source register.
    /// 
    /// This instruction is used by the alias CNEG.
    fn csneg_32(&mut self, rd: RegA64, rn: RegA64, cond: Cond, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::csneg_32`]
    fn csneg_64(&mut self, rd: RegA64, rn: RegA64, cond: Cond, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/MADD--Multiply-Add-?lang=en)
    /// Multiply-Add multiplies two register values, adds a third register 
    /// value, and writes the result to the destination register.
    /// 
    /// This instruction is used by the alias MUL.
    fn madd_32(&mut self, rd: RegA64, rn: RegA64, ra: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::madd_32`]
    fn madd_64(&mut self, rd: RegA64, rn: RegA64, ra: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/MSUB--Multiply-Subtract-?lang=en)
    /// Multiply-Subtract multiplies two register values, subtracts the product 
    /// from a third register value, and writes the result to the destination 
    /// register.
    /// 
    /// This instruction is used by the alias MNEG.
    fn msub_32(&mut self, rd: RegA64, rn: RegA64, ra: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;
    /// See [`ArmV8aA64User::msub_32`]
    fn msub_64(&mut self, rd: RegA64, rn: RegA64, ra: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/SMADDL--Signed-Multiply-Add-Long-?lang=en)
    /// Signed Multiply-Add Long multiplies two 32-bit register values, adds a 
    /// 64-bit register value, and writes the result to the 64-bit destination 
    /// register.
    /// 
    /// This instruction is used by the alias SMULL.
    fn smaddl(&mut self, rd: RegA64, rn: RegA64, ra: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/SMSUBL--Signed-Multiply-Subtract-Long-?lang=en)
    /// Signed Multiply-Subtract Long multiplies two 32-bit register values, 
    /// subtracts the product from a 64-bit register value, and writes the 
    /// result to the 64-bit destination register.
    /// 
    /// This instruction is used by the alias SMNEGL.
    fn smsubl(&mut self, rd: RegA64, rn: RegA64, ra: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/SMULH--Signed-Multiply-High-?lang=en)
    /// Signed Multiply High multiplies two 64-bit register values, and writes 
    /// bits[127:64] of the 128-bit result to the 64-bit destination register.
    fn smulh(&mut self, rd: RegA64, rn: RegA64, ra: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/UMADDL--Unsigned-Multiply-Add-Long-?lang=en)
    /// Unsigned Multiply-Add Long multiplies two 32-bit register values, adds a 
    /// 64-bit register value, and writes the result to the 64-bit destination 
    /// register.
    /// 
    /// This instruction is used by the alias UMULL.
    fn umaddl(&mut self, rd: RegA64, rn: RegA64, ra: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/UMSUBL--Unsigned-Multiply-Subtract-Long-?lang=en)
    /// Unsigned Multiply-Subtract Long multiplies two 32-bit register values, 
    /// subtracts the product from a 64-bit register value, and writes the 
    /// result to the 64-bit destination register.
    /// 
    /// This instruction is used by the alias UMNEGL.
    fn umsubl(&mut self, rd: RegA64, rn: RegA64, ra: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/UMULH--Unsigned-Multiply-High-?lang=en)
    /// Unsigned Multiply High multiplies two 64-bit register values, and writes 
    /// bits[127:64] of the 128-bit result to the 64-bit destination register.
    fn umulh(&mut self, rd: RegA64, rn: RegA64, ra: RegA64, rm: RegA64)
        -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/B--Branch-?lang=en)
    /// Branch causes an unconditional branch to a label at a PC-relative 
    /// offset, with a hint that this is not a subroutine call or return.
    fn b(&mut self, imm26: u32) -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/BL--Branch-with-Link-?lang=en)
    /// Branch with Link branches to a PC-relative offset, setting the register 
    /// X30 to PC+4. It provides a hint that this is a subroutine call.
    fn bl(&mut self, imm26: u32) -> Result<(), Self::Error>;

    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/B-cond--Branch-conditionally-?lang=en)
    /// Branch conditionally to a label at a PC-relative offset, with a hint 
    /// that this is not a subroutine call or return.
    fn b_cond(&mut self, cond: Cond, imm19: u32) 
        -> Result<(), Self::Error>;
    
    /// [Reference](https://developer.arm.com/documentation/ddi0602/2021-12/Base-Instructions/BC-cond--Branch-Consistent-conditionally-?lang=en)
    /// Branch Consistent conditionally to a label at a PC-relative offset, with 
    /// a hint that this branch will behave very consistently and is very 
    /// unlikely to change direction.
    fn bc_cond(&mut self, cond: Cond, imm19: u32) 
        -> Result<(), Self::Error>;

    
}

#[inline(always)]
fn data_processing_immediate<U: ArmV8aA64User>(user: &mut U, word: u32) 
    -> Result<(), ErrorDissArmV8aA64<U::Error>> {
    match word.extract_bits::<23,25>() {
        // ADR or ADRP
        0b000| 0b001 => {
            // 20 bits immediate
            let imm = (
                    word.extract_bits::<5, 22>() << 2
                ) | word.extract_bits::<29, 30>();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();

            if word.extract_bit::<31>() == 0 {
                user.adr( rd, imm)
            } else {
                user.adrp(rd, imm)
            }
        },
        // Add/subtract immediate
        0b010 => {
            let imm = word.extract_bits::<10, 21>();
            let rn: RegA64 = word.extract_bits::<5, 9>().into();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();

            match word.extract_bits::<29, 31>() {
                0b000 => user.add_imm_32( rd, rn, imm),
                0b001 => user.adds_imm_32(rd, rn, imm),
                0b010 => user.sub_imm_32( rd, rn, imm),
                0b011 => user.subs_imm_32(rd, rn, imm),

                0b100 => user.add_imm_64( rd, rn, imm),
                0b101 => user.adds_imm_64(rd, rn, imm),
                0b110 => user.sub_imm_64( rd, rn, imm),
                0b111 => user.subs_imm_64(rd, rn, imm),
                _ => {unreachable!();},
            }
        },
        // Add/subtract immediate, with tags
        0b011 => {
            unimplemented!("TODO!: figure out ADDG and SUBG");
        },
        // Logical (immediate)
        0b100 => {
            let immr = word.extract_bits::<15, 21>();
            let imms = word.extract_bits::<10, 15>();
            let rn: RegA64 = word.extract_bits::<5, 9>().into();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();

            // instructions with the N value set to 1 are UNALLOCATED
            if word.extract_bit::<22>() != 0 {
                return Err(
                    ErrorDissArmV8aA64::UnallocatedInstruction(word)
                );
            }

            match word.extract_bits::<29, 31>() {
                0b000 => user.and_imm_32(rd, rn, imms, immr),
                0b001 => user.orr_imm_32(rd, rn, imms, immr),
                0b010 => user.and_imm_32(rd, rn, imms, immr),
                0b011 => user.and_imm_32(rd, rn, imms, immr),

                0b100 => user.and_imm_64(rd, rn, imms, immr),
                0b101 => user.orr_imm_64(rd, rn, imms, immr),
                0b110 => user.and_imm_64(rd, rn, imms, immr),
                0b111 => user.and_imm_64(rd, rn, imms, immr),
                _ => {unreachable!()},
            }
        },
        // Move wide (immediate)
        0b101 => {
            let hw = word.extract_bits::<21, 22>();
            let imm16 = word.extract_bits::<5, 20>();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();

            match word.extract_bits::<29, 31>() {
                0b000 => user.movn_32(rd, imm16, hw),
                0b010 => user.movz_32(rd, imm16, hw),
                0b011 => user.movk_32(rd, imm16, hw),

                0b100 => user.movn_64(rd, imm16, hw),
                0b110 => user.movz_64(rd, imm16, hw),
                0b111 => user.movk_64(rd, imm16, hw),
                _ => {
                    return Err(
                        ErrorDissArmV8aA64::UnallocatedInstruction(word)
                    );
                },

            }
        },
        // Bitfield
        0b110 => {
            let immr = word.extract_bits::<16, 21>();
            let imms = word.extract_bits::<10, 15>();
            let rn: RegA64 = word.extract_bits::<5, 9>().into();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();
            // build a single opcode so the match is cleaner
            // (sf, opc[2], opc[1], N)
            let opcode = (word.extract_bits::<29, 31>() << 1)
                | word.extract_bit::<22>();
            match opcode {
                0b0_00_0 => user.sbfm_32(rd, rn, imms, immr),
                0b0_01_0 => user.bfm_32( rd, rn, imms, immr),
                0b0_10_0 => user.ubfm_32(rd, rn, imms, immr),

                0b1_00_1 => user.sbfm_64(rd, rn, imms, immr),
                0b1_01_1 => user.bfm_64( rd, rn, imms, immr),
                0b1_10_1 => user.ubfm_64(rd, rn, imms, immr),
                _ => {
                    return Err(
                        ErrorDissArmV8aA64::UnallocatedInstruction(word)
                    );
                },
            }
        },
        // Extract
        0b111 => {
            let rm = word.extract_bits::<16, 20>();
            let imms = word.extract_bits::<10, 15>();
            let rn: RegA64 = word.extract_bits::<5, 9>().into();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();
            // build a single opcode so that the match is cleaner
            // (sf, opc[2], opc[1], N, o0)
            let opcode = word.extract_bits::<21, 22>() |
                (word.extract_bits::<29, 31>() << 2);

            match opcode {
                0b00000 => {
                    user.extr_32(rd, rn, imms, rm)
                },
                0b10010 => {
                    user.extr_64(rd, rn, imms, rm)
                },
                _ => {
                    return Err(
                        ErrorDissArmV8aA64::UnallocatedInstruction(word)
                    );
                },
            }
        },
        _ => {unreachable!();},
    }.map_err(ErrorDissArmV8aA64::UserError)
}

#[inline(always)]
fn data_processing_register<U: ArmV8aA64User>(user: &mut U, word: u32) 
    -> Result<(), ErrorDissArmV8aA64<U::Error>> {
    match (word.extract_bit::<28>(), word.extract_bits::<21, 24>()) {
        // Data-processing (2 source)
        (1, 0b0110) => {
            let rn: RegA64 = word.extract_bits::<5, 9>().into();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();
            if word.extract_bit::<30>() == 0 {
                // Data-processing (2 source)
                let rm: RegA64 = word.extract_bits::<16, 20>().into();
                let opcode = word.extract_bits::<10, 15>();
                // combined opcode for easier match
                // (sd, opcode)
                let op = (word.extract_bit::<31>() << 6) | opcode;
                match op {
                    0b0_000010 => user.udiv_32(rd, rn, rm),
                    0b0_000011 => user.sdiv_32(rd, rn, rm),
                    0b0_001000 => user.lslv_32(rd, rn, rm),
                    0b0_001001 => user.lsrv_32(rd, rn, rm),
                    0b0_001010 => user.asrv_32(rd, rn, rm),
                    0b0_001011 => user.rorv_32(rd, rn, rm),
                    0b0_010000 => user.crc32b( rd, rn, rm),
                    0b0_010001 => user.crc32h( rd, rn, rm),
                    0b0_010010 => user.crc32w( rd, rn, rm),
                    0b0_010100 => user.crc32cb(rd, rn, rm),
                    0b0_010101 => user.crc32ch(rd, rn, rm),
                    0b0_010110 => user.crc32cw(rd, rn, rm),

                    0b1_000010 => user.udiv_64(rd, rn, rm),
                    0b1_000011 => user.sdiv_64(rd, rn, rm),
                    0b1_000100 => user.irg(    rd, rn, rm),
                    0b1_000101 => user.gmi(    rd, rn, rm),
                    0b1_001000 => user.lslv_64(rd, rn, rm),
                    0b1_001001 => user.lsrv_64(rd, rn, rm),
                    0b1_001010 => user.asrv_64(rd, rn, rm),
                    0b1_001011 => user.rorv_64(rd, rn, rm),
                    0b1_001100 => user.pacga(  rd, rn, rm),
                    0b1_010011 => user.crc32x( rd, rn, rm),
                    0b1_010111 => user.crc32cx(rd, rn, rm),
                    0b1_000000 => {
                        if word.extract_bit::<29>() != 0 {
                            user.subps(rd, rn, rm)
                        } else {
                            user.subp( rd, rn, rm)
                        }
                    },
                    _ => {
                        return Err(
                            ErrorDissArmV8aA64::UnallocatedInstruction(word)
                        );
                    },
                }
            } else {
                // Data-processing (1 source)
                let rn: RegA64 = word.extract_bits::<5, 9>().into();
                let rd: RegA64 = word.extract_bits::<0, 4>().into();
                let opcode = word.extract_bits::<10, 20>();
                let sf = word.extract_bit::<31>();
                // combined opcode for easier match
                // (sf, opcode2, opcode)
                match (sf << 10) | opcode {
                    0b0_00000_000000 => user.rbit_32(rd, rn),
                    0b0_00000_000001 => user.rev16_32(rd, rn),
                    0b0_00000_000010 => user.rev_32(rd, rn),
                    0b0_00000_000100 => user.clz_32(rd, rn),
                    0b0_00000_000101 => user.cls_32(rd, rn),
                    0b1_00000_000000 => user.rbit_64(rd, rn),
                    0b1_00000_000001 => user.rev16_64(rd, rn),
                    0b1_00000_000010 => user.rev32(rd, rn),
                    0b1_00000_000011 => user.rev_64(rd, rn),
                    0b1_00000_000100 => user.clz_64(rd, rn),
                    0b1_00000_000101 => user.cls_64(rd, rn),
                    0b1_00001_000000 => user.pacia(rd, rn),
                    0b1_00001_000001 => user.pacib(rd, rn),
                    0b1_00001_000010 => user.pacda(rd, rn),
                    0b1_00001_000011 => user.pacdb(rd, rn),
                    0b1_00001_000100 => user.autia(rd, rn),
                    0b1_00001_000101 => user.autib(rd, rn),
                    0b1_00001_000110 => user.autda(rd, rn),
                    0b1_00001_000111 => user.autdb(rd, rn),
                    0b1_00001_001000 => {
                        if rn != 0b11111.into() {
                            return Err(
                                ErrorDissArmV8aA64::UnallocatedInstruction(word)
                            );
                        }
                        user.paciza(rd, rn)
                    },
                    0b1_00001_001001 => {
                        if rn != 0b11111.into() {
                            return Err(
                                ErrorDissArmV8aA64::UnallocatedInstruction(word)
                            );
                        }
                        user.pacizb(rd, rn)
                    },
                    0b1_00001_001010 => {
                        if rn != 0b11111.into() {
                            return Err(
                                ErrorDissArmV8aA64::UnallocatedInstruction(word)
                            );
                        }
                        user.pacdza(rd, rn)
                    },
                    0b1_00001_001011 => {
                        if rn != 0b11111.into() {
                            return Err(
                                ErrorDissArmV8aA64::UnallocatedInstruction(word)
                            );
                        }
                        user.pacdzb(rd, rn)
                    },
                    0b1_00001_001100 => {
                        if rn != 0b11111.into() {
                            return Err(
                                ErrorDissArmV8aA64::UnallocatedInstruction(word)
                            );
                        }
                        user.autiza(rd, rn)
                    },
                    0b1_00001_001101 => {
                        if rn != 0b11111.into() {
                            return Err(
                                ErrorDissArmV8aA64::UnallocatedInstruction(word)
                            );
                        }
                        user.autizb(rd, rn)
                    },
                    0b1_00001_001110 => {
                        if rn != 0b11111.into() {
                            return Err(
                                ErrorDissArmV8aA64::UnallocatedInstruction(word)
                            );
                        }
                        user.autdza(rd, rn)
                    },
                    0b1_00001_001111 => {
                        if rn != 0b11111.into() {
                            return Err(
                                ErrorDissArmV8aA64::UnallocatedInstruction(word)
                            );
                        }
                        user.autdzb(rd, rn)
                    },
                    0b1_00001_010000 => {
                        if rn != 0b11111.into() {
                            return Err(
                                ErrorDissArmV8aA64::UnallocatedInstruction(word)
                            );
                        }
                        user.xpaci(rd, rn)
                    },
                    0b1_00001_010001 => {
                        if rn != 0b11111.into() {
                            return Err(
                                ErrorDissArmV8aA64::UnallocatedInstruction(word)
                            );
                        }
                        user.xpacd(rd, rn)
                    },
                    _ => {
                        return Err(
                            ErrorDissArmV8aA64::UnallocatedInstruction(word)
                        );
                    }
                }
            }
        },
        // Logical (shifted register)
        (0, 0b0000 | 0b0001 | 0b0010 | 0b0011 
            | 0b0100 | 0b0101 | 0b0110 | 0b0111) => {
            let sf_opc = word.extract_bits::<29, 31>();
            let shift = word.extract_bits::<22, 23>();
            let n  = word.extract_bit::<21>();
            let rm: RegA64 = word.extract_bits::<16, 20>().into();
            let imm6 = word.extract_bits::<10, 15>();
            let rn: RegA64 = word.extract_bits::<5, 9>().into();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();
            // combined opcode for easier match
            // (sf, opc, N)
            match (sf_opc << 1) | n {
                0b0_00_0 => user.and_32( rd, rn, imm6, rm, shift),
                0b0_00_1 => user.bic_32( rd, rn, imm6, rm, shift),
                0b0_01_0 => user.orr_32( rd, rn, imm6, rm, shift),
                0b0_01_1 => user.orn_32( rd, rn, imm6, rm, shift),
                0b0_10_0 => user.eor_32( rd, rn, imm6, rm, shift),
                0b0_10_1 => user.eon_32( rd, rn, imm6, rm, shift),
                0b0_11_0 => user.ands_32(rd, rn, imm6, rm, shift),
                0b0_11_1 => user.bics_32(rd, rn, imm6, rm, shift),

                0b1_00_0 => user.and_64( rd, rn, imm6, rm, shift),
                0b1_00_1 => user.bic_64( rd, rn, imm6, rm, shift),
                0b1_01_0 => user.orr_64( rd, rn, imm6, rm, shift),
                0b1_01_1 => user.orn_64( rd, rn, imm6, rm, shift),
                0b1_10_0 => user.eor_64( rd, rn, imm6, rm, shift),
                0b1_10_1 => user.eon_64( rd, rn, imm6, rm, shift),
                0b1_11_0 => user.ands_64(rd, rn, imm6, rm, shift),
                0b1_11_1 => user.bics_64(rd, rn, imm6, rm, shift),
                x => {
                    unreachable!("this value should be a 5bit integer {}", x);
                }
            }
        },
        // Add/subtract (shifted register)
        (0, 0b1000 | 0b1010 | 0b1100 | 0b1110) => {
            let sf_op_s = word.extract_bits::<29, 31>();
            let shift = word.extract_bits::<22, 23>();
            let rm: RegA64 = word.extract_bits::<16, 20>().into();
            let imm6 = word.extract_bits::<10, 15>();
            let rn: RegA64 = word.extract_bits::<5, 9>().into();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();
            match sf_op_s {
                0b0_0_0 => user.add_32( rd, rn, imm6, rm, shift),
                0b0_0_1 => user.adds_32(rd, rn, imm6, rm, shift),
                0b0_1_0 => user.sub_32( rd, rn, imm6, rm, shift),
                0b0_1_1 => user.subs_32(rd, rn, imm6, rm, shift),

                0b1_0_0 => user.add_64( rd, rn, imm6, rm, shift),
                0b1_0_1 => user.adds_64(rd, rn, imm6, rm, shift),
                0b1_1_0 => user.sub_64( rd, rn, imm6, rm, shift),
                0b1_1_1 => user.subs_64(rd, rn, imm6, rm, shift),
                x => {
                    unreachable!("this value should be a 3bit integer {}", x);
                }
            }
        },
        // Add/subtract (extended register)
        (0, 0b1001 | 0b1011 | 0b1101 | 0b1111) => {
            let sf_op_s = word.extract_bits::<29, 31>();
            let opt = word.extract_bits::<22, 23>();
            let rm: RegA64 = word.extract_bits::<16, 20>().into();
            let option = word.extract_bits::<13, 15>();
            let imm3 = word.extract_bits::<10, 12>();
            let rn: RegA64 = word.extract_bits::<5, 9>().into();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();

            if opt != 0b00 {
                return Err(
                    ErrorDissArmV8aA64::UnallocatedInstruction(word)
                );
            }

            match sf_op_s {
                0b0_0_0 => user.add_ext_32( rd, rn, imm3, option, rm),
                0b0_0_1 => user.adds_ext_32(rd, rn, imm3, option, rm),
                0b0_1_0 => user.sub_ext_32( rd, rn, imm3, option, rm),
                0b0_1_1 => user.subs_ext_32(rd, rn, imm3, option, rm),

                0b1_0_0 => user.add_ext_64( rd, rn, imm3, option, rm),
                0b1_0_1 => user.adds_ext_64(rd, rn, imm3, option, rm),
                0b1_1_0 => user.sub_ext_64( rd, rn, imm3, option, rm),
                0b1_1_1 => user.subs_ext_64(rd, rn, imm3, option, rm),
                x => {
                    unreachable!("this value should be a 3bit integer {}", x);
                }
            }
        },
        (1, 0b0000) => {
            match word.extract_bits::<10, 15>() {
                // Add/subtract (with carry)
                0b000000 => {
                    let sf_op_s = word.extract_bits::<29, 31>();
                    let rm: RegA64 = word.extract_bits::<16, 20>().into();
                    let rn: RegA64 = word.extract_bits::<5, 9>().into();
                    let rd: RegA64 = word.extract_bits::<0, 4>().into();
                    match sf_op_s {
                        0b0_0_0 => user.adc_32( rd, rn, rm),
                        0b0_0_1 => user.adcs_32(rd, rn, rm),
                        0b0_1_0 => user.sbc_32( rd, rn, rm),
                        0b0_1_1 => user.sbcs_32(rd, rn, rm),

                        0b1_0_0 => user.adc_64( rd, rn, rm),
                        0b1_0_1 => user.adcs_64(rd, rn, rm),
                        0b1_1_0 => user.sbc_64( rd, rn, rm),
                        0b1_1_1 => user.sbcs_64(rd, rn, rm),
                        x => {
                            unreachable!("this value should be a 3bit integer {}", x);
                        }
                    }
                },
                // Rotate right into flags
                0b000001 | 0b100001 => {
                    let sf_op_s = word.extract_bits::<29, 31>();
                    let imm6 = word.extract_bits::<15, 20>();
                    let rn: RegA64 = word.extract_bits::<5, 9>().into();
                    let o2 = word.extract_bit::<4>();
                    let mask = word.extract_bits::<0, 3>();

                    if sf_op_s != 0b1_0_1 || o2 != 0 {
                        return Err(
                            ErrorDissArmV8aA64::UnallocatedInstruction(word)
                        );
                    }

                    user.rmif(mask, o2, rn, imm6)
                },
                // Evaluate into flags
                0b000010 | 0b010010 | 0b100010 | 0b110010 => {
                    let sf_op_s = word.extract_bits::<29, 31>();
                    let opcode2 = word.extract_bits::<15, 20>();
                    let sz = word.extract_bit::<14>();
                    let rn: RegA64 = word.extract_bits::<5, 9>().into();
                    let o3 = word.extract_bit::<4>();
                    let mask = word.extract_bits::<0, 3>();

                    match (sf_op_s, opcode2, sz, o3, mask) {
                        (0b0_0_1, 0b000000, 0, 0, 1101) => 
                            user.setf8(rn),
                        (0b0_0_1, 0b000000, 1, 0, 1101) => 
                            user.setf16(rn),
                        _ => {
                            return Err(
                                ErrorDissArmV8aA64::UnallocatedInstruction(word)
                            );
                        }
                    }
                },
                _ => {
                    return Err(
                        ErrorDissArmV8aA64::UnallocatedInstruction(word)
                    );
                },
            }
        },
        (1, 0b0010) => {
            let sf_op_s = word.extract_bits::<29, 31>();
            let cond: Cond = word.extract_bits::<12, 15>().into();
            let o2 = word.extract_bit::<10>();
            let rn: RegA64 = word.extract_bits::<5, 9>().into();
            let o3 = word.extract_bit::<4>();
            let nzcv = word.extract_bits::<0, 3>();

            if o2 != 0 || o3 != 0 {
                return Err(
                    ErrorDissArmV8aA64::UnallocatedInstruction(word)
                );
            }

            if word.extract_bit::<11>() == 0 { 
                // Conditional compare (register)
                let rm: RegA64 = word.extract_bits::<16, 20>().into();

                match sf_op_s {
                    0b0_0_1 => user.ccmn_32(nzcv, rn, cond, rm),
                    0b0_1_1 => user.ccmp_32(nzcv, rn, cond, rm),

                    0b1_0_1 => user.ccmn_64(nzcv, rn, cond, rm),
                    0b1_1_1 => user.ccmp_64(nzcv, rn, cond, rm),
                    _ => {
                        return Err(
                            ErrorDissArmV8aA64::UnallocatedInstruction(word)
                        );
                    }
                }
            } else {
                // Conditional compare (immediate)
                let imm5 = word.extract_bits::<16, 20>();

                match sf_op_s {
                    0b0_0_1 => user.ccmn_imm_32(nzcv, rn, cond, imm5),
                    0b0_1_1 => user.ccmp_imm_32(nzcv, rn, cond, imm5),

                    0b1_0_1 => user.ccmn_imm_64(nzcv, rn, cond, imm5),
                    0b1_1_1 => user.ccmp_imm_64(nzcv, rn, cond, imm5),
                    _ => {
                        return Err(
                            ErrorDissArmV8aA64::UnallocatedInstruction(word)
                        );
                    }
                }
            }
        },
        // Conditional select
        (1, 0b0100) => {
            let sf_op  = word.extract_bits::<30, 31>();
            let s      = word.extract_bit::<29>();
            let rm: RegA64 = word.extract_bits::<16, 20>().into();
            let cond: Cond = word.extract_bits::<12, 15>().into();
            let op2    = word.extract_bits::<10, 11>();
            let rn: RegA64 = word.extract_bits::< 5,  9>().into();
            let rd: RegA64 = word.extract_bits::< 0,  4>().into();
                        
            if s != 0 {
                return Err(
                    ErrorDissArmV8aA64::UnallocatedInstruction(word)
                );
            }

            // composite match for denser match
            //(sf, op, op2)
            let op = (sf_op << 2) | op2;
            match op {
                0b0_0_00 => user.csel_32( rd, rn, cond, rm),
                0b0_0_01 => user.csinc_32(rd, rn, cond, rm),
                0b0_1_00 => user.csinv_32(rd, rn, cond, rm),
                0b0_1_01 => user.csneg_32(rd, rn, cond, rm),

                0b1_0_00 => user.csel_64( rd, rn, cond, rm),
                0b1_0_01 => user.csinc_64(rd, rn, cond, rm),
                0b1_1_00 => user.csinv_64(rd, rn, cond, rm),
                0b1_1_01 => user.csneg_64(rd, rn, cond, rm),
                _ => {
                    return Err(
                        ErrorDissArmV8aA64::UnallocatedInstruction(word)
                    );
                }
            }
        },
        // Data-processing (3 source)
        (1, 0b1000 | 0b1001 | 0b1010 | 0b1011 
            | 0b1100 | 0b1101 | 0b1110 | 0b1111) => {
            let sf_op54 = word.extract_bits::<29, 31>();
            let op31 = word.extract_bits::<21, 23>();
            let rm: RegA64 = word.extract_bits::<16, 20>().into();
            let o0 = word.extract_bit::<15>();
            let ra: RegA64 = word.extract_bits::<10, 14>().into();
            let rn: RegA64 = word.extract_bits::<5, 9>().into();
            let rd: RegA64 = word.extract_bits::<0, 4>().into();
            
            // combine opcode for faster match
            let op =  (sf_op54 << 4) | (op31 << 1) | o0; 
            match op {
                0b0_00_000_0 => user.madd_32(rd, rn, ra, rm),
                0b0_00_000_1 => user.msub_32(rd, rn, ra, rm),
                
                0b1_00_000_0 => user.madd_64(rd, rn, ra, rm),
                0b1_00_000_1 => user.msub_64(rd, rn, ra, rm),

                0b1_00_001_0 => user.smaddl(rd, rn, ra, rm),
                0b1_00_001_1 => user.smsubl(rd, rn, ra, rm),
                0b1_00_010_0 => user.smulh( rd, rn, ra, rm),
                0b1_00_101_0 => user.umaddl(rd, rn, ra, rm),
                0b1_00_101_1 => user.umsubl(rd, rn, ra, rm),
                0b1_00_110_0 => user.umulh( rd, rn, ra, rm),
                _ => {
                    return Err(
                        ErrorDissArmV8aA64::UnallocatedInstruction(word)
                    );
                }
            }
        },
        _ => {unreachable!();}
    }.map_err(ErrorDissArmV8aA64::UserError)
}

#[inline(always)]
/// <https://developer.arm.com/documentation/ddi0602/2021-12/Index-by-Encoding/Branches--Exception-Generating-and-System-instructions?lang=en>
fn branches_exception_generating_and_system_instructions<U: ArmV8aA64User>
    (user: &mut U, word: u32) -> Result<(), ErrorDissArmV8aA64<U::Error>> {
    let op0 = word.extract_bits::<29, 21>();
    let op1 = word.extract_bits::<12, 25>();

    match op0 {
        // Conditional branch (immediate)
        0b010 => {
            if word.extract_bit::<25>() != 0 {
                return Err(
                    ErrorDissArmV8aA64::UnallocatedInstruction(word)
                );
            }
            let o1 = word.extract_bit::<24>();
            let imm19 = word.extract_bits::<4, 23>();
            let o0 = word.extract_bit::<4>();
            let cond = word.extract_bits::<0, 3>().into();
            
            match (o1 << 1) | o0 {
                0b0_0 => user.b_cond( cond, imm19),
                0b0_1 => user.bc_cond(cond, imm19),
                _ => {
                    return Err(
                        ErrorDissArmV8aA64::UnallocatedInstruction(word)
                    );
                }
            }
        },
        // lot of random shit
        0b110 => {
            unimplemented!("TODO")
        },
        // Unconditional branch (immediate)
        0b000 | 0b100 => {
            let imm26 = word.extract_bits::<0, 25>();
            if word.extract_bit::<31>() == 0 {
                user.b(imm26)
            } else {
                user.bl(imm26)
            }
        },
        0b001 | 0b101 => {
            if word.extract_bit::<25>() == 0 {
                // Compare and branch (immediate)
                unimplemented!("TODO")
            } else {
                // Test and branch immediate
                unimplemented!("TODO")
            }
        },
        _ => {
            return Err(
                ErrorDissArmV8aA64::UnallocatedInstruction(word)
            );
        },
    }.map_err(ErrorDissArmV8aA64::UserError)
}



// <https://developer.arm.com/documentation/ddi0602/2021-12/Index-by-Encoding>
pub fn disassemble_armv8a_a64<U: ArmV8aA64User>(user: &mut U, word: u32) 
    -> Result<(), ErrorDissArmV8aA64<U::Error>> {
    match word.extract_bits::<25,28>() {
        0b0000 => {
            if word.extract_bit::<31>() == 0 {
                return Err(ErrorDissArmV8aA64::UnallocatedInstruction(word));
            }
            unimplemented!("TODO!: SME");
        }
        // SVE encodings
        0b0010 => {
            unimplemented!("TODO!: SVE")
        }
        // Data Processing -- immediate
        0b1000 | 0b1001 => {
            data_processing_immediate(user, word)
        },
        // Branches, Exception Generating and System instructions
        0b1010 | 0b1011 => {
            branches_exception_generating_and_system_instructions(user, word)
        },
        // Loads and Stores
        0b0100 | 0b0110 | 0b1100 | 0b1110 => {
            unimplemented!("TODO!: Loads and Stores")
        },
        // Data Processing -- Register
        0b0101 | 0b1101 => {
            data_processing_register(user, word)
        },
        // Data Processing -- Scalar Floating-Point and Advanced SIMD
        0b0111 | 0b1111 => {
            unimplemented!("TODO!: Data Processing -- Scalar Floating-Point and Advanced SIMD")
        },

        x => Err(ErrorDissArmV8aA64::UnallocatedInstruction(x)),
    }
}