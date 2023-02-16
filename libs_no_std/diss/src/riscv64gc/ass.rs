use super::*;

pub struct AssemblerRV64GC;

impl AssemblerRV64GC {
    #[inline]
    pub fn lui(&mut self, rd: Register, imm: u32) -> Result<u32, &str> {
        if imm.extract_bitfield(0, 13) != 0 {
            return Err("the lower 12 bits of the LUI offset have to be zero!");
        }
        Ok(Utype{
            opcode: 0b0110111,
            imm,
            rd: rd.into(),
        }.into())
    }
    #[inline]
    pub fn auipc(&mut self, rd: Register, imm: u32) -> Result<u32, &str> {
        if imm.extract_bitfield(0, 13) != 0 {
            return Err("the lower 12 bits of the AUIPC offset have to be zero!");
        }
        Ok(Utype{
            opcode: 0b0010111,
            imm,
            rd: rd.into(),
        }.into())
    }
    #[inline]
    pub fn addi(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, &str> {
        Ok(Itype{
            opcode: 0b0010011,
            funct3: 0b000,
            imm,
            rd: rd.into(),
            rs1: rs1.into(),
        }.into())
    }
    #[inline]
    pub fn slti(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, &str> {
        Ok(Itype{
            opcode: 0b0010011,
            funct3: 0b010,
            imm,
            rd: rd.into(),
            rs1: rs1.into(),
        }.into())
    }
    #[inline]
    pub fn sltiu(&mut self, rd: Register, rs1: Register, imm: u32) -> Result<u32, &str> {
        Ok(Itype{
            opcode: 0b0010011,
            funct3: 0b011,
            imm: imm as i32,
            rd: rd.into(),
            rs1: rs1.into(),
        }.into())
    }
    #[inline]
    pub fn xori(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, &str> {
        Ok(Itype{
            opcode: 0b0010011,
            funct3: 0b100,
            imm,
            rd: rd.into(),
            rs1: rs1.into(),
        }.into())
    }
    #[inline]
    pub fn ori(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, &str> {
        Ok(Itype{
            opcode: 0b0010011,
            funct3: 0b110,
            imm,
            rd: rd.into(),
            rs1: rs1.into(),
        }.into())
    }
    #[inline]
    pub fn andi(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, &str> {
        Ok(Itype{
            opcode: 0b0010011,
            funct3: 0b111,
            imm,
            rd: rd.into(),
            rs1: rs1.into(),
        }.into())
    }
    #[inline]
    pub fn slli(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn srli(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn srai(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn add(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b000,
            funct7: 0b0000000,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn sub(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b000,
            funct7: 0b0100000,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn sll(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b001,
            funct7: 0b0000000,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn slt(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b010,
            funct7: 0b0000000,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn sltu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b011,
            funct7: 0b0000000,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn xor(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b100,
            funct7: 0b0000000,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn srl(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b101,
            funct7: 0b0000000,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn sra(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b101,
            funct7: 0b0100000,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn or(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b110,
            funct7: 0b0000000,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn and(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b111,
            funct7: 0b0000000,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn csrrw(&mut self, rd: Register, rs1: Register, offset: u32) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn csrrs(&mut self, rd: Register, rs1: Register, offset: u32) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn csrrc(&mut self, rd: Register, rs1: Register, offset: u32) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn csrrwi(&mut self, rd: Register, zimm: u8, offset: u32) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn csrrsi(&mut self, rd: Register, zimm: u8, offset: u32) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn csrrci(&mut self, rd: Register, zimm: u8, offset: u32) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn lb(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, &str> {
        Ok(Itype{
            opcode: 0b0000011,
            funct3: 0b000,
            rd: rd.into(),
            rs1: rs1.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn lh(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, &str> {
        Ok(Itype{
            opcode: 0b0000011,
            funct3: 0b001,
            rd: rd.into(),
            rs1: rs1.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn lw(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, &str> {
        Ok(Itype{
            opcode: 0b0000011,
            funct3: 0b010,
            rd: rd.into(),
            rs1: rs1.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn ld(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, &str> {
        Ok(Itype{
            opcode: 0b0000011,
            funct3: 0b011,
            rd: rd.into(),
            rs1: rs1.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn lbu(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, &str> {
        Ok(Itype{
            opcode: 0b0000011,
            funct3: 0b100,
            rd: rd.into(),
            rs1: rs1.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn lhu(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, &str> {
        Ok(Itype{
            opcode: 0b0000011,
            funct3: 0b101,
            rd: rd.into(),
            rs1: rs1.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn lwu(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, &str> {
        Ok(Itype{
            opcode: 0b0000011,
            funct3: 0b110,
            rd: rd.into(),
            rs1: rs1.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn sb(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<u32, &str> {
        Ok(Stype{
            opcode: 0b0100011,
            funct3: 0b000,
            rs1: rs1.into(),
            rs2: rs2.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn sh(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<u32, &str> {
        Ok(Stype{
            opcode: 0b0100011,
            funct3: 0b001,
            rs1: rs1.into(),
            rs2: rs2.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn sw(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<u32, &str> {
        Ok(Stype{
            opcode: 0b0100011,
            funct3: 0b010,
            rs1: rs1.into(),
            rs2: rs2.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn sd(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<u32, &str> {
        Ok(Stype{
            opcode: 0b0100011,
            funct3: 0b011,
            rs1: rs1.into(),
            rs2: rs2.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn jal(&mut self, rd: Register, imm: i32) -> Result<u32, &str> {
        Ok(Jtype{
            opcode: 0b1101111,
            rd: rd.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn jalr(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, &str> {
        Ok(Itype{
            opcode: 0b1100111,
            funct3: 0b000,
            rd: rd.into(),
            rs1: rs1.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn beq(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<u32, &str> {
        Ok(Btype{
            opcode: 0b1100011,
            funct3: 0b000,
            rs2: rs2.into(),
            rs1: rs1.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn bne(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<u32, &str> {
        Ok(Btype{
            opcode: 0b1100011,
            funct3: 0b001,
            rs2: rs2.into(),
            rs1: rs1.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn blt(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<u32, &str> {
        Ok(Btype{
            opcode: 0b1100011,
            funct3: 0b100,
            rs2: rs2.into(),
            rs1: rs1.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn bge(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<u32, &str> {
        Ok(Btype{
            opcode: 0b1100011,
            funct3: 0b101,
            rs2: rs2.into(),
            rs1: rs1.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn bltu(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<u32, &str> {
        Ok(Btype{
            opcode: 0b1100011,
            funct3: 0b110,
            rs2: rs2.into(),
            rs1: rs1.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn bgeu(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<u32, &str> {
        Ok(Btype{
            opcode: 0b1100011,
            funct3: 0b111,
            rs2: rs2.into(),
            rs1: rs1.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn addiw(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, &str> {
        Ok(Itype{
            opcode: 0b0011011,
            funct3: 0b00,
            rd: rd.into(),
            rs1: rs1.into(),
            imm,
        }.into())
    }
    #[inline]
    pub fn slliw(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn srliw(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn sraiw(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn addw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0111011,
            funct3: 0b000,
            funct7: 0b0000000,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn subw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0111011,
            funct3: 0b000,
            funct7: 0b0100000,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn sllw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0111011,
            funct3: 0b001,
            funct7: 0b0000000,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn srlw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0111011,
            funct3: 0b101,
            funct7: 0b0000000,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn sraw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0111011,
            funct3: 0b101,
            funct7: 0b0100000,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn mul(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b000,
            funct7: 0b0000001,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn mulh(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b001,
            funct7: 0b0000001,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn mulhsu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b010,
            funct7: 0b0000001,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn mulhu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b011,
            funct7: 0b0000001,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn div(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b100,
            funct7: 0b0000001,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn divu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b101,
            funct7: 0b0000001,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn rem(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b110,
            funct7: 0b0000001,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn remu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0110011,
            funct3: 0b111,
            funct7: 0b0000001,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn mulw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0111011,
            funct3: 0b000,
            funct7: 0b0000001,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn divw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0111011,
            funct3: 0b100,
            funct7: 0b0000001,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn divuw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0111011,
            funct3: 0b101,
            funct7: 0b0000001,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn remw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0111011,
            funct3: 0b110,
            funct7: 0b0000001,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn remuw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, &str> {
        Ok(Rtype{
            opcode: 0b0111011,
            funct3: 0b111,
            funct7: 0b0000001,
            rd: rd.into(),
            rs1: rs1.into(),
            rs2: rs2.into(),
        }.into())
    }
    #[inline]
    pub fn fmadd_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fmsub_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fnmsub_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fnmadd_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fadd_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fsub_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fmul_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fdiv_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fsqrt_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fsgnj_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fsgnjn_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fsgnjx_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fmin_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fmax_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_w_s(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_wu_s(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fmv_x_w(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn feq_s(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn flt_s(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fle_s(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fclass_s(&mut self, rd: Register, rs1: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_s_w(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_s_wu(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fmv_w_x(&mut self, rd: FloatRegister, rs1: Register) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fmadd_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fmsub_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fnmsub_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fnmadd_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fadd_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fsub_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fmul_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fdiv_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fsqrt_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fsgnj_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fsgnjn_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fsgnjx_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fmin_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fmax_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_s_d(&mut self, rd: FloatRegister, rs1: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_d_s(&mut self, rd: FloatRegister, rs1: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn feq_d(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn flt_d(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fle_d(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fclass_d(&mut self, rd: Register, rs1: FloatRegister) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_w_d(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_wu_d(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_d_w(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_d_wu(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn flw(&mut self, rd: FloatRegister, rs1: Register, imm: u32) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fsw(&mut self, rs1: Register, rs2: FloatRegister, offset: u32) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fld(&mut self, rd: FloatRegister, rs1: Register, offset: u32) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fsd(&mut self, rs1: Register, rs2: FloatRegister, offset: u32) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_l_s(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_lu_s(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_s_l(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_s_lu(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_l_d(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_lu_d(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fmv_x_d(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_d_l(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fcvt_d_lu(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn fmv_d_x(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, &str> {
        todo!();
        Ok(4)
    }
    #[inline]
    pub fn c_addi4spn(&mut self, rd: Register, uimm: u16) -> Result<u16, &str> {
        Ok(CIWtype{
            opcode: 0b00,
            funct3: 0b000,
            rd_prime: rd.into_prime(),
            imm: uimm,
        }.into())
    }
    #[inline]
    pub fn c_fld(&mut self, rd: Register, rs1: Register, uimm: u16) -> Result<u16, &str> {
        Ok(CLtype{
            opcode: 0b00,
            funct3: 0b001,
            rd_prime: rd.into_prime(),
            rs1_prime: rs1.into_prime(),
            imm2: uimm.extract_bitfield(3, 6),
            imm1: uimm.extract_bitfield(6, 8),
        }.into())
    }
    #[inline]
    pub fn c_lw(&mut self, rd: Register, rs1: Register, uimm: u16) -> Result<u16, &str> {
        Ok(CLtype{
            opcode: 0b00,
            funct3: 0b010,
            rd_prime: rd.into_prime(),
            rs1_prime: rs1.into_prime(),
            imm2: uimm.extract_bitfield(3, 6),
            imm1: (uimm.extract_bitfield(2, 3) << 1) | uimm.extract_bitfield(6, 7),
        }.into())
    }
    #[inline]
    pub fn c_ld(&mut self, rd: Register, rs1: Register, uimm: u16) -> Result<u16, &str> {
        Ok(CLtype{
            opcode: 0b00,
            funct3: 0b011,
            rd_prime: rd.into_prime(),
            rs1_prime: rs1.into_prime(),
            imm2: uimm.extract_bitfield(3, 6),
            imm1: uimm.extract_bitfield(6, 8),
        }.into())
    }
    #[inline]
    pub fn c_fsd(&mut self, rs1: Register, rs2: FloatRegister, uimm: u16) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_sw(&mut self, rs1: Register, rs2: Register, uimm: u16) -> Result<u16, &str> {
        Ok(CStype{
            opcode: 0b00,
            funct3: 0b010,
            rs2_prime: rs2.into_prime(),
            rs1_prime: rs1.into_prime(),
            imm2: uimm.extract_bitfield(3, 6),
            imm1: (uimm.extract_bitfield(2, 3) << 1) | uimm.extract_bitfield(6, 7),
        }.into())
    }
    #[inline]
    pub fn c_sd(&mut self, rs1: Register, rs2: Register, uimm: u16) -> Result<u16, &str> {
        Ok(CStype{
            opcode: 0b00,
            funct3: 0b111,
            rs2_prime: rs2.into_prime(),
            rs1_prime: rs1.into_prime(),
            imm2: uimm.extract_bitfield(3, 6),
            imm1: uimm.extract_bitfield(6, 8),
        }.into())
    }
    #[inline]
    pub fn c_addi(&mut self, rd: Register, imm: i8) -> Result<u16, &str> {
        Ok(CItype{
            opcode: 0b00,
            funct3: 0b000,
            rd_rs1: rd.into_prime(),
            imm2: imm.extract_bitfield(5, 6) as _,
            imm1: imm.extract_bitfield(0, 5) as _,
        }.into())
    }
    #[inline]
    pub fn c_addiw(&mut self, rd: Register, imm: i8) -> Result<u16, &str> {
        Ok(CItype{
            opcode: 0b00,
            funct3: 0b001,
            rd_rs1: rd.into_prime(),
            imm2: imm.extract_bitfield(5, 6) as _,
            imm1: imm.extract_bitfield(0, 5) as _,
        }.into())
    }
    #[inline]
    pub fn c_li(&mut self, rd: Register, imm: i8) -> Result<u16, &str> {
        Ok(CItype{
            opcode: 0b00,
            funct3: 0b100,
            rd_rs1: rd.into_prime(),
            imm2: imm.extract_bitfield(5, 6) as _,
            imm1: imm.extract_bitfield(0, 5) as _,
        }.into())
    }
    #[inline]
    pub fn c_addi16sp(&mut self, imm: i8) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_lui(&mut self, rd: Register, imm: i8) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_srli(&mut self, rd: Register, uimm: u8) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_srai(&mut self, rd: Register, uimm: u8) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_andi(&mut self, rd: Register, uimm: u8) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_sub(&mut self, rd: Register, rs2: Register) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_xor(&mut self, rd: Register, rs2: Register) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_or(&mut self, rd: Register, rs2: Register) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_and(&mut self, rd: Register, rs2: Register) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_subw(&mut self, rd: Register, rs2: Register) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_addw(&mut self, rd: Register, rs2: Register) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_j(&mut self, imm: i16) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_beqz(&mut self, rs1: Register, offset: i16) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_bnez(&mut self, rs1: Register, offset: i16) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_slli(&mut self, rd: Register, uimm: u8) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_fldsp(&mut self, rd: FloatRegister, uimm: u8) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_lwsp(&mut self, rd: Register, uimm: u8) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_flwsp(&mut self, rd: Register, uimm: u8) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_ldsp(&mut self, rd: Register, uimm: u8) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_jr(&mut self, rs1: Register) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_mv(&mut self, rs1: Register, rs2: Register) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_jalr(&mut self, rs1: Register) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_add(&mut self, rd: Register, rs2: Register) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_fsdsp(&mut self, rd: Register, rs2: Register) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_swsp(&mut self, rs2: Register, uimm: u8) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_fswsp(&mut self, rs2: FloatRegister, uimm: u8) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }
    #[inline]
    pub fn c_sdsp(&mut self, rs2: Register, uimm: u8) -> Result<u16, &str> {
        todo!();
        Ok(2)
    }

    #[inline]
	pub fn fence(&mut self) -> Result<u32, &str>{
        todo!();
        Ok(4)
    }
    #[inline]
	pub fn fence_i(&mut self) -> Result<u32, &str>{
        todo!();
        Ok(4)
    }
    #[inline]
	pub fn ecall(&mut self) -> Result<u32, &str>{
        Ok(0b000000000000_0000_000_00000_1110011)
    }
    #[inline]
	pub fn ebreak(&mut self) -> Result<u32, &str>{
        Ok(0b000000000001_0000_000_00000_1110011)
    }
    #[inline]
	pub fn c_nop(&mut self) -> Result<u16, &str>{
        Ok(0b000_0_00000_00001_01)
    }
    #[inline]
	pub fn c_ebreak(&mut self) -> Result<u16, &str>{
        Ok(0b100_1_00000_00000_10)
    }
}
