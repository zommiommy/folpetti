use binstuff::*;

fn main() {
    // start the debugger and it's child process
    let mut dbg = Debugger::new("./service".to_string(), Vec::new());

    // set the breakpoints
    // in al, dx
    // check if data is avaliable
    dbg.set_breakpoint(Address::Section(".text".to_string(), 0x10072C))
    .set_handler(Box::new(|pt| {
        let port = pt.get_register(Register::Rdx) & 0xffff;

        match port {
            0x3fd => {
                // lowest bit == 1 -> 
                // Data ready, a complete character has been received 
                // and is available for reading
                pt.set_register(Register::Rax,  0x21);
            },
            0x3f8 => {
                // lowest bit == 1 -> 
                // Data ready, a complete character has been received 
                // and is available for reading
                pt.set_register(Register::Rax,  0x21);
            }
            _ => {}
        }
        // skip the instruction
        // in al, dx =  0xEC
        let new_rip =  pt.get_register(Register::Rip) + 1;
        pt.set_register(Register::Rip, new_rip);
    }));

    // out dx, ax
    dbg.set_breakpoint(Address::Section(".text".to_string(), 0x100706))
    .set_handler(Box::new(|pt| {
        let port = pt.get_register(Register::Rdx) & 0xffff;

        if port == 0x3f8 {
            print!("{}", pt.get_register(Register::Rax) & 0xFFFF);
        }
        // skip the instruction
        // out dx, ax =  0x66 0xEF
        let new_rip =  pt.get_register(Register::Rip) + 2;
        pt.set_register(Register::Rip, new_rip);
    }));


    // out dx, ax
    dbg.set_breakpoint(Address::Section(".text".to_string(), 0x1006D8))
    .set_handler(Box::new(|pt| {
        let port = pt.get_register(Register::Rdx) & 0xffff;

        if port == 0x3f8 {
            print!("{}", pt.get_register(Register::Rax) & 0xFF);
        }
        // skip the instruction
        // out dx, ax =  0x66 0xEF
        let new_rip =  pt.get_register(Register::Rip) + 2;
        pt.set_register(Register::Rip, new_rip);
    }));

    dbg.run();
}
