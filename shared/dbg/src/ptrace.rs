
pub type Pid = usize;

/// Wrapper for ptrace on a process
pub struct Ptrace {
    pub command: String,
    pub args: Vec<String>,
    pub pid: Pid,
    pub memory_map: MemoryMap,
}

impl Default for Ptrace {
    fn default() -> Self {
        Ptrace {
            memory_map: MemoryMap::default(),
            command: String::new(),
            args: Vec::new(),
            pid: 0,
        }
    }
}

impl Ptrace {
    /// Tell the kernel that when the parent (us) will be killed, also the child
    /// should be terminated.
    pub fn set_exitkill(&mut self) {
        let _error_code = unsafe {
            libc::ptrace(libc::PTRACE_SETOPTIONS, self.pid, 0, libc::PTRACE_O_EXITKILL);
        };
    }

    /// Wait for the child
    pub fn wait(&mut self) {
        unsafe {
            libc::waitpid(self.pid as _, std::ptr::null_mut(), 0);
        }
    }

    /// Step by a single instruction
    pub fn step(&mut self) {
        let _error_code = unsafe{
            libc::ptrace(libc::PTRACE_SINGLESTEP, self.pid, 0, 0)
        };

        // if error_code < 0 {
        //     panic!("Cannot single step child: error code: {}", error_code);
        // }
    }

    /// Continue until the next signal or breakpoint
    ///
    /// (This method should be called continue but it's a reserved word RIP)
    pub fn cont(&mut self) {
        let _error_code = unsafe{
            libc::ptrace(libc::PTRACE_CONT, self.pid, 0, 0)
        };

        // if error_code < 0 {
        //     panic!("Cannot continue child: error code: {}", error_code);
        // }
    }

    /// Continue execution and stop at the next syscall 
    pub fn continue_to_syscall(&mut self) {
        let _error_code = unsafe {
            libc::ptrace(libc::PTRACE_SYSCALL, self.pid, 0, 0)
        };
        self.wait();
    }

    /// Continue until we get to a ret instruction (0xC3, 0xC2, 0xCB, 0xCA).
    pub fn finish(&mut self) -> Result<()> {
        loop {
            let rip = self.get_register(Register::Rip);
            let current_inst = self.read_memory(Address::Absolute(rip as usize));

            // Match the first byte of the instruction
            match current_inst.to_be_bytes()[0] {
                // Near return to calling procedure
                0xC3 => {
                    return Ok(());
                }
                // Near return to calling procedure and pop imm16 bytes from 
                // stack
                0xC2 => {
                    return Ok(());
                }
                // Far return to calling procedure
                0xCB => {
                    return Ok(());
                }
                // Far return to calling procedure and pop imm16 bytes from 
                // stack
                0xCA => {
                    return Ok(());
                }
                // Not a ret instruction, just continue executing
                _ => {

                }
            }
            self.step();
        }
    }

    pub fn read_memory(&self, address: Address) -> u64 {
        unsafe{
            libc::ptrace(
                libc::PTRACE_PEEKDATA, 
                self.pid, 
                self.memory_map.resolve_address(&address), 
                0
            ) as u64    
        }
    }

    pub fn write_memory(&mut self, address: Address, data: u64) {
        unsafe{
            libc::ptrace(
                libc::PTRACE_POKEDATA, 
                self.pid, 
                self.memory_map.resolve_address(&address), 
                data,
            )
        };
    }

    /// Read the registers of the tracee
    pub fn get_registers(&self) -> Registers {
        let mut regs: Registers = Registers::default();
        unsafe{
            libc::ptrace(
                libc::PTRACE_GETREGS,
                self.pid,
                0,
                (&mut regs) as *mut Registers,
            )
        };
        println!("{:?}", regs);
        regs
    }

    /// Set the registers of the tracee
    pub fn set_registers(&mut self, regs: Registers) {
        unsafe{
            libc::ptrace(
                libc::PTRACE_SETREGS,
                self.pid,
                0,
                (&regs) as *const Registers,
            )
        };
    }

    /// Get the value of a register, we currently do not support any register 
    /// smaller than 64-bits, this isn't a problem, you can mask the values.
    pub fn get_register(&mut self, register: Register) -> u64 {
        let regs = self.get_registers();
        regs.get_register(register)
    }
    
    /// Set a register to a given value, we currently do not support any 
    /// register smaller than 64-bits, this isn't a problem, you can mask 
    /// the values.
    pub fn set_register(&mut self, register: Register, value: u64) {
        let mut regs = self.get_registers();
        regs.set_register(register, value);
        self.set_registers(regs);
    }

    pub fn setup_memory_map(&mut self, pid: Pid) {
        self.memory_map = MemoryMap::new(pid);
    }
}