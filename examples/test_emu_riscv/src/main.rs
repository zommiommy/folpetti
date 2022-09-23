
//use mmu::MMU;
use diss::riscv64gc::{diss_riscv64gc, RV64GCUser, RV64GCPrint};
use goblin::Object;

const EM_RISCV: u16 = 243;

fn main() {
    let buffer = std::fs::read("../test_fuzz/target/riscv64gc-unknown-linux-gnu/debug/test_fuzz").unwrap();
    let elf = match Object::parse(&buffer).unwrap() {
        Object::Elf(elf) => {
            elf
        },
        _ => panic!(),
    };

    assert_eq!(elf.header.e_machine, EM_RISCV);

    // FIND THE START FUNCTION
    let start_symbol = elf.syms.iter().find(|x| 
        elf.strtab.get_at(x.st_name).unwrap() == "_start"
    ).unwrap();
    assert!(start_symbol.is_function());

    println!("{:?}", start_symbol);

    let start_address = start_symbol.st_value;

    // LOAD ALL THE SEGMENTS
    for segment in elf.program_headers {
        if segment.vm_range().contains(&(start_address as usize)) {
            println!("{:?}", segment);

            let file_offset = start_address - segment.vm_range().start as u64;
            let file_pos = segment.file_range().start + file_offset as usize;
            let start = &buffer[file_pos..file_pos + 0x100];
            println!("{:02x?}", start);

            let mut ip = file_pos;
            loop {
                let inst = u32::from_le_bytes((&buffer[ip..ip+4]).try_into().unwrap());
                println!("{:016x} {:02x?}", 
                    segment.vm_range().start + ip - segment.file_range().start,
                    inst.to_le_bytes(),
                );
                let offset = diss_riscv64gc(&mut RV64GCPrint, inst).unwrap();
                ip += offset;
            }

        }
        /*
        mmu.set(
            buffer,
            segment.vm_range(),
            segment.is_read(),
            segment.is_write(),
            segment.is_executable(),
        );
        */
    }
    

}
