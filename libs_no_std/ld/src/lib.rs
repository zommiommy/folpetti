#![no_std]
use mmu::*;
use goblin::Object;

mod elf_loader;
pub use elf_loader::*;

pub struct LoadingInfo {
    /// The address for RIP or equivalent
    pub start_address: VirtAddr,
    /// the address for RSP or equivalent
    pub stack_address: VirtAddr,
}

pub fn load_object(file_bytes: &[u8], mmu: &mut Mmu) -> LoadingInfo {
    let obj = Object::parse(file_bytes).unwrap();
    match obj {
        Object::Elf(elf) => elf_loader(file_bytes, elf, mmu),
        _ => todo!()
    }
}

pub fn setup_stack(mut stack_addr: VirtAddr, args: &[&str], envp: &[&str], auxp: &[&str], mmu: &mut Mmu) -> VirtAddr {
    /// how many bytes to leave between different stack data
    const DATA_REDZONES: usize = 0x100;

    let mut allocation_ptr = stack_addr;

    macro_rules! push_data {
        ($datas:expr) => {
            for data in $datas {
                let (_, allocation) = mmu.allocate_segment(
                    Some(allocation_ptr),
                    data.len() + 1, // for the final \0 
                    PermField::ReadAfterWrite | PermField::Write,
                ).unwrap();
                unsafe{allocation.write_from_slice(VirtAddr(0), data.as_bytes()).unwrap()};

                mmu.write(stack_addr, allocation_ptr.0 as u64).unwrap();
                allocation_ptr += DATA_REDZONES;
                stack_addr -= 8;
            }
            // write ending NULL
            mmu.write(stack_addr, 0 as u64).unwrap();
            stack_addr -= 8;
        };
    }

    push_data!(auxp);
    push_data!(envp);
    push_data!(args);

    stack_addr
}
