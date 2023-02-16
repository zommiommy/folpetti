use super::{Register, FloatRegister, FloatRoundingMode};

/// Instructions skipped: `uret, srtet, mret, wfi, sfence.vma`
/// Extensions Skipped: `RV32A, RV64A` do I need atomic right now?
/// G = IMAFD, Zicsr, Zifencei
/// 
/// Compact instructions will receive already translated registers
/// so I can implement it once.
pub trait RV64GCUser<T> {
    type Error;

    /// # Load Upper Immediate (RV32I)
    /// 
    /// Build 32-bit constants and uses the U-type format. LUI places the 
    /// U-immediate value in the top 20 bits of the destination register rd, 
    /// filling in the lowest 12 bits with zeros.
    /// 
    /// `x[rd] = sext(immediate[31:12] << 12)`
    fn lui(&mut self, rd: Register, imm: u32) -> Result<T, Self::Error>;

    /// # Add Upper Immediate to PC (RV32I)
    /// 
    /// Build pc-relative addresses and uses the U-type format. AUIPC forms a 
    /// 32-bit offset from the 20-bit U-immediate, filling in the lowest 12 bits 
    /// with zeros, adds this offset to the pc, then places the result in 
    /// register rd.
    /// 
    /// `x[rd] = pc + sext(immediate[31:12] << 12)`
    fn auipc(&mut self, rd: Register, imm: u32) -> Result<T, Self::Error>;

    /// # Add Immediate (RV32I)
    /// 
    /// Adds the sign-extended 12-bit immediate to register rs1. Arithmetic 
    /// overflow is ignored and the result is simply the low XLEN bits of the 
    /// result. ADDI rd, rs1, 0 is used to implement the MV rd, rs1 assembler 
    /// pseudo-instruction.
    /// 
    /// `x[rd] = x[rs1] + sext(immediate)`
    fn addi(&mut self, rd: Register, rs1: Register, imm: i32) 
        -> Result<T, Self::Error>;

    /// # Set Less Than Immediate (RV32I)
    /// 
    /// Place the value 1 in register rd if register rs1 is less than the 
    /// signextended immediate when both are treated as signed numbers, else 0 
    /// is written to rd.
    /// 
    /// `x[rd] = x[rs1] <s sext(immediate)`
    fn slti(&mut self, rd: Register, rs1: Register, imm: i32) 
        -> Result<T, Self::Error>;

    /// # Set Less Than Immediate Unsigned (RV32I)
    /// 
    /// Place the value 1 in register rd if register rs1 is less than the 
    /// immediate when both are treated as unsigned numbers, else 0 is written 
    /// to rd.
    /// 
    /// `x[rd] = x[rs1] <u sext(immediate)`
    fn sltiu(&mut self, rd: Register, rs1: Register, imm: u32)
        -> Result<T, Self::Error>;

    /// # Xor Immediate (RV32I)
    /// 
    /// Performs bitwise XOR on register rs1 and the sign-extended 12-bit 
    /// immediate and place the result in rd
    /// Note, “XORI rd, rs1, -1” performs a bitwise logical inversion of 
    /// register rs1(assembler pseudo-instruction NOT rd, rs)
    /// 
    /// `x[rd] = x[rs1] ^ sext(immediate)`
    fn xori(&mut self, rd: Register, rs1: Register, imm: i32) 
        -> Result<T, Self::Error>;

    /// # Or Immediate (RV32I)
    /// 
    /// Performs bitwise OR on register rs1 and the sign-extended 12-bit 
    /// immediate and place the result in rd
    /// 
    /// `x[rd] = x[rs1] | sext(immediate)`
    fn ori(&mut self, rd: Register, rs1: Register, imm: i32) 
        -> Result<T, Self::Error>;

    /// # And Immediate (RV32I)
    /// 
    /// Performs bitwise AND on register rs1 and the sign-extended 12-bit 
    /// immediate and place the result in rd
    /// 
    /// `x[rd] = x[rs1] & sext(immediate)`
    fn andi(&mut self, rd: Register, rs1: Register, imm: i32) 
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

    /// # Load Byte (RV32I)
    /// 
    /// Loads a 8-bit value from memory and sign-extends this to XLEN bits 
    /// before storing it in register rd.
    /// 
    /// `x[rd] = sext(M[x[rs1] + sext(offset)][7:0])`
    fn lb(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<T, Self::Error>;

    /// # Load Half word (RV32I)
    /// 
    /// Loads a 16-bit value from memory and sign-extends this to XLEN bits 
    /// before storing it in register rd.
    /// 
    /// `x[rd] = sext(M[x[rs1] + sext(offset)][15:0])`
    fn lh(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<T, Self::Error>;

    /// # Load Word (RV32I)
    /// 
    /// Loads a 32-bit value from memory and sign-extends this to XLEN bits 
    /// before storing it in register rd.
    /// 
    /// `x[rd] = sext(M[x[rs1] + sext(offset)][31:0])`
    fn lw(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<T, Self::Error>;

    /// # Load Double word (RV64I)
    /// 
    /// Loads a 64-bit value from memory into register rd for RV64I.
    /// 
    /// `x[rd] = M[x[rs1] + sext(offset)][63:0]`
    fn ld(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<T, Self::Error>;

    /// # Load Byte Unsigned (RV32I)
    /// 
    /// Loads a 8-bit value from memory and zero-extends this to XLEN bits 
    /// before storing it in register rd.
    /// 
    /// `x[rd] = M[x[rs1] + sext(offset)][7:0]`
    fn lbu(&mut self, rd: Register, rs1: Register,  imm: i32) -> Result<T, Self::Error>;

    /// # Load Half word Unsigned (RV32I)
    /// 
    /// Loads a 16-bit value from memory and zero-extends this to XLEN bits 
    /// before storing it in register rd.
    /// 
    /// `x[rd] = M[x[rs1] + sext(offset)][15:0]`
    fn lhu(&mut self, rd: Register, rs1: Register,  imm: i32) -> Result<T, Self::Error>;

    /// # Load Word Unsigned (RV64I)
    /// 
    /// Loads a 32-bit value from memory and zero-extends this to 64 bits before 
    /// storing it in register rd.
    /// 
    /// `x[rd] = M[x[rs1] + sext(offset)][31:0]`
    fn lwu(&mut self, rd: Register, rs1: Register,  imm: i32) -> Result<T, Self::Error>;

    /// # Store Byte (RV32I)
    /// 
    /// Store 8-bit, values from the low bits of register rs2 to memory.
    /// 
    /// `M[x[rs1] + sext(offset)] = x[rs2][7:0]`
    fn sb(&mut self, rs1: Register, rs2: Register, imm: i32) 
        -> Result<T, Self::Error>;

    /// # Store Half word (RV32I)
    /// 
    /// Store 16-bit, values from the low bits of register rs2 to memory.
    /// 
    /// `M[x[rs1] + sext(offset)] = x[rs2][15:0]`
    fn sh(&mut self, rs1: Register, rs2: Register, imm: i32) 
        -> Result<T, Self::Error>;

    /// # Store Half word (RV32I)
    /// 
    /// Store 32-bit, values from the low bits of register rs2 to memory.
    /// 
    /// `M[x[rs1] + sext(offset)] = x[rs2][31:0]`
    fn sw(&mut self, rs1: Register, rs2: Register, imm: i32) 
        -> Result<T, Self::Error>;

    /// # Store Double word (RV64I)
    /// 
    /// Store 64-bit, values from register rs2 to memory.
    /// 
    /// `M[x[rs1] + sext(offset)] = x[rs2][63:0]`
    fn sd(&mut self, rs1: Register, rs2: Register, imm: i32) 
        -> Result<T, Self::Error>;

    /// # Jump and link(RV32I)
    /// 
    /// Jump to address and place return address in rd.
    /// 
    /// `x[rd] = pc+4; pc += sext(offset)`
    fn jal(&mut self, rd: Register, imm: i32) -> Result<T, Self::Error>;

    /// # Jump and link in relative (RV32I)
    /// 
    /// Jump to address and place return address in rd.
    /// 
    /// `t =pc+4; pc=(x[rs1]+sext(offset))&∼1; x[rd]=t`
    fn jalr(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<T, Self::Error>;

    /// # Branch Equal (RV32I)
    /// 
    /// Take the branch if registers rs1 and rs2 are equal.
    /// 
    /// `if (rs1 == rs2) pc += sext(offset)`
    fn beq(&mut self, rs1: Register, rs2: Register, imm: i32) 
        -> Result<T, Self::Error>;

    /// # Branch Not Equal (RV32I)
    /// 
    /// Take the branch if registers rs1 and rs2 are not equal.
    /// 
    /// `if (rs1 != rs2) pc += sext(offset)`
    fn bne(&mut self, rs1: Register, rs2: Register, imm: i32) 
        -> Result<T, Self::Error>;

    /// # Branch Less Than (RV32I)
    /// 
    /// Take the branch if registers rs1 is less than rs2, using signed 
    /// comparison.
    /// 
    /// `if (rs1 <s rs2) pc += sext(offset)`
    fn blt(&mut self, rs1: Register, rs2: Register, imm: i32) 
        -> Result<T, Self::Error>;

    /// # Branch Greater Equal (RV32I)
    /// 
    /// Take the branch if registers rs1 is greater than rs2, using signed 
    /// comparison.
    /// 
    /// `if (rs1 >=s rs2) pc += sext(offset)`
    fn bge(&mut self, rs1: Register, rs2: Register, imm: i32) 
        -> Result<T, Self::Error>;

    /// # Branch Less Than Unsigned (RV32I)
    /// 
    /// Take the branch if registers rs1 is less than rs2, using unsigned 
    /// comparison.
    /// 
    /// `if (rs1 >u rs2) pc += sext(offset)`
    fn bltu(&mut self, rs1: Register, rs2: Register, imm: i32) 
        -> Result<T, Self::Error>;

    /// # Branch Greater Equal Unsigned (RV32I)
    /// 
    /// Take the branch if registers rs1 is greater than rs2, using unsigned 
    /// comparison.
    /// 
    /// `if (rs1 >=u rs2) pc += sext(offset)`
    fn bgeu(&mut self, rs1: Register, rs2: Register, imm: i32) 
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
    fn addiw(&mut self, rd: Register, rs1: Register, imm: i32) 
        -> Result<T, Self::Error>;
        
    /// # Shift Left Logical Immediate Word (RV64I)
    /// 
    /// Performs logical left shift on the 32-bit of value in register rs1 by 
    /// the shift amount held in the lower 5 bits of the immediate.
    /// Encodings with $imm[5] neq 0$ are reserved.
    /// 
    /// `x[rd] = sext((x[rs1] << shamt)[31:0])`
    fn slliw(&mut self, rd: Register, rs1: Register, shamt: i32) 
        -> Result<T, Self::Error>;

    /// # Shift Right Logical Immediate Word (RV64I)
    /// 
    /// Performs logical right shift on the 32-bit of value in register rs1 by 
    /// the shift amount held in the lower 5 bits of the immediate.
    /// Encodings with $imm[5] neq 0$ are reserved.
    /// 
    /// `x[rd] = sext(x[rs1][31:0] >>u shamt)`
    fn srliw(&mut self, rd: Register, rs1: Register, shamt: i32) 
        -> Result<T, Self::Error>;

    /// # Shift Right Arithmetical Immediate Word (RV64I)
    /// 
    /// Performs arithmetic right shift on the 32-bit of value in register rs1 
    /// by the shift amount held in the lower 5 bits of the immediate.
    /// Encodings with $imm[5] neq 0$ are reserved.
    /// 
    /// `x[rd] = sext(x[rs1][31:0] >>s shamt)`
    fn sraiw(&mut self, rd: Register, rs1: Register, shamt: i32) 
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
    fn c_ld(&mut self, rd: Register, rs1: Register, uimm: u16) 
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
    fn c_sd(&mut self, rs1: Register, rs2: Register, uimm: u16) 
        -> Result<T, Self::Error>;

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
    fn c_j(&mut self, imm: i16) -> Result<T, Self::Error>;

    /// # Compact Jump (RV64C)
    /// 
    /// Take the branch if the value in register rs1' is zero.
    /// 
    /// `if (x[8+rs1'] == 0) pc += sext(offset)`
    /// Translated:
    /// `if (x[rs1] == 0) pc += sext(offset)`
    fn c_beqz(&mut self, rs1: Register, offset: i16) -> Result<T, Self::Error>;

    /// # Compact Jump (RV64C)
    /// 
    /// Take the branch if the value in register rs1' is not zero.
    /// 
    /// `if (x[8+rs1'] != 0) pc += sext(offset)`
    /// Translated:
    /// `if (x[rs1] != 0) pc += sext(offset)`
    fn c_bnez(&mut self, rs1: Register, offset: i16) -> Result<T, Self::Error>;

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
    fn c_sdsp(&mut self, rs2: Register, uimm: u8) -> Result<T, Self::Error>;



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

    /// # Compact NOP (RV32C)
    /// 
    /// Does not change any user-visible state, except for advancing the pc.
    /// 
    fn c_nop(&mut self) -> Result<T, Self::Error>;

    /// # Compact Ebreak (RV32C)
    /// 
    /// Cause control to be transferred back to the debugging environment.
    fn c_ebreak(&mut self) -> Result<T, Self::Error>;
}