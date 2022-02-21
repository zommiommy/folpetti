#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]
/// An utility enum to get easier to use api's for the debugger
pub enum Register {
    Rip,
    Eflags,

    Rax,
    Rbx,
    Rcx,
    Rdx,

    Rsi,
    Rdi,
    Rsp,
    Rbp,

    R8,
    R9,
    R10,
    R11,

    R12,
    R13,
    R14,
    R15,
    /// Code Segment
    Cs,
    /// Stack Segment
    Ss,
    /// Data Segment
    Ds,
    /// Extra Segment
    Es,
    /// General purpose Segment
    Fs,
    /// General purpose Segment
    Gs,
}

/// Based on `user_regs_struct` defined in `sys/user.h`.
#[repr(C)]
pub struct Registers {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbp: u64,
    pub rbx: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub orig_rax: u64,
    pub rip: u64,
    pub cs: u64,
    pub eflags: u64,
    pub rsp: u64,
    pub ss: u64,
    pub fs_base: u64,
    pub gs_base: u64,
    pub ds: u64,
    pub es: u64,
    pub fs: u64,
    pub gs: u64,
}

impl std::fmt::Debug for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("rax: {:>16X} ", self.rax)).unwrap();
        f.write_str(&format!("rbx: {:>16X} ", self.rbx)).unwrap();
        f.write_str(&format!("rcx: {:>16X} ", self.rcx)).unwrap();
        f.write_str(&format!("rdx: {:>16X} ", self.rdx)).unwrap();
        f.write_str("\n").unwrap();

        f.write_str(&format!("rsi: {:>16X} ", self.rsi)).unwrap();
        f.write_str(&format!("rdi: {:>16X} ", self.rdi)).unwrap();
        f.write_str(&format!("rsp: {:>16X} ", self.rsp)).unwrap();
        f.write_str(&format!("rbp: {:>16X} ", self.rbp)).unwrap();
        f.write_str("\n").unwrap();

        f.write_str(&format!("r8 : {:>16X} ", self.r8 )).unwrap();
        f.write_str(&format!("r9 : {:>16X} ", self.r9 )).unwrap();
        f.write_str(&format!("r10: {:>16X} ", self.r10)).unwrap();
        f.write_str(&format!("r11: {:>16X} ", self.r11)).unwrap();
        f.write_str("\n").unwrap();

        f.write_str(&format!("r12: {:>16X} ", self.r12)).unwrap();
        f.write_str(&format!("r13: {:>16X} ", self.r13)).unwrap();
        f.write_str(&format!("r14: {:>16X} ", self.r14)).unwrap();
        f.write_str(&format!("r15: {:>16X} ", self.r15)).unwrap();
        f.write_str("\n").unwrap();

        f.write_str(&format!("rip: {:>16X} ", self.rip)).unwrap();
        f.write_str(&format!("efs: {:>16X} ", self.eflags)).unwrap();
        f.write_str(&format!("cs : {:>16X} ", self.cs)).unwrap();
        f.write_str(&format!("ss : {:>16X} ", self.ss)).unwrap();
        f.write_str("\n").unwrap();

        f.write_str(&format!("ds : {:>16X} ", self.ds)).unwrap();
        f.write_str(&format!("es : {:>16X} ", self.es)).unwrap();
        f.write_str(&format!("fs : {:>16X} ", self.fs)).unwrap();
        f.write_str(&format!("gs : {:>16X} ", self.gs)).unwrap();
        f.write_str("\n").unwrap();

        Ok(())
    }
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            r15: 0,
            r14: 0,
            r13: 0,
            r12: 0,
            rbp: 0,
            rbx: 0,
            r11: 0,
            r10: 0,
            r9: 0,
            r8: 0,
            rax: 0,
            rcx: 0,
            rdx: 0,
            rsi: 0,
            rdi: 0,
            orig_rax: 0,
            rip: 0,
            cs: 0,
            eflags: 0,
            rsp: 0,
            ss: 0,
            fs_base: 0,
            gs_base: 0,
            ds: 0,
            es: 0,
            fs: 0,
            gs: 0,
        }
    }
}

impl Registers {
    pub fn get_register(&self, reg: Register) -> u64 {
        match reg {
            Register::Rax => {
                self.rax
            }
            Register::Rbx => {
                self.rbx
            }
            Register::Rcx => {
                self.rcx
            }
            Register::Rdx => {
                self.rdx
            }, 
            Register::Rsi => {
                self.rsi
            }, 
            Register::Rdi => {
                self.rdi
            }, 
            Register::Rsp => {
                self.rsp
            }, 
            Register::Rbp => {
                self.rbp
            }, 
            Register::R8 => {
                self.r8
            }, 
            Register::R9 => {
                self.r9
            }, 
            Register::R10 => {
                self.r10
            }, 
            Register::R11 => {
                self.r11
            }, 
            Register::R12 => {
                self.r12
            }, 
            Register::R13 => {
                self.r13
            }, 
            Register::R14 => {
                self.r14
            }, 
            Register::R15 => {
                self.r15
            }, 
            Register::Rip => {
                self.rip
            }, 
            Register::Eflags => {
                self.eflags
            }, 
            Register::Cs => {
                self.cs
            }, 
            Register::Ss => {
                self.ss
            }, 
            Register::Ds => {
                self.ds
            }, 
            Register::Es => {
                self.es
            }, 
            Register::Fs => {
                self.fs
            }, 
            Register::Gs => {
                self.gs
            },
        }
    }

    pub fn set_register(&mut self, reg: Register, value: u64) {
        match reg {
            Register::Rax => {
                self.rax = value;
            }
            Register::Rbx => {
                self.rbx = value;
            }
            Register::Rcx => {
                self.rcx = value;
            }
            Register::Rdx => {
                self.rdx = value;
            }, 
            Register::Rsi => {
                self.rsi = value;
            }, 
            Register::Rdi => {
                self.rdi = value;
            }, 
            Register::Rsp => {
                self.rsp = value;
            }, 
            Register::Rbp => {
                self.rbp = value;
            }, 
            Register::R8 => {
                self.r8 = value;
            }, 
            Register::R9 => {
                self.r9 = value;
            }, 
            Register::R10 => {
                self.r10 = value;
            }, 
            Register::R11 => {
                self.r11 = value;
            }, 
            Register::R12 => {
                self.r12 = value;
            }, 
            Register::R13 => {
                self.r13 = value;
            }, 
            Register::R14 => {
                self.r14 = value;
            }, 
            Register::R15 => {
                self.r15 = value;
            }, 
            Register::Rip => {
                self.rip = value;
            }, 
            Register::Eflags => {
                self.eflags = value;
            }, 
            Register::Cs => {
                self.cs = value;
            }, 
            Register::Ss => {
                self.ss = value;
            }, 
            Register::Ds => {
                self.ds = value;
            }, 
            Register::Es => {
                self.es = value;
            }, 
            Register::Fs => {
                self.fs = value;
            }, 
            Register::Gs => {
                self.gs = value;
            },
        }
    }
}
