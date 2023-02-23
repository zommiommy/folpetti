use super::{CoreEmu, CoreEmuError, LinuxSyscall};
use mmu::{Mmu, MmuError, VirtAddr};
use diss::riscv64gc::*;

#[derive(Debug)]
pub enum LinuxEmuError {
    /// the system called a syscall that isn't bussing:)
    BadSyscall(u64),
    /// The execution hitted a breakpoint
    Breakpoint,
    /// Oh-oh a memory error! yays
    MmuError(MmuError),
    /// Buuuu, can't write to the zero reg
    RegWrite,
    /// We are done!
    Exit(u64),
}

pub struct LinuxEmu {
    pub core: CoreEmu,
}

impl LinuxEmu {
    pub fn new(mem: Mmu) -> Self {
        LinuxEmu{
            core:CoreEmu::new(mem)
        }
    }

    pub fn reset(&mut self, other: &Self) {
        self.core.reset(&other.core);
    }

    pub fn fork(&self) -> Self {
        LinuxEmu { core: self.core.fork() }
    }

    pub fn run(&mut self) -> LinuxEmuError {
        loop {
            match self.core.run() {
                // useful only on multithreaded systems
                CoreEmuError::Yield => {},
                CoreEmuError::Syscall => {
                    // https://github.com/riscv-collab/riscv-gnu-toolchain/blob/master/linux-headers/include/asm-generic/unistd.h#L183
                    let syscall_number = self.core.read_reg(Register::A7);
                    let syscall_variant = syscall_number.try_into();
                    // TODO!: make this cleaner
                    if syscall_variant.is_err() {
                        return LinuxEmuError::BadSyscall(syscall_number);
                    }
                    let syscall_variant = syscall_variant.unwrap();

                    #[cfg(feature="dbg_prints")]
                    println!("syscall {:?}", syscall_variant);
                    match syscall_variant {
                        LinuxSyscall::exit => {
                            return LinuxEmuError::Exit(self.core.read_reg(Register::A0));
                        }
                        LinuxSyscall::read => {
                            let fd = self.core.read_reg(Register::A1);
                            let buf = self.core.read_reg(Register::A2);
                            let fcount= self.core.read_reg(Register::A3);
                            todo!("read");
                        }
                        LinuxSyscall::write => {
                            let fd = self.core.read_reg(Register::A1);
                            let buf = self.core.read_reg(Register::A2);
                            let fcount= self.core.read_reg(Register::A3);
                            todo!("write");
                        }
                        LinuxSyscall::close => {
                            let fd = self.core.read_reg(Register::A1);
                            todo!("close");
                        }
                        LinuxSyscall::brk => {
                            let brk = self.core.read_reg(Register::A1);
                            todo!("brk");
                        }
                        LinuxSyscall::mmap => {
                            todo!("mmap");
                        }
                        LinuxSyscall::munmap => {
                            todo!("munmap");
                        }
                        LinuxSyscall::mremap => {
                            todo!("mremap");
                        }
                        LinuxSyscall::clone => {
                            let flags = self.core.read_reg(Register::A1);
                            let newsp = self.core.read_reg(Register::A2);
                            let parent_tid = self.core.read_reg(Register::A3);
                            let child_tid = self.core.read_reg(Register::A4);
                            todo!("clone");
                        }
                        LinuxSyscall::execve => {
                            let filename = self.core.read_reg(Register::A1);
                            let argv = self.core.read_reg(Register::A2);
                            let envp = self.core.read_reg(Register::A3);
                            todo!("execve");
                        }
                        syscall_variant => {
                            todo!("handle syscall {:?}", syscall_variant)
                        }
                    }
                },
                CoreEmuError::Breakpoint => {
                    return LinuxEmuError::Breakpoint;
                },
                CoreEmuError::RegWrite => {
                    return LinuxEmuError::RegWrite;
                },
                CoreEmuError::MmuError(mmu_error) => {
                    return LinuxEmuError::MmuError(mmu_error);
                },
            }
        }
    }
}