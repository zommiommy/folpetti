#![feature(asm, panic_info_message)]
#![no_std]
#![no_main]

#[macro_use] mod print;

// Import the core requirements routines for the compiler
#[allow(unused_imports)]
use core_requirements;

use efi::*;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("!!! PANIC !!!\n");
    if let Some(location) = info.location() {
        print!("{}:{}:{}\n", 
            location.file(), location.line(), location.column());
    }

    if let Some(message) = info.message() {
        print!("{}\n", message);
    }

    loop {
        unsafe{ asm!("hlt");}
    }
}

/// Set the 8-th bit of cr4 to enable the performance counters
fn enable_performance_counters() {
    unsafe{
        asm!(
            "mov rax, cr4",
            "or rax, 256",
            "mov cr4, rax",
            lateout("rax") _
        );
    }
}

fn read_performance_counter() {
    // TODO add selector for performance counter
    // and figure out how to configure them
    unsafe{
        asm!(
            "rdpmc "
        )
    }
}

#[no_mangle]
extern fn efi_main(_image_handle: EfiHandle, system_table: *mut EfiSystemTable) {
    let st = unsafe {&mut *system_table};

    unsafe{
        efi::register_system_table(system_table);
    }    
    
    let start = unsafe{core::arch::x86_64::_rdtsc()};
    
    let memory_map = st.get_memory_map();

    print!("Memory Map\n");
    print!("Phys Virt #pages\n");
    for i in 0..memory_map.len() {
        let table = memory_map.get_table(i).unwrap();
        if !table.typ.avail_post_exit_boot_services() {
            print!(
                "{:08x} {:08x} {:08x} {:?} {:?}\n", 
                table.physical_start,
                table.virtual_start,
                table.number_of_pages,
                table.attribute,
                table.typ,
            );
        }
    }

    let tables = st.get_efi_tables();

    //print!("{:#4?}", tables);

    let end = unsafe{core::arch::x86_64::_rdtsc()};

    print!("It took {}\n", end - start);

    print!("{:#4?}\n", unsafe{&*system_table});

    enable_performance_counters();
    
    read_performance_counter();

    panic!("CULO");
}
