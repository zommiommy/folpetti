#![feature(panic_info_message)]
#![no_std]
#![no_main]

#[macro_use] mod print;

use acpi::DescriptionHeader;
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
        println!(
            "{:08x} {:08x} {:08x} {:>20?} {:?}", 
            table.physical_start,
            table.virtual_start,
            table.number_of_pages,
            table.typ,
            table.attribute,
        );
    }

    println!("==================================================================");
    println!(" Tables");
    println!("------------------------------------------------------------------");
    for table in st.get_efi_tables() {
        println!("{:08x?} {:?} {:x?}", table.vendor_table, efi::EfiGuidEnum::from(table.vendor_guid), table.vendor_guid);
    }
    println!("==================================================================");

    let acpi_ptr = st.get(efi::EfiGuidEnum::Acpi20Table)
        .expect("Cannto find APIC 2.0 EFI Configuration Table");

    let xsdp: &acpi::XSDP =  acpi_ptr.try_into()
        .expect("Invalid table for XSDP or currpted table");
    
    println!("{:#4x?}", xsdp);

    let xsdt = xsdp.get_xsdt().expect("Corrupted XSDT");
    println!("{:#4x?}", xsdt);
    
    for ptr in xsdt.get_entries() {
        println!("{:?}", unsafe{&**ptr});
    }
    println!("------------------------------------------------------------------");

    let madt = xsdt.get_madt().expect("MADT table not found in XSDT");
    
    println!("{:x?}", madt);

    // Exit boot services
    unsafe{
        // exit the services
        ((*st.boot_services).exit_boot_services)(image_handle, key);
        // and unregister the table so that we canno longer access it
        efi::unregister_system_table();
    }

    panic!("CULO");
}
