#![feature(panic_info_message)]
#![no_std]
#![no_main]

#[macro_use] mod print;

// Import the core requirements routines for the compiler
#[allow(unused_imports)]
use core_requirements;

use efi::*;
use core::panic::PanicInfo;
use core::arch::asm;

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
#[allow(dead_code)]
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


#[no_mangle]
extern fn efi_main(image_handle: EfiHandle, system_table: *mut EfiSystemTable) {
    let st = unsafe {&mut *system_table};

    unsafe{
        efi::register_system_table(system_table);
    }    

    println!("==================================================================");
    println!(" Pee Pee Poo Poo OS");
    println!("==================================================================");
    
    let (memory_map, key) = st.get_memory_map_and_key();

    println!(" Memory Maps");
    println!("------------------------------------------------------------------");
    println!("{:>8} {:>8} {:>8} {}", "Phys", "Virt", "#pages", "type & perms");
    for i in 0..memory_map.len() {
        let table = memory_map.get_table(i).unwrap();
        if !table.typ.avail_post_exit_boot_services() {
            println!(
                "{:08x} {:08x} {:08x} {:>20?} {:?}", 
                table.physical_start,
                table.virtual_start,
                table.number_of_pages,
                table.typ,
                table.attribute,
            );
        }
    }

    println!("==================================================================");
    println!(" Tables");
    println!("------------------------------------------------------------------");
    for table in st.get_efi_tables() {
        println!("{:08x?} {:?} {:x?}", table.vendor_table, efi::EfiGuidEnum::from(table.vendor_guid), table.vendor_guid);
    }
    println!("==================================================================");

    let apic_ptr = st.get_efi_tables().iter()
        .find(|x| 
            x.vendor_guid == efi::EfiGuidEnum::Acpi20Table.into()
        ).expect("Cannto find APIC 2.0 EFI Configuration Table");

    let xsdp =  unsafe{&*(apic_ptr.vendor_table as *const acpi::XSDP)};
    
    println!("{:#4x?}", xsdp);
    xsdp.validate().expect("Corrupted XSDP");

    let xsdt =  unsafe{&*(xsdp.xsdt_address as *const acpi::XSDT)};
    println!("{:#4x?}", xsdt);
    xsdt.validate().expect("Corrupted XSDT");
    println!("{:#4x?}", xsdt.get_entries());

    for ptr in xsdt.get_entries() {
        println!("{:?}", unsafe{&**ptr});
    }


    // Exit boot services
    unsafe{
        // exit the services
        ((*st.boot_services).exit_boot_services)(image_handle, key);
        // and unregister the table so that we canno longer access it
        efi::unregister_system_table();
    }

    panic!("CULO");
}
