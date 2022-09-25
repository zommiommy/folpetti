use crate::utils::*;

pub enum Reg {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    Fp, // X29
    Lr, // x30

    Xzr,
    Pc,
    Sp,
    Elr,
    Spsr,
}

/// 
pub trait ArmV8A32User<E> {
    fn add(&mut self, condition: Cond, r1: Reg, r2: Reg, r3: Reg) -> Result<(), E>;
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


/// <https://developer.arm.com/documentation/ddi0406/cb/Application-Level-Architecture/ARM-Instruction-Set-Encoding/ARM-instruction-set-encoding>
pub fn disassemble_armv8a_a32<U: ArmV8A32User<()>>(user: &mut U, word: u32) -> Result<(), ()>{
    let cond = word.extract_bits::<28,32>();
    let op1  = word.extract_bits::<25,28>();

    // if cond == Cond::Unconditional {
    //     unimplemented!()
    // }

    match op1 {
        // Data processing and miscellaneous
        0b000 => {
            let op1 = word.extract_bits::<20,25>();
            let op2 = word.extract_bits::< 4, 8>();
            unimplemented!()
        },
        // Data processing and miscellaneous
        0b001 => {
            let op1 = word.extract_bits::<20, 25>();
            unimplemented!()
        },
        // Load/store word and unsigned byte
        0b010 => {
            unimplemented!()
        },
        0b011 => {
            let op = word.extract_bit::<4>();
            if op == 0 {
                // Load/store word and unsigned byte
                unimplemented!()
            } else {
                // Media Instructions
                unimplemented!()
            }
        },
        // Branch branch with link and block data transfer
        0b100 => {
            unimplemented!()
        },
        // Branch branch with link and block data transfer
        0b101 => {
            unimplemented!()
        },
        // coprocessor instruction and supervisor call + SIMD or FP
        0b110 => {
            unimplemented!()
        },
        // coprocessor instruction and supervisor call + SIMD or FP
        0b111 => {
            unimplemented!()
        },
        _ => {
            unreachable!();
        }
    }

    Ok(())
}