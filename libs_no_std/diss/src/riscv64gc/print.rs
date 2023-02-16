use std::convert::Infallible;

use super::{Register, FloatRegister, FloatRoundingMode};
use super::RV64GCUser;

pub struct RV64GCPrint;

impl RV64GCUser<usize, usize> for RV64GCPrint {
    type Error = Infallible;
    fn lui(&mut self, rd: Register, imm: u32) -> Result<usize, Self::Error> {
        println!("lui {:?} {}", rd, imm);
        Ok(4)
    }
    
    fn auipc(&mut self, rd: Register, imm: u32) -> Result<usize, Self::Error> {
        println!("auipc {:?} {}", rd, imm);
        Ok(4)
    }
    
    fn addi(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<usize, Self::Error> {
        println!("addi {:?} {:?} {}", rd, rs1, imm);
        Ok(4)
    }
    
    fn slti(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<usize, Self::Error> {
        println!("slti {:?} {:?} {}", rd, rs1, imm);
        Ok(4)
    }
    
    fn sltiu(&mut self, rd: Register, rs1: Register, imm: u32) -> Result<usize, Self::Error> {
        println!("sltiu {:?} {:?} {}", rd, rs1, imm);
        Ok(4)
    }
    
    fn xori(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<usize, Self::Error> {
        println!("xori {:?} {:?} {}", rd, rs1, imm);
        Ok(4)
    }
    
    fn ori(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<usize, Self::Error> {
        println!("ori {:?} {:?} {}", rd, rs1, imm);
        Ok(4)
    }
    
    fn andi(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<usize, Self::Error> {
        println!("andi {:?} {:?} {}", rd, rs1, imm);
        Ok(4)
    }
    
    fn slli(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<usize, Self::Error> {
        println!("slli {:?} {:?} {}", rd, rs1, shamt);
        Ok(4)
    }
    
    fn srli(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<usize, Self::Error> {
        println!("srli {:?} {:?} {}", rd, rs1, shamt);
        Ok(4)
    }
    
    fn srai(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<usize, Self::Error> {
        println!("srai {:?} {:?} {}", rd, rs1, shamt);
        Ok(4)
    }
    
    fn add(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("add {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn sub(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("sub {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn sll(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("sll {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn slt(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("slt {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn sltu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("sltu {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn xor(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("xor {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn srl(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("srl {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn sra(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("sra {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn or(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("or {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn and(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("and {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn csrrw(&mut self, rd: Register, rs1: Register, offset: u32) -> Result<usize, Self::Error> {
        println!("csrrw {:?} {:?} {}", rd, rs1, offset);
        Ok(4)
    }
    
    fn csrrs(&mut self, rd: Register, rs1: Register, offset: u32) -> Result<usize, Self::Error> {
        println!("csrrs {:?} {:?} {}", rd, rs1, offset);
        Ok(4)
    }
    
    fn csrrc(&mut self, rd: Register, rs1: Register, offset: u32) -> Result<usize, Self::Error> {
        println!("csrrc {:?} {:?} {}", rd, rs1, offset);
        Ok(4)
    }
    
    fn csrrwi(&mut self, rd: Register, zimm: u8, offset: u32) -> Result<usize, Self::Error> {
        println!("csrrwi {:?} {} {}", rd, zimm, offset);
        Ok(4)
    }
    
    fn csrrsi(&mut self, rd: Register, zimm: u8, offset: u32) -> Result<usize, Self::Error> {
        println!("csrrsi {:?} {} {}", rd, zimm, offset);
        Ok(4)
    }
    
    fn csrrci(&mut self, rd: Register, zimm: u8, offset: u32) -> Result<usize, Self::Error> {
        println!("csrrci {:?} {} {}", rd, zimm, offset);
        Ok(4)
    }
    
    fn lb(&mut self, rd: Register, imm: u64) -> Result<usize, Self::Error> {
        println!("lb {:?} {}", rd, imm);
        Ok(4)
    }
    
    fn lh(&mut self, rd: Register, imm: u64) -> Result<usize, Self::Error> {
        println!("lh {:?} {}", rd, imm);
        Ok(4)
    }
    
    fn lw(&mut self, rd: Register, imm: u64) -> Result<usize, Self::Error> {
        println!("lw {:?} {}", rd, imm);
        Ok(4)
    }
    
    fn ld(&mut self, rd: Register, imm: u64) -> Result<usize, Self::Error> {
        println!("ld {:?} {}", rd, imm);
        Ok(4)
    }
    
    fn lbu(&mut self, rd: Register, imm: u64) -> Result<usize, Self::Error> {
        println!("lbu {:?} {}", rd, imm);
        Ok(4)
    }
    
    fn lhu(&mut self, rd: Register, imm: u64) -> Result<usize, Self::Error> {
        println!("lhu {:?} {}", rd, imm);
        Ok(4)
    }
    
    fn lwu(&mut self, rd: Register, imm: u64) -> Result<usize, Self::Error> {
        println!("lwu {:?} {}", rd, imm);
        Ok(4)
    }
    
    fn sb(&mut self, rs1: Register, rs2: Register, imm: u64) -> Result<usize, Self::Error> {
        println!("sb {:?} {:?} {}", rs1, rs2, imm);
        Ok(4)
    }
    
    fn sh(&mut self, rs1: Register, rs2: Register, imm: u64) -> Result<usize, Self::Error> {
        println!("sh {:?} {:?} {}", rs1, rs2, imm);
        Ok(4)
    }
    
    fn sw(&mut self, rs1: Register, rs2: Register, imm: u64) -> Result<usize, Self::Error> {
        println!("sw {:?} {:?} {}", rs1, rs2, imm);
        Ok(4)
    }
    
    fn sd(&mut self, rs1: Register, rs2: Register, imm: u64) -> Result<usize, Self::Error> {
        println!("sd {:?} {:?} {}", rs1, rs2, imm);
        Ok(4)
    }
    
    fn jal(&mut self, rd: Register, imm: i32) -> Result<usize, Self::Error> {
        println!("jal {:?} {}", rd, imm);
        Ok(4)
    }
    
    fn jalr(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<usize, Self::Error> {
        println!("jalr {:?} {:?} {}", rd, rs1, imm);
        Ok(4)
    }
    
    fn beq(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<usize, Self::Error> {
        println!("beq {:?} {:?} {}", rs1, rs2, imm);
        Ok(4)
    }
    
    fn bne(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<usize, Self::Error> {
        println!("bne {:?} {:?} {}", rs1, rs2, imm);
        Ok(4)
    }
    
    fn blt(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<usize, Self::Error> {
        println!("blt {:?} {:?} {}", rs1, rs2, imm);
        Ok(4)
    }
    
    fn bge(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<usize, Self::Error> {
        println!("bge {:?} {:?} {}", rs1, rs2, imm);
        Ok(4)
    }
    
    fn bltu(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<usize, Self::Error> {
        println!("bltu {:?} {:?} {}", rs1, rs2, imm);
        Ok(4)
    }
    
    fn bgeu(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<usize, Self::Error> {
        println!("bgeu {:?} {:?} {}", rs1, rs2, imm);
        Ok(4)
    }
    
    fn addiw(&mut self, rd: Register, rsq: Register, imm: u32) -> Result<usize, Self::Error> {
        println!("addiw {:?} {:?} {}", rd, rsq, imm);
        Ok(4)
    }
    
    fn slliw(&mut self, rd: Register, rsq: Register, shamt: i32) -> Result<usize, Self::Error> {
        println!("slliw {:?} {:?} {}", rd, rsq, shamt);
        Ok(4)
    }
    
    fn srliw(&mut self, rd: Register, rsq: Register, shamt: i32) -> Result<usize, Self::Error> {
        println!("srliw {:?} {:?} {}", rd, rsq, shamt);
        Ok(4)
    }
    
    fn sraiw(&mut self, rd: Register, rsq: Register, shamt: i32) -> Result<usize, Self::Error> {
        println!("sraiw {:?} {:?} {}", rd, rsq, shamt);
        Ok(4)
    }
    
    fn addw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("addw {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn subw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("subw {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn sllw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("sllw {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn srlw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("srlw {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn sraw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("sraw {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn mul(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("mul {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn mulh(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("mulh {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn mulhsu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("mulhsu {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn mulhu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("mulhu {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn div(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("div {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn divu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("divu {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn rem(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("rem {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn remu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("remu {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn mulw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("mulw {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn divw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("divw {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn divuw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("divuw {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn remw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("remw {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn remuw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("remuw {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn fmadd_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fmadd_s {:?} {:?} {:?} {:?} {:?}", rd, rs1, rs2, rs3, rm);
        Ok(4)
    }
    
    fn fmsub_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fmsub_s {:?} {:?} {:?} {:?} {:?}", rd, rs1, rs2, rs3, rm);
        Ok(4)
    }
    
    fn fnmsub_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fnmsub_s {:?} {:?} {:?} {:?} {:?}", rd, rs1, rs2, rs3, rm);
        Ok(4)
    }
    
    fn fnmadd_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fnmadd_s {:?} {:?} {:?} {:?} {:?}", rd, rs1, rs2, rs3, rm);
        Ok(4)
    }
    
    fn fadd_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fadd_s {:?} {:?} {:?} {:?}", rd, rs1, rs2, rm);
        Ok(4)
    }
    
    fn fsub_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fsub_s {:?} {:?} {:?} {:?}", rd, rs1, rs2, rm);
        Ok(4)
    }
    
    fn fmul_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fmul_s {:?} {:?} {:?} {:?}", rd, rs1, rs2, rm);
        Ok(4)
    }
    
    fn fdiv_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fdiv_s {:?} {:?} {:?} {:?}", rd, rs1, rs2, rm);
        Ok(4)
    }
    
    fn fsqrt_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fsqrt_s {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fsgnj_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<usize, Self::Error> {
        println!("fsgnj_s {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn fsgnjn_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<usize, Self::Error> {
        println!("fsgnjn_s {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn fsgnjx_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<usize, Self::Error> {
        println!("fsgnjx_s {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn fmin_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<usize, Self::Error> {
        println!("fmin_s {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn fmax_s(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<usize, Self::Error> {
        println!("fmax_s {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn fcvt_w_s(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fcvt_w_s {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fcvt_wu_s(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fcvt_wu_s {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fmv_x_w(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fmv_x_w {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn feq_s(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<usize, Self::Error> {
        println!("feq_s {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn flt_s(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<usize, Self::Error> {
        println!("flt_s {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn fle_s(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<usize, Self::Error> {
        println!("fle_s {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn fclass_s(&mut self, rd: Register, rs1: FloatRegister) -> Result<usize, Self::Error> {
        println!("fclass_s {:?} {:?}", rd, rs1);
        Ok(4)
    }
    
    fn fcvt_s_w(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fcvt_s_w {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fcvt_s_wu(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fcvt_s_wu {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fmv_w_x(&mut self, rd: FloatRegister, rs1: Register) -> Result<usize, Self::Error> {
        println!("fmv_w_x {:?} {:?}", rd, rs1);
        Ok(4)
    }
    
    fn fmadd_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fmadd_d {:?} {:?} {:?} {:?} {:?}", rd, rs1, rs2, rs3, rm);
        Ok(4)
    }
    
    fn fmsub_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fmsub_d {:?} {:?} {:?} {:?} {:?}", rd, rs1, rs2, rs3, rm);
        Ok(4)
    }
    
    fn fnmsub_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fnmsub_d {:?} {:?} {:?} {:?} {:?}", rd, rs1, rs2, rs3, rm);
        Ok(4)
    }
    
    fn fnmadd_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fnmadd_d {:?} {:?} {:?} {:?} {:?}", rd, rs1, rs2, rs3, rm);
        Ok(4)
    }
    
    fn fadd_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fadd_d {:?} {:?} {:?} {:?}", rd, rs1, rs2, rm);
        Ok(4)
    }
    
    fn fsub_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fsub_d {:?} {:?} {:?} {:?}", rd, rs1, rs2, rm);
        Ok(4)
    }
    
    fn fmul_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fmul_d {:?} {:?} {:?} {:?}", rd, rs1, rs2, rm);
        Ok(4)
    }
    
    fn fdiv_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fdiv_d {:?} {:?} {:?} {:?}", rd, rs1, rs2, rm);
        Ok(4)
    }
    
    fn fsqrt_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fsqrt_d {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fsgnj_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<usize, Self::Error> {
        println!("fsgnj_d {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn fsgnjn_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<usize, Self::Error> {
        println!("fsgnjn_d {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn fsgnjx_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<usize, Self::Error> {
        println!("fsgnjx_d {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn fmin_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<usize, Self::Error> {
        println!("fmin_d {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn fmax_d(&mut self, rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister) -> Result<usize, Self::Error> {
        println!("fmax_d {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn fcvt_s_d(&mut self, rd: FloatRegister, rs1: FloatRegister) -> Result<usize, Self::Error> {
        println!("fcvt_s_d {:?} {:?}", rd, rs1);
        Ok(4)
    }
    
    fn fcvt_d_s(&mut self, rd: FloatRegister, rs1: FloatRegister) -> Result<usize, Self::Error> {
        println!("fcvt_d_s {:?} {:?}", rd, rs1);
        Ok(4)
    }
    
    fn feq_d(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<usize, Self::Error> {
        println!("feq_d {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn flt_d(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<usize, Self::Error> {
        println!("flt_d {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn fle_d(&mut self, rd: Register, rs1: FloatRegister, rs2: FloatRegister) -> Result<usize, Self::Error> {
        println!("fle_d {:?} {:?} {:?}", rd, rs1, rs2);
        Ok(4)
    }
    
    fn fclass_d(&mut self, rd: Register, rs1: FloatRegister) -> Result<usize, Self::Error> {
        println!("fclass_d {:?} {:?}", rd, rs1);
        Ok(4)
    }
    
    fn fcvt_w_d(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fcvt_w_d {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fcvt_wu_d(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fcvt_wu_d {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fcvt_d_w(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fcvt_d_w {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fcvt_d_wu(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fcvt_d_wu {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn flw(&mut self, rd: FloatRegister, rs1: Register, imm: u32) -> Result<usize, Self::Error> {
        println!("flw {:?} {:?} {}", rd, rs1, imm);
        Ok(4)
    }
    
    fn fsw(&mut self, rs1: Register, rs2: FloatRegister, offset: u32) -> Result<usize, Self::Error> {
        println!("fsw {:?} {:?} {}", rs1, rs2, offset);
        Ok(4)
    }
    
    fn fld(&mut self, rd: FloatRegister, rs1: Register, offset: u32) -> Result<usize, Self::Error> {
        println!("fld {:?} {:?} {}", rd, rs1, offset);
        Ok(4)
    }
    
    fn fsd(&mut self, rs1: Register, rs2: FloatRegister, offset: u32) -> Result<usize, Self::Error> {
        println!("fsd {:?} {:?} {}", rs1, rs2, offset);
        Ok(4)
    }
    
    fn fcvt_l_s(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fcvt_l_s {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fcvt_lu_s(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fcvt_lu_s {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fcvt_s_l(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fcvt_s_l {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fcvt_s_lu(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fcvt_s_lu {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fcvt_l_d(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fcvt_l_d {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fcvt_lu_d(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fcvt_lu_d {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fmv_x_d(&mut self, rd: Register, rs1: FloatRegister, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fmv_x_d {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fcvt_d_l(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fcvt_d_l {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fcvt_d_lu(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fcvt_d_lu {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn fmv_d_x(&mut self, rd: FloatRegister, rs1: Register, rm: FloatRoundingMode) -> Result<usize, Self::Error> {
        println!("fmv_d_x {:?} {:?} {:?}", rd, rs1, rm);
        Ok(4)
    }
    
    fn c_addi4spn(&mut self, rd: Register, uimm: u16) -> Result<usize, Self::Error> {
        println!("c_addi4spn {:?} {}", rd, uimm);
        Ok(2)
    }
    
    fn c_fld(&mut self, rd: Register, rs1: Register, imm: u16) -> Result<usize, Self::Error> {
        println!("c_fld {:?} {:?} {}", rd, rs1, imm);
        Ok(2)
    }
    
    fn c_lw(&mut self, rd: Register, rs1: Register, uimm: u16) -> Result<usize, Self::Error> {
        println!("c_lw {:?} {:?} {}", rd, rs1, uimm);
        Ok(2)
    }
    
    fn c_flw(&mut self, rd: FloatRegister, rs1: Register, uimm: u16) -> Result<usize, Self::Error> {
        println!("c_flw {:?} {:?} {}", rd, rs1, uimm);
        Ok(2)
    }
    
    fn c_ld(&mut self, rd: Register, rs1: Register, uimm: u16) -> Result<usize, Self::Error> {
        println!("c_ld {:?} {:?} {}", rd, rs1, uimm);
        Ok(2)
    }
    
    fn c_fsd(&mut self, rs1: Register, rs2: FloatRegister, uimm: u16) -> Result<usize, Self::Error> {
        println!("c_fsd {:?} {:?} {}", rs1, rs2, uimm);
        Ok(2)
    }
    
    fn c_sw(&mut self, rs1: Register, rs2: Register, uimm: u16) -> Result<usize, Self::Error> {
        println!("c_sw {:?} {:?} {}", rs1, rs2, uimm);
        Ok(2)
    }
    
    fn c_fsw(&mut self, rs1: Register, rs2: FloatRegister, uimm: u8) -> Result<usize, Self::Error> {
        println!("c_fsw {:?} {:?} {}", rs1, rs2, uimm);
        Ok(2)
    }
    
    fn c_sd(&mut self, rs1: Register, rs2: Register, uimm: u16) -> Result<usize, Self::Error> {
        println!("c_sd {:?} {:?} {}", rs1, rs2, uimm);
        Ok(2)
    }
    
    fn c_addi(&mut self, rd: Register, imm: i8) -> Result<usize, Self::Error> {
        println!("c_addi {:?} {}", rd, imm);
        Ok(2)
    }
    
    fn c_jal(&mut self, imm: u16) -> Result<usize, Self::Error> {
        println!("c_jal {}", imm);
        Ok(2)
    }
    
    fn c_addiw(&mut self, rd: Register, imm: i8) -> Result<usize, Self::Error> {
        println!("c_addiw {:?} {}", rd, imm);
        Ok(2)
    }
    
    fn c_li(&mut self, rd: Register, imm: i8) -> Result<usize, Self::Error> {
        println!("c_li {:?} {}", rd, imm);
        Ok(2)
    }
    
    fn c_addi16sp(&mut self, imm: i8) -> Result<usize, Self::Error> {
        println!("c_addi16sp {}", imm);
        Ok(2)
    }
    
    fn c_lui(&mut self, rd: Register, imm: i8) -> Result<usize, Self::Error> {
        println!("c_lui {:?} {}", rd, imm);
        Ok(2)
    }
    
    fn c_srli(&mut self, rd: Register, uimm: u8) -> Result<usize, Self::Error> {
        println!("c_srli {:?} {}", rd, uimm);
        Ok(2)
    }
    
    fn c_srai(&mut self, rd: Register, uimm: u8) -> Result<usize, Self::Error> {
        println!("c_srai {:?} {}", rd, uimm);
        Ok(2)
    }
    
    fn c_andi(&mut self, rd: Register, uimm: u8) -> Result<usize, Self::Error> {
        println!("c_andi {:?} {}", rd, uimm);
        Ok(2)
    }
    
    fn c_sub(&mut self, rd: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("c_sub {:?} {:?}", rd, rs2);
        Ok(2)
    }
    
    fn c_xor(&mut self, rd: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("c_xor {:?} {:?}", rd, rs2);
        Ok(2)
    }
    
    fn c_or(&mut self, rd: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("c_or {:?} {:?}", rd, rs2);
        Ok(2)
    }
    
    fn c_and(&mut self, rd: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("c_and {:?} {:?}", rd, rs2);
        Ok(2)
    }
    
    fn c_subw(&mut self, rd: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("c_subw {:?} {:?}", rd, rs2);
        Ok(2)
    }
    
    fn c_addw(&mut self, rd: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("c_addw {:?} {:?}", rd, rs2);
        Ok(2)
    }
    
    fn c_j(&mut self, imm: i16) -> Result<usize, Self::Error> {
        println!("c_j {}", imm);
        Ok(2)
    }
    
    fn c_beqz(&mut self, rs1: Register, offset: u16) -> Result<usize, Self::Error> {
        println!("c_beqz {:?} {}", rs1, offset);
        Ok(2)
    }
    
    fn c_bnez(&mut self, rs1: Register, offset: u16) -> Result<usize, Self::Error> {
        println!("c_bnez {:?} {}", rs1, offset);
        Ok(2)
    }
    
    fn c_slli(&mut self, rd: Register, uimm: u8) -> Result<usize, Self::Error> {
        println!("c_slli {:?} {}", rd, uimm);
        Ok(2)
    }
    
    fn c_fldsp(&mut self, rd: FloatRegister, uimm: u8) -> Result<usize, Self::Error> {
        println!("c_fldsp {:?} {}", rd, uimm);
        Ok(2)
    }
    
    fn c_lwsp(&mut self, rd: Register, uimm: u8) -> Result<usize, Self::Error> {
        println!("c_lwsp {:?} {}", rd, uimm);
        Ok(2)
    }
    
    fn c_flwsp(&mut self, rd: Register, uimm: u8) -> Result<usize, Self::Error> {
        println!("c_flwsp {:?} {}", rd, uimm);
        Ok(2)
    }
    
    fn c_ldsp(&mut self, rd: Register, uimm: u8) -> Result<usize, Self::Error> {
        println!("c_ldsp {:?} {}", rd, uimm);
        Ok(2)
    }
    
    fn c_jr(&mut self, rs1: Register) -> Result<usize, Self::Error> {
        println!("c_jr {:?}", rs1);
        Ok(2)
    }
    
    fn c_mv(&mut self, rs1: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("c_mv {:?} {:?}", rs1, rs2);
        Ok(2)
    }
    
    fn c_jalr(&mut self, rs1: Register) -> Result<usize, Self::Error> {
        println!("c_jalr {:?}", rs1);
        Ok(2)
    }
    
    fn c_add(&mut self, rd: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("c_add {:?} {:?}", rd, rs2);
        Ok(2)
    }
    
    fn c_fsdsp(&mut self, rd: Register, rs2: Register) -> Result<usize, Self::Error> {
        println!("c_fsdsp {:?} {:?}", rd, rs2);
        Ok(2)
    }
    
    fn c_swsp(&mut self, rs2: Register, uimm: u8) -> Result<usize, Self::Error> {
        println!("c_swsp {:?} {}", rs2, uimm);
        Ok(2)
    }
    
    fn c_fswsp(&mut self, rs2: FloatRegister, uimm: u8) -> Result<usize, Self::Error> {
        println!("c_fswsp {:?} {}", rs2, uimm);
        Ok(2)
    }
    
    fn c_sdsp(&mut self, rs2: Register, uimm: u8) -> Result<usize, Self::Error> {
        println!("c_sdsp {:?} {}", rs2, uimm);
        Ok(2)
    }

	fn fence(&mut self) -> Result<usize, Self::Error>{println!("fence"); Ok(4)}
	fn fence_i(&mut self) -> Result<usize, Self::Error>{println!("fence_i"); Ok(4)}
	fn ecall(&mut self) -> Result<usize, Self::Error>{println!("ecall"); Ok(4)}
	fn ebreak(&mut self) -> Result<usize, Self::Error>{println!("ebreak"); Ok(4)}
	fn c_nop(&mut self) -> Result<usize, Self::Error>{println!("c_nop"); Ok(2)}
	fn c_ebreak(&mut self) -> Result<usize, Self::Error>{println!("c_ebreak"); Ok(2)}
}
