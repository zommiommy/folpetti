
use goblin::elf64::header::*;
use mmu::{Mmu, Perm, PermField, VirtAddr};
use emu::riscv64gc::*;
use goblin::Object;
use goblin::elf64::program_header::*;

fn main() {
    let file_bytes = std::fs::read("../test_fuzz/target/riscv64gc-unknown-linux-gnu/debug/test_fuzz").unwrap();
    let elf = match Object::parse(&file_bytes).unwrap() {
        Object::Elf(elf) => {
            elf
        },
        _ => panic!(),
    };

    assert_eq!(elf.header.e_machine, EM_RISCV);
    // if it's relocatable add an offset so we don't map in the 0x0 page
    // so we catch null derefs
    let vaddr_offset = if elf.header.e_type == ET_DYN {
        0x1000
    } else {
        0x0
    };

    let mut mmu = <Mmu<
            256, // DIRTY_BLOCK_SIZE
            true, // RAW
            true, // TAINT
        >>::new();

    // LOAD ALL THE SEGMENTS
    for segment in elf.program_headers {
        println!("{:?}", segment);

        let mut perms = Perm::default();
        if segment.is_read() {
            perms |= PermField::Read;
        }
        if segment.is_write() {
            perms |= PermField::Write;
        }
        if segment.is_executable() {
            perms |= PermField::Executable;
        }
        if segment.p_type == PT_LOAD {
            println!("{:x?} {:?}", segment.vm_range(), perms);
            let (_idx, seg) = mmu.allocate_segment(
                VirtAddr(vaddr_offset + segment.vm_range().start),
                segment.vm_range().len(), 
                perms
            ).unwrap();


            // data from file
            unsafe {
                seg.write_from_slice(
                    VirtAddr(0),
                    &file_bytes[segment.file_range()],
                ).unwrap();
            }
        }
    }

    // add a stack segment of 1MB
    let (stack_idx, _seg) = mmu.allocate_segment(
        VirtAddr(0x7fff_ffff_0000_0000),
        1 << 6, 
        PermField::ReadAfterWrite | PermField::Write,
    ).unwrap();
    mmu.stack_segment_idx = stack_idx;


    // FIND THE START FUNCTION and load its address in the program counter
    let start_symbol = elf.syms.iter().find(|x| 
        elf.strtab.get_at(x.st_name).unwrap() == "_start"
    ).unwrap();
    assert!(start_symbol.is_function());
    let start_address = start_symbol.st_value;

    let mut start_emu = LinuxEmu::new(mmu); 

    // setup the emulator registers
    start_emu.core.pc = start_address + vaddr_offset as u64;
    start_emu.core.write_reg(Register::Sp,  
        (&start_emu.core.mem.segments[stack_idx]).0.0 as u64
    ).unwrap();


    // Run my beauftiful intellectuals, run
    let mut emu = start_emu.fork();
    println!("{:?}", emu.run());
    emu.reset(&start_emu);
    println!("{:?}", emu.run());

}
