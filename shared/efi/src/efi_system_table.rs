use super::*;
use core::sync::atomic::{AtomicPtr, Ordering};

/// A pointer to the EFI system table which is saved upon the entry of the
/// kernel
///
/// We'll need access to this table to do input and output to the console
pub(crate) static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = 
    AtomicPtr::new(core::ptr::null_mut());


/// Register a system table pointer. This of course is unsafe as it requires 
/// the caller to provide a valid EFI system table pointer.
///
/// Only the first non-null system table will be stored into the
/// `EFI_SYSTEM_TABLE` global
pub unsafe fn register_system_table(system_table: *mut EfiSystemTable) {
    EFI_SYSTEM_TABLE.compare_exchange(
        core::ptr::null_mut(), 
        system_table,
        Ordering::SeqCst,
        Ordering::SeqCst,
    ).expect("Could not register the system table");
}

/// Contains pointers to the runtime and boot services tables.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSystemTable {
    /// The common table header
    pub header: EfiTableHeader,

    /// A pointer to a null terminated string that identifies the vendor that
    /// produces the system firmware for the platform.
    pub firmware_vendor: *const u16,

    /// A firmware vendor specific value tat identifies the revision of the 
    /// system firmware for the platform.
    pub firmware_revision: u32,

    /// The handle tfor the active console input device. This handle must 
    /// support EFI_SIMPLE_TEXT_INPUT_PROTOCOL and
    /// EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL.
    pub console_in_handle: EfiHandle,
    
    /// A pointer ot the EFI_SIMPLE_TEXT_INPUT_PROTOCOL interface that is
    /// associated with ConsoleInHandle,
    pub console_in: *const EfiSimpleTextInputProtocol,

    /// A handle for the active console output device. This handle must
    /// supprot the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.
    pub console_out_handle: EfiHandle,

    /// A pointer to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL interface that is
    /// associated with ConsoleOutHandle.
    pub console_out: *const EfiSimpleTextOutputProtocol,

    /// The handle for the acrive standard error console device. This handle
    /// must support the EFIS_IMPLE_TEXT_OUTPUT_PROTOCOL.
    pub console_err_handle: EfiHandle,

    /// A pointer to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL interface that is
    /// associated with StandardErrorHandle.
    pub console_err: *const EfiSimpleTextOutputProtocol,

    /// A pointer to the EFI Runtime Services Table.
    pub _runtime_services: usize,

    /// A pointer to the EFI Boot Services Table.
    pub boot_services: *const EfiBootServices,

    /// Number of EFI tables
    pub number_of_tables: usize,

    /// Pointer to EFI table array
    pub tables: *const EfiConfigurationTable,
}

impl EfiSystemTable {
    pub fn get_acpi_table(&self) {
        // TODO!
        let _tables = unsafe {
            core::slice::from_raw_parts(
            self.tables, 
            self.number_of_tables
            )
        };

        //print!("{:#4x?}\n", tables);
    }

    /// Get the memory map for the system from UEFI
    pub fn get_memory_map(&self) -> u64 {
        // TODO!

        // Create an empty memory map
        let mut memory_map = [0_u8; 16 * 1024];

        let mut free_memory = 0_u64;
        unsafe {
            let mut size = core::mem::size_of_val(&memory_map);
            let mut key = 0;
            let mut mdesc_size = 0;
            let mut mdesc_version = 0;

            let ret = ((*self.boot_services).get_memory_map)(
                &mut size,
                memory_map.as_mut_ptr(),
                &mut key,
                &mut mdesc_size,
                &mut mdesc_version
            );

            assert!(ret == EfiStatus::EfiSuccess, "Error {:x?} while getting the memory map", ret);

            for off in (0..size).step_by(mdesc_size) {
                let entry = core::ptr::read_unaligned(
                    memory_map[off..].as_ptr() as *const EfiMemoryDescriptor
                );

                let typ: EfiMemoryType = entry.typ.into();

                if typ.avail_post_exit_boot_services() {
                    free_memory += entry.number_of_pages * 4096;
                }

                //print!("{:016x} {:016x} {:?}\n",
                //     entry.physical_start,
                //     entry.number_of_pages * 4096,
                //     typ
                // );
            }
        }

        //print!("Total bytes free {}\n", free_memory);
        free_memory
    }
}
