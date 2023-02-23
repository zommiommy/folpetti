use core::intrinsics::unlikely;
use diss::riscv64gc::*;
use mmu::{Mmu, VirtAddr, MmuError, PermField};
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
    pub instructions_executed: usize,
}

impl CoreEmu {
    pub fn new(mem: Mmu) -> Self {
        CoreEmu {
            regs: [0; 32],
            fregs: [0.0; 32],
            pc: 0,
            mem,
            instructions_executed: 0,
        }
    }

    #[cfg(feature="std")]
    pub fn debug(&self) {
        println!("PC: {:>16x} Zero: {:>16x}", 
            self.pc, 
            self.read_reg(Register::Zero),
        );
        println!(
            "Ra: {:>16x} Sp: {:>16x} Gp : {:>16x} Tp : {:>16x}", 
            self.read_reg(Register::Ra),
            self.read_reg(Register::Sp),
            self.read_reg(Register::Gp),
            self.read_reg(Register::Tp),
        );
        println!(
            "T0: {:>16x} T1: {:>16x} T2 : {:>16x} T3 : {:>16x}", 
            self.read_reg(Register::T0),
            self.read_reg(Register::T1),
            self.read_reg(Register::T2),
            self.read_reg(Register::T3),
        );
        println!(
            "T4: {:>16x} T5: {:>16x} T6 : {:>16x}", 
            self.read_reg(Register::T4),
            self.read_reg(Register::T5),
            self.read_reg(Register::T6),
        );
        println!(
            "A0: {:>16x} A1: {:>16x} A2 : {:>16x} A3 : {:>16x}", 
            self.read_reg(Register::A0),
            self.read_reg(Register::A1),
            self.read_reg(Register::A2),
            self.read_reg(Register::A3),
        );
        println!(
            "A4: {:>16x} A5: {:>16x} A6 : {:>16x} A7 : {:>16x}", 
            self.read_reg(Register::A4),
            self.read_reg(Register::A5),
            self.read_reg(Register::A6),
            self.read_reg(Register::A7),
        );
        println!(
            "S0: {:>16x} S1: {:>16x} S2 : {:>16x} S3 : {:>16x}", 
            self.read_reg(Register::S0),
            self.read_reg(Register::S1),
            self.read_reg(Register::S2),
            self.read_reg(Register::S3),
        );
        println!(
            "S4: {:>16x} S5: {:>16x} S6 : {:>16x} S7 : {:>16x}", 
            self.read_reg(Register::S4),
            self.read_reg(Register::S5),
            self.read_reg(Register::S6),
            self.read_reg(Register::S7),
        );
        println!(
            "S8: {:>16x} S9: {:>16x} S10: {:>16x} S11: {:>16x}", 
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
    pub fn write_reg(&mut self, reg: Register, value: u64)  {
        // ignore writes to reg zero by documentation
        if unlikely(reg == Register::Zero) {
            return;
        }

        self.regs[reg as usize] = value;
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
            instructions_executed: self.instructions_executed,
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
                println!("\n{:016x} {:02x?} {}", self.pc, &inst.to_le_bytes(), self.instructions_executed);
                self.debug();
            }
            self.instructions_executed += 1;
            if let Err(e) = diss_riscv64gc(self, inst) {
                return e;
            }
        }
    }

    #[cfg(feature="std")]
    pub fn print_stack(&mut self) {
        let sp = self.read_reg(Register::Sp) as usize;
        let (stack_start, stack) = self.mem.resolve_segment(VirtAddr(sp)).unwrap();
        for addr in (sp..stack_start.0 + stack.len() - 8).step_by(16) {
            println!("{:016x}: {:016x} {:016x}", addr, 
                unsafe{self.mem.read_with_perm::<u64>(VirtAddr(addr), PermField::None.into()).unwrap()},
                unsafe{self.mem.read_with_perm::<u64>(VirtAddr(addr)+8, PermField::None.into()).unwrap()},
            );
        }
    }
}

impl RV64GCUser<()> for CoreEmu {
    type Error = CoreEmuError;
    #[inline(always)]
    fn lui(&mut self, rd: Register, imm: u32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("lui {:?} {}", rd, imm);
        self.write_reg(rd, imm as i32 as i64 as u64);
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn auipc(&mut self, rd: Register, imm: u32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("auipc {:?} {}", rd, imm);
        let imm = imm << 12;
        self.write_reg(rd, self.pc.wrapping_add_signed(imm as i32 as i64));
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn addi(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("addi {:?} {:?} {}", rd, rs1, imm);
        self.write_reg(rd, self.read_reg(rs1).wrapping_add_signed(imm as i64));
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn slti(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("slti {:?} {:?} {}", rd, rs1, imm);
        Ok(())
    }
    #[inline(always)]
    fn sltiu(&mut self, rd: Register, rs1: Register, imm: u32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("sltiu {:?} {:?} {}", rd, rs1, imm);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn xori(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("xori {:?} {:?} {}", rd, rs1, imm);
        self.write_reg(rd, self.read_reg(rs1) ^ imm as i64 as u64);
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn ori(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("ori {:?} {:?} {}", rd, rs1, imm);
        self.write_reg(rd, self.read_reg(rs1) | imm as i64 as u64);
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn andi(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("andi {:?} {:?} {}", rd, rs1, imm);
        self.write_reg(rd, self.read_reg(rs1) & imm as i64 as u64);
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn slli(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("slli {:?} {:?} {}", rd, rs1, shamt);
        self.write_reg(rd, self.read_reg(rs1).overflow_shl(shamt as u32 as u64));
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn srli(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("srli {:?} {:?} {}", rd, rs1, shamt);
        self.write_reg(rd, self.read_reg(rs1).overflow_shr(shamt as u32 as u64));
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn srai(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("srai {:?} {:?} {}", rd, rs1, shamt);
        self.write_reg(rd, self.read_reg(rs1).overflow_sar(shamt as u32 as u64));
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn add(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("add {:?} {:?} {:?}", rd, rs1, rs2);
        self.write_reg(rd, self.read_reg(rs1).wrapping_add(self.read_reg(rs2)));
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn sub(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("sub {:?} {:?} {:?}", rd, rs1, rs2);
        self.write_reg(rd, self.read_reg(rs1).wrapping_sub(self.read_reg(rs2)));
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn sll(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("sll {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn slt(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("slt {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn sltu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("sltu {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn xor(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("xor {:?} {:?} {:?}", rd, rs1, rs2);
        self.write_reg(rd, self.read_reg(rs1) ^ self.read_reg(rs2));
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn srl(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("srl {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn sra(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("sra {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn or(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("or {:?} {:?} {:?}", rd, rs1, rs2);
        self.write_reg(rd, self.read_reg(rs1) | self.read_reg(rs2));
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn and(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("and {:?} {:?} {:?}", rd, rs1, rs2);
        self.write_reg(rd, self.read_reg(rs1) & self.read_reg(rs2));
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn csrrw(&mut self, rd: Register, rs1: Register, offset: u32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("csrrw {:?} {:?} {}", rd, rs1, offset);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn csrrs(&mut self, rd: Register, rs1: Register, offset: u32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("csrrs {:?} {:?} {}", rd, rs1, offset);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn csrrc(&mut self, rd: Register, rs1: Register, offset: u32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("csrrc {:?} {:?} {}", rd, rs1, offset);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn csrrwi(&mut self, rd: Register, zimm: u8, offset: u32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("csrrwi {:?} {} {}", rd, zimm, offset);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn csrrsi(&mut self, rd: Register, zimm: u8, offset: u32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("csrrsi {:?} {} {}", rd, zimm, offset);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn csrrci(&mut self, rd: Register, zimm: u8, offset: u32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("csrrci {:?} {} {}", rd, zimm, offset);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn lb(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("lb {:?} [{:?}+{}]", rd, rs1, imm);
        let addr = self.read_reg(rs1).wrapping_add_signed(imm as i64);
        let res: u8= self.mem.read(VirtAddr(addr as usize))?;
        self.write_reg(rd, res as i8 as i64 as u64);
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn lh(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("lh {:?} [{:?}+{}]", rd, rs1, imm);
        let addr = self.read_reg(rs1).wrapping_add_signed(imm as i64);
        let res: u16 = self.mem.read(VirtAddr(addr as usize))?;
        self.write_reg(rd, res as i16 as i64 as u64);
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn lw(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("lw {:?} [{:?}+{}]", rd, rs1, imm);
        let addr = self.read_reg(rs1).wrapping_add_signed(imm as i64);
        let res: u32 = self.mem.read(VirtAddr(addr as usize))?;
        self.write_reg(rd, res as i32 as i64 as u64);
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn ld(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("ld {:?} [{:?}+{}]", rd, rs1, imm);
        let addr = self.read_reg(rs1).wrapping_add_signed(imm as i64);
        let res = self.mem.read(VirtAddr(addr as usize))?;
        self.write_reg(rd, res);
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn lbu(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("lbu {:?} [{:?}+{}]", rd, rs1, imm);
        let addr = self.read_reg(rs1).wrapping_add_signed(imm as i64);
        let res: u8 = self.mem.read(VirtAddr(addr as usize))?;
        self.write_reg(rd, res as u64);
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn lhu(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("lhu {:?} [{:?}+{}]", rd, rs1, imm);
        let addr = self.read_reg(rs1).wrapping_add_signed(imm as i64);
        let res: u16 = self.mem.read(VirtAddr(addr as usize))?;
        self.write_reg(rd, res as u64);
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn lwu(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("lwu {:?} [{:?}+{}]", rd, rs1, imm);
        let addr = self.read_reg(rs1).wrapping_add_signed(imm as i64);
        let res: u32 = self.mem.read(VirtAddr(addr as usize))?;
        self.write_reg(rd, res as u64);
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn sb(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("sb {:?} {:?} {}", rs1, rs2, imm);
        let addr = VirtAddr(self.read_reg(rs1).wrapping_add_signed(imm as i64) as _);
        self.mem.write(addr, self.read_reg(rs2) as u8)?;
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn sh(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("sh {:?} {:?} {}", rs1, rs2, imm);
        let addr = VirtAddr(self.read_reg(rs1).wrapping_add_signed(imm as i64) as _);
        self.mem.write(addr, self.read_reg(rs2) as u16)?;
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn sw(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("sw {:?} {:?} {}", rs1, rs2, imm);
        let addr = VirtAddr(self.read_reg(rs1).wrapping_add_signed(imm as i64) as _);
        self.mem.write(addr, self.read_reg(rs2) as u32)?;
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn sd(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("sd {:?} {:?} {}", rs1, rs2, imm);
        let addr = VirtAddr(self.read_reg(rs1).wrapping_add_signed(imm as i64) as _);
        self.mem.write(addr, self.read_reg(rs2))?;
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn jal(&mut self, rd: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("jal {:?} {}", rd, imm);
        // ret addr
        self.write_reg(rd, self.pc.wrapping_add(4));
        // jmp
        self.pc = self.pc.wrapping_add_signed(imm as i64);
        Ok(())
    }
    #[inline(always)]
    fn jalr(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("jalr {:?} {:?} {}", rd, rs1, imm);
        // ret addr
        let ret_addr = self.pc.wrapping_add(4);
        // jmp
        self.pc = self.read_reg(rs1).wrapping_add_signed(imm as i64);
        self.pc &= !1; // se the LSB to 0 for some reason TODO!: needed?
        self.write_reg(rd, ret_addr);
        Ok(())
    }
    #[inline(always)]
    fn beq(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("beq {:?} {:?} {}", rs1, rs2, imm);
        if self.read_reg(rs1) == self.read_reg(rs2) {
            self.pc = self.pc.wrapping_add_signed(imm as i64);
        } else {
            self.pc += 4;
        }
        Ok(())
    }
    #[inline(always)]
    fn bne(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("bne {:?} {:?} {}", rs1, rs2, imm);
        if self.read_reg(rs1) != self.read_reg(rs2) {
            self.pc = self.pc.wrapping_add_signed(imm as i64);
        } else {
            self.pc += 4;
        }
        Ok(())
    }
    #[inline(always)]
    fn blt(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("blt {:?} {:?} {}", rs1, rs2, imm);
        if (self.read_reg(rs1) as i64) < (self.read_reg(rs2) as i64) {
            self.pc = self.pc.wrapping_add_signed(imm as i64);
        } else {
            self.pc += 4;
        }
        Ok(())
    }
    #[inline(always)]
    fn bge(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("bge {:?} {:?} {}", rs1, rs2, imm);
        if (self.read_reg(rs1) as i64) > (self.read_reg(rs2) as i64) {
            self.pc = self.pc.wrapping_add_signed(imm as i64);
        } else {
            self.pc += 4;
        }
        Ok(())
    }
    #[inline(always)]
    fn bltu(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("bltu {:?} {:?} {}", rs1, rs2, imm);
        if self.read_reg(rs1) < self.read_reg(rs2) {
            self.pc = self.pc.wrapping_add_signed(imm as i64);
        } else {
            self.pc += 4;
        }
        Ok(())
    }
    #[inline(always)]
    fn bgeu(&mut self, rs1: Register, rs2: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("bgeu {:?} {:?} {}", rs1, rs2, imm);
        if self.read_reg(rs1) > self.read_reg(rs2) {
            self.pc = self.pc.wrapping_add_signed(imm as i64);
        } else {
            self.pc += 4;
        }
        Ok(())
    }
    #[inline(always)]
    fn addiw(&mut self, rd: Register, rs1: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("addiw {:?} {:?} {}", rd, rs1, imm);
        self.write_reg(rd, 
            self.read_reg(rs1).wrapping_add_signed(imm as i64) as i32 as i64 as u64
        );
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn slliw(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("slliw {:?} {:?} {}", rd, rs1, shamt);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn srliw(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("srliw {:?} {:?} {}", rd, rs1, shamt);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn sraiw(&mut self, rd: Register, rs1: Register, shamt: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("sraiw {:?} {:?} {}", rd, rs1, shamt);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn addw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("addw {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn subw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("subw {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn sllw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("sllw {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn srlw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("srlw {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn sraw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("sraw {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn mul(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("mul {:?} {:?} {:?}", rd, rs1, rs2);
        self.write_reg(rd, self.read_reg(rs1).wrapping_mul(self.read_reg(rs2)));
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn mulh(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("mulh {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn mulhsu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("mulhsu {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn mulhu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("mulhu {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn div(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("div {:?} {:?} {:?}", rd, rs1, rs2);
        self.write_reg(rd, (self.read_reg(rs1) as i64).wrapping_div(self.read_reg(rs2) as i64) as u64);
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn divu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("divu {:?} {:?} {:?}", rd, rs1, rs2);
        self.write_reg(rd, self.read_reg(rs1).wrapping_div(self.read_reg(rs2)));
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn rem(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("rem {:?} {:?} {:?}", rd, rs1, rs2);
        self.write_reg(rd, (self.read_reg(rs1) as i64).wrapping_rem(self.read_reg(rs2) as i64) as u64);
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn remu(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("remu {:?} {:?} {:?}", rd, rs1, rs2);
        self.write_reg(rd, self.read_reg(rs1).wrapping_rem(self.read_reg(rs2)));
        self.pc += 4;
        Ok(())
    }
    #[inline(always)]
    fn mulw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("mulw {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn divw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("divw {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn divuw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("divuw {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn remw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("remw {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn remuw(&mut self, rd: Register, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("remuw {:?} {:?} {:?}", rd, rs1, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn fsqrt_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fsgnj_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fsgnjn_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fsgnjx_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fmin_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fmax_s(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_w_s(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_wu_s(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fmv_x_w(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn feq_s(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn flt_s(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fle_s(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fclass_s(&mut self, rd: Register, rs1: FloatRegister) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_s_w(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_s_wu(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fmv_w_x(&mut self, rd: FloatRegister, rs1: Register) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn fsqrt_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fsgnj_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fsgnjn_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fsgnjx_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fmin_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fmax_d(
        &mut self,
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_s_d(&mut self, rd: FloatRegister, rs1: FloatRegister) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_d_s(&mut self, rd: FloatRegister, rs1: FloatRegister) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn feq_d(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn flt_d(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fle_d(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rs2: FloatRegister,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fclass_d(&mut self, rd: Register, rs1: FloatRegister) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_w_d(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_wu_d(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_d_w(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_d_wu(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn flw(&mut self, rd: FloatRegister, rs1: Register, imm: u32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fsw(
        &mut self,
        rs1: Register,
        rs2: FloatRegister,
        offset: u32,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fld(&mut self, rd: FloatRegister, rs1: Register, offset: u32) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fsd(
        &mut self,
        rs1: Register,
        rs2: FloatRegister,
        offset: u32,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_l_s(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_lu_s(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_s_l(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_s_lu(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_l_d(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_lu_d(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fmv_x_d(
        &mut self,
        rd: Register,
        rs1: FloatRegister,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_d_l(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fcvt_d_lu(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn fmv_d_x(
        &mut self,
        rd: FloatRegister,
        rs1: Register,
        rm: FloatRoundingMode,
    ) -> Result<(), Self::Error> {
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_addi4spn(&mut self, rd: Register, uimm: u16) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_addi4spn {:?} {}", rd, uimm);
        let value = self.read_reg(Register::Sp).wrapping_add(uimm as u64);
        self.write_reg(rd, value);
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn c_fld(&mut self, rd: Register, rs1: Register, imm: u16) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_fld {:?} {:?} {}", rd, rs1, imm);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_lw(&mut self, rd: Register, rs1: Register, uimm: u16) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_lw {:?} {:?} {}", rd, rs1, uimm);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_flw(&mut self, rd: FloatRegister, rs1: Register, uimm: u16) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_flw {:?} {:?} {}", rd, rs1, uimm);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_ld(&mut self, rd: Register, rs1: Register, uimm: u16) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_ld {:?} {:?} {}", rd, rs1, uimm);
        // check
        let addr = self.read_reg(rs1).wrapping_add(8 * uimm as u64);
        let res = self.mem.read(VirtAddr(addr as usize))?;
        self.write_reg(rd, res);
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn c_fsd(
        &mut self,
        rs1: Register,
        rs2: FloatRegister,
        uimm: u16,
    ) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_fsd {:?} {:?} {}", rs1, rs2, uimm);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_sw(&mut self, rs1: Register, rs2: Register, uimm: u16) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_sw {:?} {:?} {}", rs1, rs2, uimm);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_fsw(&mut self, rs1: Register, rs2: FloatRegister, uimm: u8) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_fsw {:?} {:?} {}", rs1, rs2, uimm);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_sd(&mut self, rs1: Register, rs2: Register, uimm: u16) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_sd {:?} {:?} {}", rs1, rs2, uimm);
        let addr = self.read_reg(rs1).wrapping_add(8 * uimm as u64);
        self.mem.write(VirtAddr(addr as _), self.read_reg(rs2));
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn c_addi(&mut self, rd: Register, imm: i8) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_addi {:?} {}", rd, imm);
        self.write_reg(rd, 
            self.read_reg(rd).wrapping_add_signed(imm as i64)
        );
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn c_jal(&mut self, imm: u16) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_jal {}", imm);
        // ret addr
        self.write_reg(Register::Ra, self.pc.wrapping_add(4));
        // jmp
        self.pc = self.pc.wrapping_add_signed(imm as i64);
        Ok(())
    }
    #[inline(always)]
    fn c_addiw(&mut self, rd: Register, imm: i8) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_addiw {:?} {}", rd, imm);
        self.write_reg(rd, 
            self.read_reg(rd).wrapping_add_signed(imm as i64) as i32 as i64 as u64
        );
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn c_li(&mut self, rd: Register, imm: i8) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_li {:?} {}", rd, imm);
        self.write_reg(rd, imm as i64 as u64);
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn c_addi16sp(&mut self, imm: i16) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_addi16sp {}", imm);
        let value = self.read_reg(Register::Sp).wrapping_add_signed(imm as i64);
        self.write_reg(Register::Sp, value);
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn c_lui(&mut self, rd: Register, imm: i32) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_lui {:?} {}", rd, imm);
        let value = self.read_reg(Register::Sp).wrapping_add_signed(imm as i64);
        self.write_reg(rd, value);
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn c_srli(&mut self, rd: Register, uimm: u8) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_srli {:?} {}", rd, uimm);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_srai(&mut self, rd: Register, uimm: u8) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_srai {:?} {}", rd, uimm);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_andi(&mut self, rd: Register, uimm: u8) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_andi {:?} {}", rd, uimm);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_sub(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_sub {:?} {:?}", rd, rs2);
        self.write_reg(rd, self.read_reg(rd).wrapping_sub(self.read_reg(rs2)));
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn c_xor(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_xor {:?} {:?}", rd, rs2);
        self.write_reg(rd, self.read_reg(rd) ^ self.read_reg(rs2));
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn c_or(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_or {:?} {:?}", rd, rs2);
        self.write_reg(rd, self.read_reg(rd) | self.read_reg(rs2));
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn c_and(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_and {:?} {:?}", rd, rs2);
        self.write_reg(rd, self.read_reg(rd) & self.read_reg(rs2));
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn c_subw(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_subw {:?} {:?}", rd, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_addw(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_addw {:?} {:?}", rd, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_j(&mut self, imm: i16) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_j {}", imm);
        self.pc = self.pc.wrapping_add_signed(imm as i64);
        Ok(())
    }
    #[inline(always)]
    fn c_beqz(&mut self, rs1: Register, offset: i16) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_beqz {:?} {}", rs1, offset);
        if self.read_reg(rs1) == 0 {
            self.pc = self.pc.wrapping_add_signed(offset as i64);
        } else {
            self.pc += 2;
        }
        Ok(())
    }
    #[inline(always)]
    fn c_bnez(&mut self, rs1: Register, offset: i16) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_bnez {:?} {}", rs1, offset);
        if self.read_reg(rs1) != 0 {
            self.pc = self.pc.wrapping_add_signed(offset as i64);
        } else {
            self.pc += 2;
        }
        Ok(())
    }
    #[inline(always)]
    fn c_slli(&mut self, rd: Register, uimm: u8) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_slli {:?} {}", rd, uimm);
        let res = self.read_reg(rd).overflow_shl(uimm as u64);
        self.write_reg(rd, res);
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn c_fldsp(&mut self, rd: FloatRegister, uimm: u8) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_fldsp {:?} {}", rd, uimm);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_lwsp(&mut self, rd: Register, uimm: u8) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_lwsp {:?} {}", rd, uimm);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_flwsp(&mut self, rd: Register, uimm: u8) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_flwsp {:?} {}", rd, uimm);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_ldsp(&mut self, rd: Register, uimm: u8) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_ldsp {:?} {}", rd, uimm);
        let addr = self.read_reg(Register::Sp).wrapping_add(uimm as u64);
        let res = self.mem.read(VirtAddr(addr as usize))?;
        self.write_reg(rd, res);
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn c_jr(&mut self, rs1: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]{
            if rs1 == Register::Ra {
                println!("ret");
            } else {
                println!("c_jr {:?}", rs1);
            }
        }
        self.pc = self.read_reg(rs1);
        Ok(())
    }
    #[inline(always)]
    fn c_mv(&mut self, rs1: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_mv {:?} {:?}", rs1, rs2);
        self.write_reg(rs1, self.read_reg(rs2));
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn c_jalr(&mut self, rs1: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_jalr {:?}", rs1);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_add(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_add {:?} {:?}", rd, rs2);
        self.write_reg(rd, self.read_reg(rd).wrapping_add(self.read_reg(rs2)));
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn c_fsdsp(&mut self, rd: Register, rs2: Register) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_fsdsp {:?} {:?}", rd, rs2);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_swsp(&mut self, rs2: Register, uimm: u8) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_swsp {:?} {}", rs2, uimm);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_fswsp(&mut self, rs2: FloatRegister, uimm: u8) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_fswsp {:?} {}", rs2, uimm);
        todo!();
        Ok(())
    }
    #[inline(always)]
    fn c_sdsp(&mut self, rs2: Register, uimm: u8) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_sdsp {:?} {}", rs2, uimm);
        let addr = VirtAddr(self.read_reg(Register::Sp).wrapping_add(uimm as u64) as _);
        self.mem.write(addr, self.read_reg(rs2));
        self.pc += 2;
        Ok(())
    }
    #[inline(always)]
    fn fence(&mut self) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("fence");
        self.pc += 4;            
        Err(CoreEmuError::Yield)
    }
    #[inline(always)]
    fn fence_i(&mut self) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("fence_i");
        // CHECK
        self.pc += 4;            
        Err(CoreEmuError::Yield)
    }
    #[inline(always)]
    fn ecall(&mut self) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("ecall");
        self.pc += 4;            
        Err(CoreEmuError::Syscall)
    }
    #[inline(always)]
    fn ebreak(&mut self) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("ebreak");
        self.pc += 4;            
        Err(CoreEmuError::Breakpoint)
    }
    #[inline(always)]
    fn c_nop(&mut self) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_nop");
        self.pc += 2;            
        Ok(())
    }
    #[inline(always)]
    fn c_ebreak(&mut self) -> Result<(), Self::Error> {
        #[cfg(feature="dbg_prints")]
        println!("c_ebreak");
        self.pc += 2;            
        Err(CoreEmuError::Breakpoint)
    }
}