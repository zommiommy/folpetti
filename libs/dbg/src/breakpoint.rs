use super::*;

/// A breakpoint, this is implemented by writing an int3 instruction (0xCC)
/// at the given addres, so that we can catch the SIGTRAP signal and correctly
/// handle it.
///
/// A breakpoint can be either enabled or disabled, in the former case we write the 
/// 0xCC byte, in latter we restore the memory wort to the original one.
///
/// This could break bad if the code is self-modifying.
pub struct Breakpoint
{
    /// The child of the child process on which the breakpoint will work on
    pub child_pid: Pid,

    /// The address of the breakpoint, this will be resolved at runtime
    /// Since relative addresses could change during the execution
    pub address: Address,
    
    /// The original word of memory at `address`, this is needed to be able to
    /// restore / disable the breakpoint
    pub original_word: u64,

    /// If the current breakpoint is enabled or not
    pub is_active: bool,

    /// Only used for conditional breakpoints,
    /// This is the function that will takes a reference 
    /// to the debugger and it must return if we should break or just continue
    /// executing
    pub condition: Box<dyn Fn(&mut Ptrace) ->  bool>,

    /// Set what to do when reaching this breakpoint
    pub handler: Box<dyn Fn(&mut Ptrace)>
}

impl std::fmt::Display for Breakpoint {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("Breakpoint")
            .field("child_pid", &self.child_pid)
            .field("address", &self.address)
            .field("is_active", &self.is_active)
            .finish()
    }
}

impl Breakpoint {
    /// Create a new breakpoint
    pub fn new(address: Address, child_pid: Pid) -> Breakpoint {
        Breakpoint{
            address,
            child_pid,
            original_word: 0,
            is_active: false,
            // By default always break
            condition: Box::new(|_| true),
            handler: Box::new(|_| {}),
        }
    }
    
    /// Set a custom condition to check if we need to stop at this breakpoint
    pub fn set_condition(
        &mut self, 
        condition : Box<dyn Fn(&mut Ptrace) ->  bool>
    ) -> &mut Breakpoint {
        self.condition = condition;
        self
    }
    
    /// Set a custom handler, this function will be triggered everytime the 
    /// breakpoint is reached and the condition is met.
    pub fn set_handler(
        &mut self, 
        handler : Box<dyn Fn(&mut Ptrace)>
    ) -> &mut Breakpoint {
        self.handler = handler;
        self
    }

    /// Enable the current breakpoint, this will read the original memory word
    /// and then it will write a 0xCC to it.
    ///
    /// While we could cache the memory word, during execution the code could 
    /// change, thus if we re-read it each time we can handle more cases
    /// even tho it's slower
    pub fn enable(&mut self, ptrace: &mut Ptrace) {
        // A double enable will result in non being ever able to restore 
        // the code and thus disabling the breakpoint
        if self.is_active == true {
            return;
        }
        println!("Breakpoint at {:>16x} enabled", ptrace.memory_map.resolve_address(&self.address) as usize);
        self.original_word = ptrace.read_memory(self.address.clone());
        println!("Orig code {:>16x}", self.original_word);

        let breakpoint_word = 
            (self.original_word as u64 & (!0xFF)) 
            | 0xCC;

        println!("Brk  code {:>16x}", breakpoint_word);
        ptrace.write_memory(self.address.clone(), breakpoint_word);
        self.is_active = true;
    }


    /// Disable the current breakpoint, this will write the original 
    /// insruction to the code.
    pub fn disable(&mut self, ptrace: &mut Ptrace) {
        // while it would not create any problems, re-writing the same data it's
        // a waste of time
        if self.is_active == false {
            return;
        }

        ptrace.write_memory(self.address.clone(), self.original_word);
        self.is_active = false; 
    }

    /// Given the current breakpoint address, check if this breakpoint was
    /// reached, in that case check the condition and handle it if needed.
    pub fn handle(&mut self, address: usize, ptrace: &mut Ptrace) {
        // check if the address match
        if address != ptrace.memory_map.resolve_address(&self.address) as usize {
            return;
        }

        // check if the condition is met
        if !((self.condition)(ptrace)) {
            return;
        }

        println!("Handling breakpoint at address {:08X}", address);
        // handle the breakpoint
        (self.handler)(ptrace)

        // TODO!: step to skip the current instruction
        // This requires a disassembler smh
    }
}

/// Collections of all the breakpoints
pub struct Breakpoints {
    pub breakpoints: Vec<Breakpoint>,
}

impl Default for Breakpoints {
    fn default() -> Self {
        Breakpoints{
            breakpoints: Vec::new(),
        }
    }
}

impl Breakpoints {
    /// Add a breakpoint to the debugger
    pub fn push(&mut self, breakpoint: Breakpoint) -> &mut Breakpoint {
        self.breakpoints.push(breakpoint);
        self.breakpoints.last_mut().unwrap()
    }
}