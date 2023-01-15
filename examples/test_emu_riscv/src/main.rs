
use mmu::{Mmu, Perm, PermField, VirtAddr};
use emu::riscv64gc::LinuxEmu;
use goblin::Object;

fn main() {
    let file_bytes = std::fs::read("../test_fuzz/target/riscv64gc-unknown-linux-gnu/debug/test_fuzz").unwrap();
    let elf = match Object::parse(&file_bytes).unwrap() {
        Object::Elf(elf) => {
            elf
        },
        _ => panic!(),
    };

    const EM_RISCV: u16 = 243;
    assert_eq!(elf.header.e_machine, EM_RISCV);

    let mut mmu = <Mmu<
            256, // DIRTY_BLOCK_SIZE
            true, // RAW
            true, // TAINT
        >>::new().unwrap();

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

        unsafe {
            // data from file
            mmu.write_from_slice(
                VirtAddr(segment.vm_range().start),
                &file_bytes[segment.file_range()],
                Some(perms),
            ).unwrap();
        }

        // padded with zeros if needed
        let padding_length = segment.vm_range().len() - segment.file_range().len();
        if segment.vm_range().len() > segment.file_range().len() {
            unsafe{
                mmu.write_from_slice(
                    VirtAddr(
                        segment.vm_range().start + segment.file_range().end
                    ), 
                    &vec![0; padding_length], 
                    Some(perms),
                ).unwrap();
            }
        }
    }

    let mut emu = LinuxEmu::new(mmu); 

    // FIND THE START FUNCTION and load its address in the program counter
    let start_symbol = elf.syms.iter().find(|x| 
        elf.strtab.get_at(x.st_name).unwrap() == "_start"
    ).unwrap();
    assert!(start_symbol.is_function());
    let start_address = start_symbol.st_value;
    emu.core.pc = start_address;

    println!("{:?}", emu.run());
}
