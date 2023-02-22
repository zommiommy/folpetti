use mmu::*;
use ld::*;
use goblin::elf::*;
use emu::riscv64gc::*;

fn main() {
    let file_bytes = std::fs::read("../test_fuzz/target/riscv64gc-unknown-linux-gnu/debug/test_fuzz").unwrap();
    let ld_bytes = std::fs::read("/usr/riscv64-linux-gnu/lib/ld-linux-riscv64-lp64d.so.1").unwrap();

    let mut mmu = <Mmu<
            0x100, // DIRTY_BLOCK_SIZE
            true, // RAW
            true, // TAINT
        >>::new();

    let mut ld = Loader {
        ld_name: &"/lib/ld-linux-riscv64-lp64d.so.1",
        ld_bytes: &ld_bytes,
        ld_addr: VirtAddr(0xa000_0000_0000),//VirtAddr(0x7fff_f7fc_9000),
        exec_filename: b"\xAA\xAA\xAA\xAA\xAA\x00",//b"test_fuzz\0",
        random_value: b"\x69\x69\x69\x69\x69\x69\x69\x69\x69\x69\x69\x69\x69\x69\x69\x69", //b"OwO UwU? OwO UwU!",
        platform: b"\xBB\xBB\x00",//b"folpettOs\0",
        stack_size: 0x10_000,//0x2_0000,
    };

    let load_info = ld.load_object(&file_bytes, &mut mmu,
        &["test_fuzz"], 
        &[], 
        &[], 
    );

    let mut start_emu = LinuxEmu::new(mmu); 

    // setup the emulator registers
    start_emu.core.pc = load_info.loader_entry.0 as _;
    // The +8 i'ts RISCV specific https://stackoverflow.com/questions/68645402/where-does-the-stack-pointer-start-for-risc-v-and-where-does-the-stack-pointer
    start_emu.core.write_reg(Register::Sp, load_info.rsp.0 as u64 + 8);

    // Run my beauftiful intellectuals, run
    let mut emu = start_emu.fork();
    
    emu.core.print_stack();
    emu.core.mem.vmmap();

    println!("{:?}", emu.run());

    emu.core.mem.vmmap();
    //emu.reset(&start_emu);
    //println!("{:?}", emu.run());
}
