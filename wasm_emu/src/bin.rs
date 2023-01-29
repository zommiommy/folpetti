use wasmemu::*;

fn main() {
    let data = std::fs::read("./sum.wasm").unwrap();
    let (_, wasm) = WasmModule::parse(&data);
    println!("##############################################################");
    println!("{:#4?}", wasm);
    /*let mut emu = Emu::new(wasm);

    let res = emu.call(
        1, 
        vec![Value::I32(2), Value::I32(5)]
    );
    println!("RESULT: {:?}", res);

    let res = emu.call(
        2, 
        vec![Value::I32(2)]
    );
    println!("RESULT: {:?}", res);
    */
}
