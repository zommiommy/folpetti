use super::*;
use goblin::elf::Elf;
use goblin::elf64::program_header::*;
use goblin::elf64::header::*;

pub fn elf_loader(file_bytes: &[u8], elf: Elf, mmu: &mut Mmu) -> LoadingInfo {
    // if it's relocatable add an offset so we don't map in the 0x0 page
    // so we catch null derefs
    let vaddr_offset = if elf.header.e_type == ET_DYN {
        0x0000004000000000
        //0x0000004000005170
    } else {
        0x0
    };
    
    // LOAD ALL THE SEGMENTS
    let mut data_segment_idx = usize::MAX;

    // load the memory segments
    for segment in elf.program_headers {

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
            let (idx, seg) = mmu.allocate_segment(
                Some(VirtAddr(vaddr_offset + segment.vm_range().start)),
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
    let stack_size = 8 << 20; // 1MB
    let stack_top_addr = VirtAddr(stack_base_addr.0 - stack_size);

    let (stack_idx, _) = mmu.allocate_segment(
        Some(stack_top_addr),
        stack_size + 8, 
        PermField::ReadAfterWrite | PermField::Write,
    ).unwrap();

    mmu.data_segment_idx = data_segment_idx;
    mmu.stack_segment_idx = stack_idx;

    LoadingInfo{
        start_address: VirtAddr(vaddr_offset + elf.entry as usize),
        stack_address: stack_base_addr,
    }
}
