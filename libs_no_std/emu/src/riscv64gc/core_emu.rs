use core::intrinsics::unlikely;
use diss::riscv64gc::*;
use mmu::{Mmu, VirtAddr, MmuError};
use traits::{Word, Number};

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

impl From<MmuError> for CoreEmuError {
    fn from(value: MmuError) -> Self {
        CoreEmuError::MmuError(value)
    }
} 

pub struct CoreEmu {
    pub regs: [u64; 32],
    pub fregs: [f64; 32],
//  pub csr: u64,
//  pub fcsr: u64,
    pub pc: u64,
    pub mem: Mmu,
}

impl CoreEmu {
    pub fn new(mem: Mmu) -> Self {
        CoreEmu {
            regs: [0; 32],
            fregs: [0.0; 32],
            pc: 0,
            mem,
        }
    }

    #[cfg(feature="std")]
    pub fn debug(&self) {
        println!("PC: {:016x} Zero: {:016x}", 
            self.pc, 
            self.read_reg(Register::Zero),
        );
        println!(
            "Ra: {:016x} Sp: {:016x} Gp : {:016x} Tp : {:016x}", 
            self.read_reg(Register::Ra),
            self.read_reg(Register::Sp),
            self.read_reg(Register::Gp),
            self.read_reg(Register::Tp),
        );
        println!(
            "T0: {:016x} T1: {:016x} T2 : {:016x} T3 : {:016x}", 
            self.read_reg(Register::T0),
            self.read_reg(Register::T1),
            self.read_reg(Register::T2),
            self.read_reg(Register::T3),
        );
        println!(
            "T4: {:016x} T5: {:016x} T6 : {:016x}", 
            self.read_reg(Register::T4),
            self.read_reg(Register::T5),
            self.read_reg(Register::T6),
        );
        println!(
            "A0: {:016x} A1: {:016x} A2 : {:016x} A3 : {:016x}", 
            self.read_reg(Register::A0),
            self.read_reg(Register::A1),
            self.read_reg(Register::A2),
            self.read_reg(Register::A3),
        );
        println!(
            "A4: {:016x} A5: {:016x} A6 : {:016x} A7 : {:016x}", 
            self.read_reg(Register::A4),
            self.read_reg(Register::A5),
            self.read_reg(Register::A6),
            self.read_reg(Register::A7),
        );
        println!(
            "S0: {:016x} S1: {:016x} S2 : {:016x} S3 : {:016x}", 
            self.read_reg(Register::S0),
            self.read_reg(Register::S1),
            self.read_reg(Register::S2),
            self.read_reg(Register::S3),
        );
        println!(
            "S4: {:016x} S5: {:016x} S6 : {:016x} S7 : {:016x}", 
            self.read_reg(Register::S4),
            self.read_reg(Register::S5),
            self.read_reg(Register::S6),
            self.read_reg(Register::S7),
        );
        println!(
            "S8: {:016x} S9: {:016x} S10: {:016x} S11: {:016x}", 
            self.read_reg(Register::S8),
            self.read_reg(Register::S9),
            self.read_reg(Register::S10),
            self.read_reg(Register::S11),
        );
    }

    #[inline(always)]
    pub fn read_reg(&self, reg: Register) -> u64 {
        self.regs[reg as usize]
    }

    #[inline(always)]
    pub fn write_reg(&mut self, reg: Register, value: u64) -> Result<(), CoreEmuError> {
        if unlikely(reg == Register::Zero) {
            return Err(CoreEmuError::RegWrite);
        }

        self.regs[reg as usize] = value;

        Ok(())
    }

    #[inline(always)]
    pub fn read_freg(&self, reg: FloatRegister) -> f64 {
        self.fregs[reg as usize]
    }

    #[inline(always)]
    pub fn write_freg(&mut self, reg: FloatRegister, value: f64) {
        self.fregs[reg as usize] = value;
    }

    pub fn fork(&self) -> Self {
        Self {
            regs: self.regs,
            fregs: self.fregs,
            pc: self.pc,
            mem: self.mem.fork(),
        }
    }

    pub fn reset(&mut self, other: &Self) {
        self.regs = other.regs;
        self.fregs = other.fregs;
        self.pc = other.pc;
        self.mem.reset(&other.mem);
    }


    pub fn run(&mut self) -> CoreEmuError {
        loop {
            let inst = self.mem.read(
                VirtAddr(self.pc as usize)
            );
            
            // TODO!: simplify this poopoo
            if let Err(e) = inst {
                return e.into();
            }
            let inst: u32 = inst.unwrap();
            #[cfg(feature="dbg_prints")]
            {
                println!("\n{:016x} {:02x?}", self.pc, &inst.to_le_bytes());
                self.debug();
            }
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
        #[cfg(feature="dbg_prints")]
        println!("auipc {:?} {}", rd, imm);
        self.write_reg(rd, self.pc.wrapping_add_signed(imm as i64));
        self.pc += 4;
        Ok(())
    }

    fn addi(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("addi {:?} {:?} {}", rd, rs1, imm);
        self.write_reg(rd, self.read_reg(rs1).wrapping_add_signed(imm as i64))?;
        self.pc += 4;
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
        #[cfg(feature="dbg_prints")]
        println!("xori {:?} {:?} {}", rd, rs1, imm);
        self.write_reg(rd, self.read_reg(rs1) ^ imm as i64 as u64)?;
        self.pc += 4;
        Ok(())
    }

    fn ori(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("ori {:?} {:?} {}", rd, rs1, imm);
        self.write_reg(rd, self.read_reg(rs1) | imm as i64 as u64)?;
        self.pc += 4;
        Ok(())
    }

    fn andi(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("andi {:?} {:?} {}", rd, rs1, imm);
        self.write_reg(rd, self.read_reg(rs1) & imm as i64 as u64)?;
        self.pc += 4;
        Ok(())
    }

    fn slli(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("slli {:?} {:?} {}", rd, rs1, shamt);
        self.write_reg(rd, self.read_reg(rs1).overflow_shl(shamt as u32 as u64))?;
        self.pc += 4;
        Ok(())
    }

    fn srli(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("srli {:?} {:?} {}", rd, rs1, shamt);
        self.write_reg(rd, self.read_reg(rs1).overflow_shr(shamt as u32 as u64))?;
        self.pc += 4;
        Ok(())
    }

    fn srai(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("srai {:?} {:?} {}", rd, rs1, shamt);
        self.write_reg(rd, self.read_reg(rs1).overflow_sar(shamt as u32 as u64))?;
        self.pc += 4;
        Ok(())
    }

    fn add(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("add {:?} {:?} {:?}", rd, rs1, rs2);
        self.write_reg(rd, self.read_reg(rs1).wrapping_add(self.read_reg(rs2)))?;
        self.pc += 4;
        Ok(())
    }

    fn sub(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("sub {:?} {:?} {:?}", rd, rs1, rs2);
        self.write_reg(rd, self.read_reg(rs1).wrapping_sub(self.read_reg(rs2)))?;
        self.pc += 4;
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
        #[cfg(feature="dbg_prints")]
        println!("xor {:?} {:?} {:?}", rd, rs1, rs2);
        self.write_reg(rd, self.read_reg(rs1) ^ self.read_reg(rs2))?;
        self.pc += 4;
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
        #[cfg(feature="dbg_prints")]
        println!("or {:?} {:?} {:?}", rd, rs1, rs2);
        self.write_reg(rd, self.read_reg(rs1) | self.read_reg(rs2))?;
        self.pc += 4;
        Ok(())
    }

    fn and(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("and {:?} {:?} {:?}", rd, rs1, rs2);
        self.write_reg(rd, self.read_reg(rs1) & self.read_reg(rs2))?;
        self.pc += 4;
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
        #[cfg(feature="dbg_prints")]
        println!("jal {:?} {}", rd, imm);
        // ret addr
        self.write_reg(rd, self.pc.wrapping_add(4));
        // jmp
        self.pc = self.pc.wrapping_add_signed(imm as i64);
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
        self.write_reg(rd, self.read_reg(rs1).wrapping_mul(self.read_reg(rs2)))?;
        self.pc += 4;
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
        self.write_reg(rd, self.read_reg(rs1).wrapping_div(self.read_reg(rs2)))?;
        self.pc += 4;
        Ok(())
    }

    fn divu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn rem(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        self.write_reg(rd, self.read_reg(rs1).wrapping_rem(self.read_reg(rs2)))?;
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

    fn c_ld(&mut self, rd: Register, rs1: Register, uimm: u16) -> Result<(), Self::Error> {
        let addr = self.read_reg(rs1).wrapping_add(uimm as u64);
        let res = self.mem.read(VirtAddr(addr as usize))?;
        self.write_reg(rd, res)?;
        self.pc += 2;
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
        self.write_reg(rd, self.read_reg(rd).wrapping_sub(self.read_reg(rs2)))?;
        self.pc += 2;
        Ok(())
    }

    fn c_xor(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        self.write_reg(rd, self.read_reg(rd) ^ self.read_reg(rs2))?;
        self.pc += 2;
        Ok(())
    }

    fn c_or(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        self.write_reg(rd, self.read_reg(rd) | self.read_reg(rs2))?;
        self.pc += 2;
        Ok(())
    }

    fn c_and(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        self.write_reg(rd, self.read_reg(rd) & self.read_reg(rs2))?;
        self.pc += 2;
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
        #[cfg(feature="dbg_prints")]
        println!("c_jr {:?}", rs1);
        self.pc = self.read_reg(rs1);
        Ok(())
    }

    fn c_mv(&mut self, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_mv {:?} {:?}", rs1, rs2);
        self.write_reg(rs1, self.read_reg(rs2))?;
        self.pc += 2;
        Ok(())
    }

    fn c_jalr(&mut self, rs1: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }

    fn c_add(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_add {:?} {:?}", rd, rs2);
        self.write_reg(rd, self.read_reg(rd).wrapping_add(self.read_reg(rs2)))?;
        self.pc += 2;
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
        self.pc += 4;            
        Err(CoreEmuError::Yield)
    }
    fn fence_i(&mut self) -> Result<(), Self::Error> {
        // CHECK
        self.pc += 4;            
        Err(CoreEmuError::Yield)
    }
    fn ecall(&mut self) -> Result<(), Self::Error> {
        self.pc += 4;            
        Err(CoreEmuError::Syscall)
    }
    fn ebreak(&mut self) -> Result<(), Self::Error> {
        self.pc += 4;            
        Err(CoreEmuError::Breakpoint)
    }
    fn c_nop(&mut self) -> Result<(), Self::Error> {
        self.pc += 2;            
        Ok(())
    }
    fn c_ebreak(&mut self) -> Result<(), Self::Error> {
        self.pc += 2;            
        Err(CoreEmuError::Breakpoint)
    }
}
