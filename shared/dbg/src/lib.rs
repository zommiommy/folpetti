use super::*;

mod address;
pub use address::*;
mod breakpoint;
pub use breakpoint::*;
mod memory_map;
pub use memory_map::*;
mod ptrace;
pub use ptrace::*;
mod registers;
pub use registers::*;

const SEPARATOR: &str = concat!(
    "----------------------------------------",
    "----------------------------------------",
);

/// This is thought to be used as a programmatic debugger, this can be compiled 
/// against musl to get a static compiler that can easily be shipped.
///
/// I will also use this to catch syscalls or to get coverage for fuzzing.
///
/// TODO!: add the ELF parser to get debug symbols and add sections info for 
/// better breakpoints
///
/// TODO!: Handle threading
pub struct Debugger{
    ptrace: Ptrace,
    breakpoints: Breakpoints,
}

impl Default for Debugger {
    fn default() -> Self {
        Debugger{
            ptrace: Ptrace::default(),
            breakpoints: Breakpoints::default(),
        }
    }
}

impl std::fmt::Debug for Debugger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(SEPARATOR).unwrap();

        let regs = self.ptrace.get_registers();
        f.write_str(&format!("{:?}", regs)).unwrap();

        f.write_str(SEPARATOR).unwrap();

        // disassembler current RIP and next instructions

        f.write_str(SEPARATOR).unwrap();

        // function trace

        f.write_str(SEPARATOR).unwrap();

        // stack 
        for _ in 0..10 {
            f.write_str(&format!(
                "{:016x}\n",
                self.ptrace.read_memory(
                    Address::Absolute(
                        regs.get_register(Register::Rsp) as usize
                    )
                )
            )).unwrap();
        }

        f.write_str(SEPARATOR).unwrap();

        Ok(())
    }
}

impl Debugger {
    /// Create a new debugger
    /// 
    /// # Arguments
    /// * `command`: String - The path to the executable
    /// * `args`: Vec<String> - The list of arguments to be passed to the 
    ///     executable.
    pub fn new(command: String, args: Vec<String>) -> Debugger {
        let mut dbg = Debugger::default();
        dbg.ptrace.command = command;
        dbg.ptrace.args = args;
        dbg
    }

    pub fn get_memory_map() {
        unimplemented!("TODO!:");
    }


    pub fn get_breakpoint() {
        unimplemented!("TODO!:");
    }

    /// Register a new breakpoint at a given address
    pub fn set_breakpoint(&mut self, address: Address) -> &mut Breakpoint {
        self.breakpoints.push(
            Breakpoint::new(address, self.ptrace.pid)
        )
    }

    /// Get a reference to the structs that holds all the registered breakpoints
    pub fn get_breakpoints(&self) -> &Breakpoints {
        &self.breakpoints
    }

    /// Delete **all** the breakpoints set to a given address
    pub fn delete_breakpoint(&mut self, _address: Address) {
        unimplemented!("TODO!:")
    }

    fn handle_breakpoints(&mut self) {
        let address = self.ptrace.get_register(Register::Rip) as usize;
        for breakpoint in self.breakpoints.breakpoints.iter_mut() {
            breakpoint.handle(address, &mut self.ptrace);
        }
    }

    fn handle_signals(&mut self) {
        loop {
            self.ptrace.cont();
            let mut status: i32 = 0;
            unsafe{
                libc::waitpid(self.ptrace.pid as i32, (&mut status) as _, 0)
            };

            if libc::WIFEXITED(status) {
                println!("Inferior exited - debugger terminating");
                std::process::exit(0);
            }

            if !libc::WIFSTOPPED(status) {
                panic!("Unexpected stop in inferior with status: {}", status);
            }

            match libc::WSTOPSIG(status) {
                libc::SIGTRAP => {
                    self.handle_breakpoints()
                }
                libc::SIGSEGV => {
                    println!("Inferior Seg faulted at {:>16x}", self.ptrace.get_register(Register::Rip));
                    std::process::exit(0);
                }
                x @ _ => {
                    println!("Inferior got signal {}", x);
                }
            }
        }
    }

    fn start_child(&self) {
        // allow tracing of this process
        unsafe{libc::ptrace(libc::PTRACE_TRACEME, 0, 0, 0)};

        println!("Starting the child process");

        // replace us with the process
        let args = vec![self.ptrace.command.as_ptr(), 0 as _];
        unsafe{
                libc::execv(
                self.ptrace.command.as_bytes().as_ptr() as _,
                args.as_ptr() as _,
            )
        };
        
        println!("Child process exited");
        std::process::exit(0);
    }

    fn enable_all_breakpoints(&mut self) {
        for brk in self.breakpoints.breakpoints.iter_mut() {
            brk.enable(&mut self.ptrace);
        }
    }

    /// Start the child process and start handling the signals and breakpoints
    pub fn run(&mut self) {
        match unsafe {libc::fork()} {
            // Start the child process, setup ptrace and replace the process
            // with the program we want to debug
            0 => self.start_child(),
            -1 => {
                panic!("Cannot fork process");
            }
            child @ _ => {
                println!("The child process has pid {}", child);
                self.ptrace.pid = child as Pid;
                self.ptrace.set_exitkill();
                // continue until execve
                // then setup the memory maps and the breakpoints
                self.ptrace.setup_memory_map(child as Pid);
                self.ptrace.continue_to_syscall();
                self.ptrace.continue_to_syscall();
                self.enable_all_breakpoints();
                self.handle_signals();
            },
        }
    }

    /// Start an interactive pwndbg like debugger
    pub fn interactive(&mut self) {
        unimplemented!("TODO!:")
    //     loop { 
    //         println!("{:?}", dbg);

    //         line = read_line(stdio);
    //         match line {
    //             // single step
    //             "s" => {
    //                 dbg.step();
    //             }
    //             // continue
    //             "c" => {
    //                 dbg.cont();
    //             }
    //             // finish
    //             "f" => {
    //                 dbg.finish();
    //             }
    //             // set break point
    //             "b" => {
    //                 dbg.set_breakpoint(address);
    //             }
    //             // delete breakpoint
    //             "d" => {
    //                 dbg.delete_breakpoint(address);
    //             }
    //             // examine memory
    //             "x" => {
    //                 dbg.delete_breakpoint(address);
    //             }
    //             // help
    //             "h" => {
    //                 println!("HelP!")
    //             }
    //             // Exit the interactive shell
    //             "quit" => {
    //                 println!("bye dude")
    //                 break
    //             }
    //         }
    //     }
    }
}