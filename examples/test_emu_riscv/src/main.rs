
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
        //0x0000004000000000
        0x0000004000005170
    } else {
        0x0
    };

    let mut mmu = <Mmu<
            0x100, // DIRTY_BLOCK_SIZE
            true, // RAW
            true, // TAINT
        >>::new();

    // LOAD ALL THE SEGMENTS
    let mut data_segment_idx = usize::MAX;
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
            let (idx, seg) = mmu.allocate_segment(
                VirtAddr(vaddr_offset + segment.vm_range().start),
                segment.vm_range().len(), 
                perms
            ).unwrap();

            // keep track of which is the data segment to be able to do the BRK 
            // syscall
            if perms == (PermField::Read | PermField::Write) {
                data_segment_idx = idx;
            }

            // data from file
            unsafe {
                seg.write_from_slice(
                    VirtAddr(0),
                    &file_bytes[segment.file_range()],
                ).unwrap();
            }
        }
    }

    let stack_base_addr = VirtAddr(0x7fff_ffff_0000_0000);
    let stack_size = 8 << 10; // 1KB
    let stack_top_addr = VirtAddr(stack_base_addr.0 - stack_size);

    let prog_name = b"my_awesome_prog\0";
    let prog_name_addr = VirtAddr(stack_base_addr.0 + 0x1000);

    let (_, prog_name_seg) = mmu.allocate_segment(
        prog_name_addr,
        0x100, 
        PermField::ReadAfterWrite | PermField::Write,
    ).unwrap();
    unsafe{prog_name_seg.write_from_slice(VirtAddr(0), prog_name).unwrap()};

    let (stack_idx, stack) = mmu.allocate_segment(
        stack_top_addr,
        stack_size + 8, 
        PermField::ReadAfterWrite | PermField::Write,
    ).unwrap();

    let mut sp = VirtAddr(stack_size);
    stack.write(sp, 0_u64).unwrap(); // Auxp end
    sp -= 8;
    stack.write(sp, 0_u64).unwrap(); // Envp end
    sp -= 8;
    stack.write(sp, 0_u64).unwrap(); // argv end
    sp -= 8;
    stack.write(sp, prog_name_addr.0 as u64).unwrap(); // argv
    sp -= 8;
    stack.write(sp, 1_u64).unwrap(); // argc
    // sp -= 8; // TODO!: NEEDED????

    mmu.stack_segment_idx = stack_idx;
    mmu.data_segment_idx = data_segment_idx;


    // FIND THE START FUNCTION and load its address in the program counter
    let start_symbol = elf.syms.iter().find(|x| 
        elf.strtab.get_at(x.st_name).unwrap() == "_start"
    ).unwrap();
    assert!(start_symbol.is_function());
    let start_address = start_symbol.st_value;

    let mut start_emu = LinuxEmu::new(mmu); 

    // setup the emulator registers
    start_emu.core.pc = start_address + vaddr_offset as u64;
    start_emu.core.write_reg(
        Register::Sp, stack_top_addr.0 as u64 + sp.0 as u64
    ).unwrap();


    // Run my beauftiful intellectuals, run
    let mut emu = start_emu.fork();
    println!("{:?}", emu.run());
    emu.reset(&start_emu);
    println!("{:?}", emu.run());

}
