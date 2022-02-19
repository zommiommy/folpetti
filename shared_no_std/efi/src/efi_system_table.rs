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

/// Remove the system table, if present, so that it can-no-longer be accessed
pub unsafe fn unregister_system_table() {
    EFI_SYSTEM_TABLE.store(
        core::ptr::null_mut(), 
        Ordering::SeqCst,
    );
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

/// Save to the side the retrieved memory map
static mut MEMORY_MAP: MemoryMap = MemoryMap::default();


#[derive(Debug)]
pub struct MemoryMap {
    size: usize,
    mdesc_size: usize,
    mdesc_version: u32,
    data: [u8; 16 * 1024],
}

impl MemoryMap {
    const fn default() -> Self {
        MemoryMap{
            size: 0,
            mdesc_size: 0,
            mdesc_version: 0,
            data: [0; 16*1024],
        }
    }

    /// Get the number of MemoryDescriptors in the MemoryMap
    pub fn len(&self) -> usize {
        self.size / self.mdesc_size
    }

    pub fn get_table(&self, index: usize) -> Result<MemoryDescriptor, &str> {
        if index >= self.len() {
            return Err("The given index in the memory map is too big");
        }

        Ok(unsafe {
            MemoryDescriptor::from(*(
                self.data.as_ptr().add(self.mdesc_size * index) 
                as *const EfiMemoryDescriptor
            ))
        })
    }
}

impl EfiSystemTable {
    pub fn get_efi_tables(&self) -> &[EfiConfigurationTable] {
        unsafe {
            core::slice::from_raw_parts(
            self.tables, 
            self.number_of_tables
            )
        }
    }

    /// Get the memory map for the system from UEFI
    pub fn get_memory_map_and_key(&self) -> (&MemoryMap, usize) {
        unsafe {
            MEMORY_MAP.size = core::mem::size_of_val(&MEMORY_MAP.data);
            let mut key = 0;

            let ret = ((*self.boot_services).get_memory_map)(
                &mut MEMORY_MAP.size,
                MEMORY_MAP.data.as_mut_ptr(),
                &mut key,
                &mut MEMORY_MAP.mdesc_size,
                &mut MEMORY_MAP.mdesc_version
            );

            assert!(
                ret == EfiStatus::EfiSuccess, 
                "Error {:x?} while getting the memory map", ret
            );

            (&MEMORY_MAP, key)
        }
    }
}
