//#![no_std]
use mmu::*;
use traits::*;
use goblin::Object;
use goblin::elf::Elf;
use goblin::elf64::program_header::*;
use goblin::elf64::header::*;

pub struct LoadingInfo {
    pub file_baseaddress: VirtAddr,
    /// The address for RIP or equivalent
    pub start_address: VirtAddr,
    /// the address for RSP or equivalent
    pub rsp: VirtAddr,

    pub loader_entry: VirtAddr,
}

pub struct Loader<'a> {
    pub ld_name: &'a str,
    pub ld_bytes: &'a [u8], 
    pub ld_addr: VirtAddr,
    pub random_value: &'a [u8],
    pub platform: &'a [u8],
    pub stack_size: usize,
    pub exec_filename: &'a [u8],
}

impl<'a> Loader<'a> {
    pub fn load_object(&mut self, file_bytes: &[u8], mmu: &mut Mmu,
        args: &[&str], envp: &[&str], auxp: &[(AT, u64)],
    ) -> LoadingInfo {
        let obj = Object::parse(file_bytes).unwrap();
        let elf = match obj {
            Object::Elf(elf) => elf,
            _ => panic!()
        };

        // if it's relocatable add an offset so we don't map in the 0x0 page
        // so we catch null derefs
        let file_baseaddress = if elf.header.e_type == ET_DYN {
            VirtAddr(0x40000000)
        } else {
            VirtAddr(0x0)
        };

        let start_address = file_baseaddress + elf.entry as usize;
    
        // mmap in the file segments
        load_segments(file_bytes, &elf, mmu, file_baseaddress);
    
        // find the last segment to allocate the brk area
        let max_addr = mmu.segments.iter().map(|(base_addr, segment)| 
            VirtAddr(base_addr.0+segment.len())
        ).max().unwrap();
        let (brk_idx, _) = mmu.allocate_segment(
            Some(max_addr),
            1, 
            PermField::ReadAfterWrite | PermField::Write,
        ).unwrap();
        mmu.brk_idx = brk_idx;
    
        // load the interpreter
        let loader_entry = if let Some(interp) = elf.interpreter {
            assert_eq!(interp, self.ld_name);
            let ld_obj = Object::parse(self.ld_bytes).unwrap();
            let ld_elf = match ld_obj {
                Object::Elf(elf) => elf,
                _ => panic!("The given ELF loader is not an elf OwO"),
            };
            load_segments(self.ld_bytes, &ld_elf, mmu, self.ld_addr);
            self.ld_addr + ld_elf.entry as usize
        } else {
            VirtAddr(0x0)
        };
    
        let mut data_size = 0;
        data_size += self.exec_filename.len().align_to_ceil(8);
        data_size += self.random_value.len().align_to_ceil(8);
        data_size += self.platform.len().align_to_ceil(8);
        data_size += envp.iter().map(|s| (s.len() + 1).align_to_ceil(8)).sum::<usize>();
        data_size += args.iter().map(|s| (s.len() + 1).align_to_ceil(8)).sum::<usize>();
        data_size = data_size.align_to_ceil(16);

        // compute the total stack size, aligning it to a page boundary
        let stack_base_addr = VirtAddr(0x8000_0000_0000); // VirtAddr(0x7fff_ffff_f000);
        // + 8 because the base_addr is owned!
        let stack_start_addr = VirtAddr(stack_base_addr.0 - self.stack_size);

        // allocate the stack
        mmu.allocate_segment(
            Some(stack_start_addr),
            self.stack_size + 8, 
            PermField::ReadAfterWrite | PermField::Write,
        ).unwrap();

        // align the start of the stack at a page boundary
        let mut data_ptr  = stack_base_addr;
        let mut rsp = stack_base_addr - data_size;

        // alloc executable filename
        data_ptr -= self.exec_filename.len().align_to_ceil(8);
        let exec_filename_add = data_ptr;
        unsafe{mmu.write_from_slice(
            data_ptr, 
            self.exec_filename,
        ).unwrap()};
        // alloc random data
        data_ptr -= self.random_value.len().align_to_ceil(8);
        let random_addr = data_ptr;
        unsafe{mmu.write_from_slice(
            data_ptr, 
            self.random_value,
        ).unwrap()};
        // alloc platform data
        data_ptr -= self.platform.len().align_to_ceil(8);
        let platform_addr = data_ptr;
        unsafe{mmu.write_from_slice(
            data_ptr, 
            self.platform,
        ).unwrap()};

        // aux data we fill in
        let base_aux = [
            (AT::NULL, 0_u64),
            (AT::EXECFN, exec_filename_add.0 as _),
            (AT::HWCAP2, 0),
            (AT::RANDOM, random_addr.0 as _),

            (AT::BASE_PLATFORM, platform_addr.0 as _),
            (AT::SECURE, 0),

            (AT::CLKTCK, 100),
            (AT::HWCAP, 0x112d),
            (AT::PLATFORM, platform_addr.0 as _),
            (AT::EGID, 0),
            (AT::GID,  0),
            (AT::EUID, 0),
            (AT::UID,  0),
            (AT::NOTELF,  0),
            (AT::ENTRY, start_address.0 as _),
            (AT::FLAGS, 0),
            (AT::BASE, self.ld_addr.0 as _),
            (AT::PAGESZ, 4096),
            (AT::PHNUM, elf.header.e_phnum as _),
            (AT::PHENT, elf.header.e_phentsize as _),
            (AT::PHDR, file_baseaddress.0 as u64 + elf.header.e_phoff),
        ];

        // write the aux data we define
        for (key, value) in base_aux {
            rsp -= 8;
            mmu.write(rsp, value).unwrap();
            rsp -= 8;
            mmu.write(rsp, key as u64).unwrap();
        }
        // write the user defined aux data
        for (key, value) in auxp {
            rsp -= 8;
            mmu.write(rsp, *value as u64).unwrap();
            rsp -= 8;
            mmu.write(rsp, (*key) as u64).unwrap();
        }

        // env NULL end
        rsp -= 8;
        mmu.write(rsp, 0_u64).unwrap();

        for env in envp {
            data_ptr -= (env.len() + 1).align_to_ceil(8);
            unsafe{
                mmu.write_from_slice_with_perm(
                    data_ptr,
                    env.as_bytes(),
                    PermField::Read | PermField::Write,
                ).unwrap();
            }

            rsp -= 8;
            mmu.write(rsp, data_ptr.0 as u64).unwrap();
        }

        // argv NULL end
        rsp -= 8;
        mmu.write(rsp, 0_u64).unwrap();

        for arg in args {
            data_ptr -= (arg.len() + 1).align_to_ceil(8);
            unsafe{
                mmu.write_from_slice_with_perm(
                    data_ptr,
                    arg.as_bytes(),
                    PermField::Read | PermField::Write,
                ).unwrap();
            }

            rsp -= 8;
            mmu.write(rsp, data_ptr.0 as u64).unwrap();
        }
        // argc
        rsp -= 8;
        mmu.write(rsp, args.len() as u64).unwrap();
        rsp -= 8;

        LoadingInfo{
            file_baseaddress,
            loader_entry,
            rsp: rsp,
            start_address,
        }
    }
}


fn load_segments(file_bytes: &[u8], elf: &Elf, mmu: &mut Mmu, base_addr: VirtAddr) {
    // load the memory segments
    for segment in &elf.program_headers {

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
        
        match segment.p_type {
            PT_GNU_STACK => {
            //    if (elf_ppnt->p_flags & PF_X)
            //        executable_stack = EXSTACK_ENABLE_X;
            //    else
            //        executable_stack = EXSTACK_DISABLE_X;
            //    break;
            }
            PT_LOAD => { //  | PT_PHDR | PT_INTERP
                let (_, seg) = mmu.allocate_segment(
                    Some(VirtAddr(base_addr.0 + segment.vm_range().start)),
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
            },
            // ignore other segments
            _ => {},
        }
    }
}

#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Copy, Clone)]
pub enum AT {
    NULL =   0x0,	/* end of vector */
    IGNORE = 0x1,	/* entry should be ignored */
    EXECFD = 0x2,	/* file descriptor of program */
    PHDR =   0x3,	/* program headers for program */
    PHENT =  0x4,	/* size of program header entry */
    PHNUM =  0x5,	/* number of program headers */
    PAGESZ = 0x6,	/* system page size */
    BASE =   0x7,	/* base address of interpreter */
    FLAGS =  0x8,	/* flags */
    ENTRY =  0x9,	/* entry point of program */
    NOTELF = 0xa,	/* program is not ELF */
    UID =    0xb,	/* real uid */
    EUID =   0xc,	/* effective uid */
    GID =    0xd,	/* real gid */
    EGID =   0xe,	/* effective gid */
    PLATFORM = 0xf,  /* string identifying CPU for optimizations */
    HWCAP =  0x10,    /* arch dependent hints at CPU capabilities */
    CLKTCK = 0x11,	/* frequency at which times() increments */

/* AT_* values 18 through 22 are reserved */

    SECURE = 0x17,   /* secure mode boolean */
    BASE_PLATFORM = 0x18,	/* string identifying real platform, may
				 * differ from AT_PLATFORM. */
    RANDOM = 0x19,	/* address of 16 random bytes */
    HWCAP2 = 0x1a,	/* extension of AT_HWCAP */

    EXECFN =  0x1f,	/* filename of program */

    MINSIGSTKSZ =	51,	/* minimal stack size for signal delivery */
}