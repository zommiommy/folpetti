use diss::riscv64gc::*;
use mmu::{Mmu, VirtAddr, MmuError};

#[derive(Debug)]
pub enum CoreEmuError {
    /// the system called a syscall :)
    Syscall,
    /// The execution hitted a breakpoint
    Breakpoint,
    /// Oh-oh a memory error! yays
    MmuError(MmuError),
    /// Buuuu, can't write to the zero reg
    RegWrite,
    /// Yield execution, for multithreading mainly
    Yield,
}

pub struct CoreEmu {
    pub regs: [u64; 33],
    pub mem: Mmu,
}

impl CoreEmu {
    pub fn read_reg(&self, reg: Register) -> u64 {
        self.regs[reg as usize]
    }

    pub fn write_reg(&mut self, reg: Register, value: u64) -> Result<(), CoreEmuError> {
        if reg == Register::Zero {
            return Err(CoreEmuError::RegWrite);
        }

        self.regs[reg as usize] = value;

        Ok(())
    }

    pub fn run(&mut self) -> CoreEmuError {
        loop {
            let inst = self.mem.read(
                VirtAddr(self.read_reg(Register::Pc) as usize)
            );
            
            // TODO!: simplify this poopoo
            if let Err(e) = inst {
                return CoreEmuError::MmuError(e);
            }
            let inst = inst.unwrap();
    
            if let Err(e) = diss_riscv64gc(self, inst) {
                return e;
            }
        }
    }
}

impl RV64GCUser<()> for CoreEmu {
    type Error = CoreEmuError;
    fn lui(&mut self, rd: Register, imm: u32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn auipc(&mut self, rd: Register, imm: u32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn addi(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn slti(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn sltiu(&mut self, rd: Register, rs1: Register, imm: u32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn xori(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn ori(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn andi(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn slli(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn srli(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn srai(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn add(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn sub(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn sll(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn slt(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn sltu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn xor(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn srl(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn sra(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn or(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn and(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn csrrw(&mut self, rd: Register, rs1: Register, offset: u32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn csrrs(&mut self, rd: Register, rs1: Register, offset: u32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn csrrc(&mut self, rd: Register, rs1: Register, offset: u32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn csrrwi(&mut self, rd: Register, zimm: u8, offset: u32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn csrrsi(&mut self, rd: Register, zimm: u8, offset: u32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn csrrci(&mut self, rd: Register, zimm: u8, offset: u32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn lb(&mut self, rd: Register, imm: u64) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn lh(&mut self, rd: Register, imm: u64) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn lw(&mut self, rd: Register, imm: u64) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn ld(&mut self, rd: Register, imm: u64) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn lbu(&mut self, rd: Register, imm: u64) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn lhu(&mut self, rd: Register, imm: u64) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn lwu(&mut self, rd: Register, imm: u64) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn sb(&mut self, rs1: Register, rs2: Register, imm: u64) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn sh(&mut self, rs1: Register, rs2: Register, imm: u64) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn sw(&mut self, rs1: Register, rs2: Register, imm: u64) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn sd(&mut self, rs1: Register, rs2: Register, imm: u64) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn jal(&mut self, rd: Register, imm: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn jalr(&mut self, rd: Register, imm: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn beq(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn bne(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn blt(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn bge(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn bltu(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn bgeu(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn addiw(&mut self, rd: Register, rsq: Register, imm: u32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn slliw(&mut self, rd: Register, rsq: Register, shamt: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn srliw(&mut self, rd: Register, rsq: Register, shamt: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn sraiw(&mut self, rd: Register, rsq: Register, shamt: i32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn addw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn subw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn sllw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn srlw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn sraw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn mul(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn mulh(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn mulhsu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn mulhu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn div(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn divu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn rem(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn remu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn mulw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn divw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn divuw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn remw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn remuw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fmadd_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rs3: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fmsub_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rs3: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fnmsub_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rs3: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fnmadd_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rs3: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fadd_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fsub_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fmul_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fdiv_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fsqrt_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fsgnj_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fsgnjn_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fsgnjx_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fmin_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fmax_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_w_s(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_wu_s(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fmv_x_w(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn feq_s(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn flt_s(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fle_s(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fclass_s(&mut self, rd: Register, rs1: FloatRegister) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_s_w(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_s_wu(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fmv_w_x(&mut self, rd: FloatRegister, rs1: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fmadd_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rs3: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fmsub_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rs3: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fnmsub_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rs3: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fnmadd_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rs3: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fadd_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fsub_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fmul_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fdiv_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fsqrt_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fsgnj_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fsgnjn_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fsgnjx_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fmin_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fmax_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_s_d(&mut self, rd: FloatRegister, rs1: FloatRegister) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_d_s(&mut self, rd: FloatRegister, rs1: FloatRegister) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn feq_d(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn flt_d(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fle_d(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fclass_d(&mut self, rd: Register, rs1: FloatRegister) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_w_d(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_wu_d(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_d_w(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_d_wu(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn flw(&mut self, rd: FloatRegister, rs1: Register, imm: u32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fsw(
        &mut self,
        rs1: Register,
        rs2: FloatRegister,
        offset: u32,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fld(&mut self, rd: FloatRegister, rs1: Register, offset: u32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fsd(
        &mut self,
        rs1: Register,
        rs2: FloatRegister,
        offset: u32,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_l_s(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_lu_s(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_s_l(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_s_lu(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_l_d(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_lu_d(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fmv_x_d(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_d_l(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fcvt_d_lu(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fmv_d_x(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_addi4spn(&mut self, rd: Register, uimm: u16) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_fld(&mut self, rd: Register, rs1: Register, imm: u16) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_lw(&mut self, rd: Register, rs1: Register, uimm: u16) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_flw(&mut self, rd: FloatRegister, rs1: Register, uimm: u16) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_ld(&mut self, rd: Register, rs1: FloatRegister, uimm: u16) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_fsd(
        &mut self,
        rs1: Register,
        rs2: FloatRegister,
        uimm: u16,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_sw(&mut self, rs1: Register, rs2: Register, uimm: u16) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_fsw(&mut self, rs1: Register, rs2: FloatRegister, uimm: u8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_sd(&mut self, rs1: Register, rs2: Register, uimm: u16) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_addi(&mut self, rd: Register, imm: i8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_jal(&mut self, imm: u16) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_addiw(&mut self, rd: Register, imm: i8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_li(&mut self, rd: Register, imm: i8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_addi16sp(&mut self, imm: i8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_lui(&mut self, rd: Register, imm: i8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_srli(&mut self, rd: Register, uimm: u8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_srai(&mut self, rd: Register, uimm: u8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_andi(&mut self, rd: Register, uimm: u8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_sub(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_xor(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_or(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_and(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_subw(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_addw(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_j(&mut self, imm: i16) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_beqz(&mut self, rs1: Register, offset: u16) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_bnez(&mut self, rs1: Register, offset: u16) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_slli(&mut self, rd: Register, uimm: u8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_fldsp(&mut self, rd: FloatRegister, uimm: u8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_lwsp(&mut self, rd: Register, uimm: u8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_flwsp(&mut self, rd: Register, uimm: u8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_ldsp(&mut self, rd: Register, uimm: u8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_jr(&mut self, rs1: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_mv(&mut self, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_jalr(&mut self, rs1: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_add(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_fsdsp(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_swsp(&mut self, rs2: Register, uimm: u8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_fswsp(&mut self, rs2: FloatRegister, uimm: u8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_sdsp(&mut self, rs2: Register, uimm: u8) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn fence(&mut self) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    fn fence_i(&mut self) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    fn ecall(&mut self) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    fn ebreak(&mut self) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    fn c_nop(&mut self) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    fn c_ebreak(&mut self) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
}