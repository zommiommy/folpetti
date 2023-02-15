use super::{Register, FloatRegister, FloatRoundingMode};
use super::RV64GCUser;

pub struct AssemblerRV64GC;

impl RV64GCUser<u32> for AssemblerRV64GC {
    type Error = &'static str;
    #[inline]
    fn lui(&mut self, rd: Register, imm: u32) -> Result<u32, Self::Error> {
        if (imm & 0b1111_1111_1111) != 0 {
            return Err("the lower 12 bits of the LUI offset have to be zero!");
        }
        Ok(0b0110111_u32 | imm | ((rd as u32) << 6))
    }
    #[inline]
    fn auipc(&mut self, rd: Register, imm: u32) -> Result<u32, Self::Error> {
        if (imm & 0b1111_1111_1111) != 0 {
            return Err("the lower 12 bits of the AUIPC offset have to be zero!");
        }
        Ok(0b0010111_u32 | imm | ((rd as u32) << 6))
    }
    #[inline]
    fn addi(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, Self::Error> {
        todo!();
        // here
        Ok(4)
    }
    #[inline]
    fn slti(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn sltiu(&mut self, rd: Register, rs1: Register, imm: u32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn xori(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn ori(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn andi(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn slli(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn srli(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<u32, Self::Error> {
        todo!();
        // here
        Ok(4)
    }
    #[inline]
    fn srai(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn add(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn sub(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        // here
        Ok(4)
    }
    #[inline]
    fn sll(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn slt(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn sltu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn xor(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn srl(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn sra(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn or(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn and(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn csrrw(&mut self, rd: Register, rs1: Register, offset: u32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn csrrs(&mut self, rd: Register, rs1: Register, offset: u32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn csrrc(&mut self, rd: Register, rs1: Register, offset: u32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn csrrwi(&mut self, rd: Register, zimm: u8, offset: u32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn csrrsi(&mut self, rd: Register, zimm: u8, offset: u32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn csrrci(&mut self, rd: Register, zimm: u8, offset: u32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn lb(&mut self, rd: Register, imm: u64) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn lh(&mut self, rd: Register, imm: u64) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn lw(&mut self, rd: Register, imm: u64) -> Result<u32, Self::Error> {
        todo!();
        // here
        Ok(4)
    }
    #[inline]
    fn ld(&mut self, rd: Register, imm: u64) -> Result<u32, Self::Error> {
        todo!();
        // here
        Ok(4)
    }
    #[inline]
    fn lbu(&mut self, rd: Register, imm: u64) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn lhu(&mut self, rd: Register, imm: u64) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn lwu(&mut self, rd: Register, imm: u64) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn sb(&mut self, rs1: Register, rs2: Register, imm: u64) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn sh(&mut self, rs1: Register, rs2: Register, imm: u64) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn sw(&mut self, rs1: Register, rs2: Register, imm: u64) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn sd(&mut self, rs1: Register, rs2: Register, imm: u64) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn jal(&mut self, rd: Register, imm: i32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn jalr(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<u32, Self::Error> {
        todo!();
        // here
        Ok(4)
    }
    #[inline]
    fn beq(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn bne(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn blt(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn bge(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn bltu(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn bgeu(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn addiw(&mut self, rd: Register, rsq: Register, imm: u32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn slliw(&mut self, rd: Register, rsq: Register, shamt: i32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn srliw(&mut self, rd: Register, rsq: Register, shamt: i32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn sraiw(&mut self, rd: Register, rsq: Register, shamt: i32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn addw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn subw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn sllw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn srlw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn sraw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn mul(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn mulh(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn mulhsu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn mulhu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn div(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn divu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn rem(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn remu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn mulw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn divw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn divuw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn remw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn remuw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fmadd_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fmsub_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fnmsub_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fnmadd_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fadd_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fsub_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fmul_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fdiv_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fsqrt_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fsgnj_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fsgnjn_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fsgnjx_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fmin_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fmax_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_w_s(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_wu_s(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fmv_x_w(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn feq_s(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn flt_s(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fle_s(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fclass_s(&mut self, rd: Register, rs1: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_s_w(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_s_wu(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fmv_w_x(&mut self, rd: FloatRegister, rs1: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fmadd_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fmsub_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fnmsub_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fnmadd_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fadd_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fsub_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fmul_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fdiv_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fsqrt_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fsgnj_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fsgnjn_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fsgnjx_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fmin_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fmax_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_s_d(&mut self, rd: FloatRegister, rs1: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_d_s(&mut self, rd: FloatRegister, rs1: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn feq_d(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn flt_d(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fle_d(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fclass_d(&mut self, rd: Register, rs1: FloatRegister) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_w_d(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_wu_d(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_d_w(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_d_wu(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn flw(&mut self, rd: FloatRegister, rs1: Register, imm: u32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fsw(&mut self, rs1: Register, rs2: FloatRegister, offset: u32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fld(&mut self, rd: FloatRegister, rs1: Register, offset: u32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fsd(&mut self, rs1: Register, rs2: FloatRegister, offset: u32) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_l_s(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_lu_s(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_s_l(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_s_lu(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_l_d(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_lu_d(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fmv_x_d(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_d_l(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fcvt_d_lu(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn fmv_d_x(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<u32, Self::Error> {
        todo!();
        Ok(4)
    }
    #[inline]
    fn c_addi4spn(&mut self, rd: Register, uimm: u16) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_fld(&mut self, rd: Register, rs1: Register, imm: u16) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_lw(&mut self, rd: Register, rs1: Register, uimm: u16) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_flw(&mut self, rd: FloatRegister, rs1: Register, uimm: u16) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_ld(&mut self, rd: Register, rs1: Register, uimm: u16) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_fsd(&mut self, rs1: Register, rs2: FloatRegister, uimm: u16) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_sw(&mut self, rs1: Register, rs2: Register, uimm: u16) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_fsw(&mut self, rs1: Register, rs2: FloatRegister, uimm: u8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_sd(&mut self, rs1: Register, rs2: Register, uimm: u16) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_addi(&mut self, rd: Register, imm: i8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_jal(&mut self, imm: u16) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_addiw(&mut self, rd: Register, imm: i8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_li(&mut self, rd: Register, imm: i8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_addi16sp(&mut self, imm: i8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_lui(&mut self, rd: Register, imm: i8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_srli(&mut self, rd: Register, uimm: u8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_srai(&mut self, rd: Register, uimm: u8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_andi(&mut self, rd: Register, uimm: u8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_sub(&mut self, rd: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_xor(&mut self, rd: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_or(&mut self, rd: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_and(&mut self, rd: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_subw(&mut self, rd: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_addw(&mut self, rd: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_j(&mut self, imm: i16) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_beqz(&mut self, rs1: Register, offset: i16) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_bnez(&mut self, rs1: Register, offset: i16) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_slli(&mut self, rd: Register, uimm: u8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_fldsp(&mut self, rd: FloatRegister, uimm: u8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_lwsp(&mut self, rd: Register, uimm: u8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_flwsp(&mut self, rd: Register, uimm: u8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_ldsp(&mut self, rd: Register, uimm: u8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_jr(&mut self, rs1: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_mv(&mut self, rs1: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_jalr(&mut self, rs1: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_add(&mut self, rd: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_fsdsp(&mut self, rd: Register, rs2: Register) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_swsp(&mut self, rs2: Register, uimm: u8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_fswsp(&mut self, rs2: FloatRegister, uimm: u8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }
    #[inline]
    fn c_sdsp(&mut self, rs2: Register, uimm: u8) -> Result<u32, Self::Error> {
        todo!();
        Ok(2)
    }

    #[inline]
	fn fence(&mut self) -> Result<u32, Self::Error>{
        todo!();
        Ok(4)
    }
    #[inline]
	fn fence_i(&mut self) -> Result<u32, Self::Error>{
        todo!();
        Ok(4)
    }
    #[inline]
	fn ecall(&mut self) -> Result<u32, Self::Error>{
        Ok(0b000000000000_0000_000_00000_1110011)
    }
    #[inline]
	fn ebreak(&mut self) -> Result<u32, Self::Error>{
        Ok(0b000000000001_0000_000_00000_1110011)
    }
    #[inline]
	fn c_nop(&mut self) -> Result<u32, Self::Error>{
        Ok(0b000_0_00000_00001_01)
    }
    #[inline]
	fn c_ebreak(&mut self) -> Result<u32, Self::Error>{
        Ok(0b100_1_00000_00000_10)
    }
}
