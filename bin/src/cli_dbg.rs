use binstuff::*;

fn main() {

    let mut dbg = Debugger::new("./service".to_string(), Vec::new());
    dbg.interactive();
}