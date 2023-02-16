
use mmu::Mmu;
use emu::riscv64gc::*;

fn main() {
    let file_bytes = std::fs::read("../test_fuzz/target/riscv64gc-unknown-linux-gnu/debug/test_fuzz").unwrap();

    let mut mmu = <Mmu<
            0x100, // DIRTY_BLOCK_SIZE
            true, // RAW
            true, // TAINT
        >>::new();

    let load_info = ld::load_object(&file_bytes, &mut mmu);

    let rsp = ld::setup_stack(
        load_info.stack_address, 
        &["test_fuzz"], 
        &[], 
        &[], 
        &mut mmu,
    );

    let mut start_emu = LinuxEmu::new(mmu); 

    // setup the emulator registers
    start_emu.core.pc = load_info.start_address.0 as _;
    // The +8 i'ts RISCV specific https://stackoverflow.com/questions/68645402/where-does-the-stack-pointer-start-for-risc-v-and-where-does-the-stack-pointer
    start_emu.core.write_reg(Register::Sp, rsp.0 as u64 + 8);

    // Run my beauftiful intellectuals, run
    let mut emu = start_emu.fork();
    println!("{:?}", emu.run());
    emu.reset(&start_emu);
    println!("{:?}", emu.run());

}
