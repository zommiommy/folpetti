#![no_std]

use core::sync::atomic::{AtomicPtr, Ordering};

/// A pointer to the EFI system table which is saved upon the entry of the
/// kernel
///
/// We'll need access to this table to do input and output to the console
static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = 
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

/// Write a `string` to the UEFI console output
pub fn output_string(string: &str) {
    // Get the system table
    let st = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);

    // We can't do anything if it is null
    if st.is_null() { return; }

    // Get the console out pointer
    let out = unsafe {
        (*st).console_out
    };

    // Create a temporary buffer capable of holdoing 31 characters at a time
    // plus a null terminator.
    //
    // We are using UCS-2 and not UTF-16, as that's what UEFI used. THus, we 
    // don't have to worry about 32-but code points
    let mut tmp = [0_u16; 32];
    let mut in_use = 0;

    for chr in string.encode_utf16() {
        // Inject carriage return if needed. We always make sure there's room
        // for one based on the way we check the buffer length (-2 instead of 
        // -1)
        if chr == b'\n' as u16 {
            tmp[in_use] = b'\r' as u16;
            in_use += 1;
        }

        // Write a character into the buffer
        tmp[in_use] = chr;
        in_use += 1;

        if in_use == (tmp.len() - 2) {
            // Null terminate the buffer
            tmp[in_use] = 0;

            // Write out the buffer
            unsafe {
                ((*out).output_string)(out, tmp.as_ptr());

                // Clear the buffer
                in_use = 0;
            }
        }
    }

    // Write out any remaining characters
    if in_use > 0 {
        // Null terminate the buffer
        tmp[in_use] = 0;

        unsafe {
            ((*out).output_string)(out, tmp.as_ptr());
        }
    }
}

/// A collection of related interfaces. Type VOID *
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiHandle(usize);

// Status code.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum EfiStatus{
    EfiSuccess,
    EfiLoadError,
    EfiInvalidParameter,
    EfiUnsupported,
    EfiBadBufferSize,
    EfiBufferTooSmall,
    EfiNotReady,
    EfiDeviceError,
    EfiWriteProtected,
    EfiOutOfResources,
    EfiVolumeCorrupted,
    EfiVolumeFull,
    EfiNoMedia,
    EfiMediaChanged,
    EfiNotFound,
    EfiAccessDenied,
    EfiNoResponse,
    EfiNoMapping,
    EfiTimeout,
    EfiNotStarted,
    EfiAlreadyStarted,
    EfiAborted,
    EfiIcmpError,
    EfiTftpError,
    EfiProtocolError,
    EfiIncompatibleVersion,
    EfiSecurityViolation,
    EfiCrcError,
    EfiEndOfMedia,
    EfiEndOfFile,
    EfiInvalidLanguage,
    EfiCompromisedData,
    EfiHttpError,
    EfiNetworkUnreachable,
    EfiHostUnreachable,
    EfiProtocolUnreachable,
    EfiPortUnreachable,
    EfiConnectioNFin,
    EfiConnectionReset,
    EfiConnectionRefused,
    EfiWarnUnknownGlyph,
    EfiWarnDeleteFailure,
    EfiWarnWriteFailure,
    EfiWarnBufferTooSmall,
    EfiWarnStaleData,
    EfiWarnFileSystem,
    InvalidEfiStatusCode(usize),
}

impl From<usize> for EfiStatus {
    fn from(val: usize) -> Self {
        use EfiStatus::*;
        match val {
            0x0 => EfiSuccess,
            0x8000000000000001 => EfiLoadError,
            0x8000000000000002 => EfiInvalidParameter,
            0x8000000000000003 => EfiUnsupported,
            0x8000000000000004 => EfiBadBufferSize,
            0x8000000000000005 => EfiBufferTooSmall,
            0x8000000000000006 => EfiNotReady,
            0x8000000000000007 => EfiDeviceError,
            0x8000000000000008 => EfiWriteProtected,
            0x8000000000000009 => EfiOutOfResources,
            0x800000000000000a => EfiVolumeCorrupted,
            0x800000000000000b => EfiVolumeFull,
            0x800000000000000c => EfiNoMedia,
            0x800000000000000d => EfiMediaChanged,
            0x800000000000000e => EfiNotFound,
            0x800000000000000f => EfiAccessDenied,
            0x8000000000000010 => EfiNoResponse,
            0x8000000000000011 => EfiNoMapping,
            0x8000000000000012 => EfiTimeout,
            0x8000000000000013 => EfiNotStarted,
            0x8000000000000014 => EfiAlreadyStarted,
            0x8000000000000015 => EfiAborted,
            0x8000000000000016 => EfiIcmpError,
            0x8000000000000017 => EfiTftpError,
            0x8000000000000018 => EfiProtocolError,
            0x8000000000000019 => EfiIncompatibleVersion,
            0x800000000000001a => EfiSecurityViolation,
            0x800000000000001b => EfiCrcError,
            0x800000000000001c => EfiEndOfMedia,
            0x800000000000001f => EfiEndOfFile,
            0x8000000000000020 => EfiInvalidLanguage,
            0x8000000000000021 => EfiCompromisedData,
            0x8000000000000023 => EfiHttpError,
            0x8000000000000064 => EfiNetworkUnreachable,
            0x8000000000000065 => EfiHostUnreachable,
            0x8000000000000066 => EfiProtocolUnreachable,
            0x8000000000000067 => EfiPortUnreachable,
            0x8000000000000068 => EfiConnectioNFin,
            0x8000000000000069 => EfiConnectionReset,
            0x800000000000006a => EfiConnectionRefused,
            0x1 => EfiWarnUnknownGlyph,
            0x2 => EfiWarnDeleteFailure,
            0x3 => EfiWarnWriteFailure,
            0x4 => EfiWarnBufferTooSmall,
            0x5 => EfiWarnStaleData,
            0x6 => EfiWarnFileSystem,
            _ => InvalidEfiStatusCode(val),
        }
    }
}

/// A scan code and unicode value for a input keypress
#[repr(C)]
struct EfiInputKey {
    /// The scan code for the key press
    scan_code: u16,

    /// The unicode representation of the key
    unicode_char: u16,
}

/// EFI memory types
#[derive(Clone, Copy, Debug)]
#[repr(C)]
enum EfiMemoryType {
    ReservedMemoryType,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
    Invalid,
}

impl EfiMemoryType {
    fn avail_post_exit_boot_services(&self) -> bool {
        use EfiMemoryType::*;
        match self {
            BootServicesCode |
            BootServicesData | 
            ConventionalMemory |
            PersistentMemory => true,
            _ => false
        }
    }
}

impl From<u32> for EfiMemoryType {
    fn from(val: u32) -> Self {
        use EfiMemoryType::*;
        match val {
            00 => ReservedMemoryType,
            01 => LoaderCode,
            02 => LoaderData,
            03 => BootServicesCode,
            04 => BootServicesData,
            05 => RuntimeServicesCode,
            06 => RuntimeServicesData,
            07 => ConventionalMemory,
            08 => UnusableMemory,
            09 => ACPIReclaimMemory,
            10 => ACPIMemoryNVS,
            11 => MemoryMappedIO,
            12 => MemoryMappedIOPortSpace,
            13 => PalCode,
            14 => PersistentMemory,
            _  => Invalid,
        }
    }
}

/// Data structure that precedes all of the standard EFI table types.
#[derive(Debug)]
#[repr(C)]
struct EfiTableHeader {
    /// A 64-bit signature that identifies the type of table that follows.
    /// Unique signatures have been generate for the EFI System Table, the EFI
    /// Boot Services Table, and the EFI Runtime Services Table
    signature: u64,

    /// The revision of the EFI Specification to which this table conforms. The
    /// upper 16 bits of this field contains the major revision value, and the 
    /// lower 16 bits contain the minor revision value. The minor revision 
    /// values are binary coded decimales that are limited to the range of
    /// 00..99
    ///
    /// When printed or displayed UEFI spec revision is referred as <Major
    /// revision>.<Minor revision upper decimal>.<Minor revision lower decimal
    /// or 
    /// <Major revision>.<Minor revision upper decimal> in case Minor revision 
    /// lower decimal is set to 0. For example:
    ///
    /// A specification with the revision value ((2<<16) | 30) would be 
    /// referred as 2.3;
    ///
    /// A specification with the revision value ((2<<16) | 31) would be
    /// referred as 2.3.1
    revision: u32,

    /// The size, in bytes of the entire table including the EfiTableHeader`
    header_size: u32,

    /// The 32-bit CRC for the entire table. This value is computed by setting
    /// this field to 0, and computing the 32-bit CRC for `header_size` bytes.
    crc32: u32,

    /// Reserved field that must be set to 0.
    reserved: u32,
}

/// The memory descriptor for a record returned from `GetMemoryMap()`
#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct EfiMemoryDescriptor {
    /// Type of the memory region. Type EFI_MEMORY_TYPE is defined in the
    /// AllocatePages() function description.
    typ: u32,

    /// Physical address of the first byte in the memory region.
    /// PhysicalStart must be aligned on a 4kib boundaryu, and must not be above
    /// 0xfffffffffffff000. Type EFI_PHYSICAL_ADDRESS is defined in the
    /// AllocatePages() function description.
    physical_start: u64,

    /// Virtual address of the first byte in the memory region.
    /// VirtualStart must be aligned on a 4kib boundaryu, and must not be above
    /// 0xfffffffffffff000. Type EFI_Virtual_ADDRESS is defined in 
    /// "Related Definitions."
    virtual_start: u64,

    /// Number of 4KiB pages in the memory region. NumberOfPages must not be 
    /// 0, and must not be any value that would represent a memoruy page with a
    /// start address, either physical or virtual, above 0xfffffffffffff000
    number_of_pages: u64,

    /// Attributes of the memory region that describe the bit mask of
    /// capabilities for that memory region, and not necessarly the current.
    /// settings for that memory region. See the following
    /// "Memory Attribute Definitions."
    attribute: u64,
}

/// Contains a table header and pointers to all of the boot services.
#[repr(C)]
struct EfiBootServices {
    /// The table HEader for the EFI Boot Services Table. This header contains 
    /// the EFI_BOOT_SERVICES_SIGNATURE and EFI_BOOT_SERVICES_REVISION values
    /// along with the size of the EFI_BOOT_SERVICES structure and a 32-bit CRC
    /// to verify that the content of the EFI Boot Services Table are valid.
    header: EfiTableHeader,

    /// Raises the task priority level.
    _raise_tpl: usize,
    
    /// Restores/lowers the task priority level.
    _restore_tpl: usize,
    
    /// Allocates pages of a particular type.
    _allocate_pages: usize,

    /// Frees allocated pages.
    _free_pages: usize,

    /// Returns the courrent boot services memory map and memory map key.
    get_memory_map: unsafe fn(
        memory_map_size:    &mut usize,
        memory_map:         *mut u8,
        map_key:            &mut usize,
        descriptor_size:    &mut usize,
        descriptor_version: &mut u32
    ) -> EfiStatus,

    /// Allocates a pool of a particular type
    _allocate_pool: usize,

    /// Frees allocated pool.
    _free_pool: usize,

    /// Creates a general-purpose even structure.
    _create_event: usize,

    /// Sets an event to be signaled at a particular time.
    _set_timer: usize,

    /// Stops execution until an event is signaled.
    _wait_for_event: usize,

    /// Signals an event.
    _signal_event: usize,

    /// Closes and frees an event structure.
    _close_event: usize,

    /// Checks whether an event is in the signaled state.
    _check_event: usize,

    /// Install a protocol interface on a device handle.
    _install_protocol_interface: usize,

    /// Reinstalls a protocol interface on a device handle.
    _reinstall_protocol_interface: usize,

    /// Removes a protocol interface from a device handle.
    _uninstall_protocol_interface: usize,

    /// Queries a handle to determine if it supports a specified protocol.
    _handle_protocol: usize,

    /// Reserved
    _reserved: usize,

    /// Registers an event that is to be signaled whenever an interface is 
    /// installed for a specified protocol.
    _register_protocol_notify: usize,

    /// Returns an array of handles that support a specified protocol.
    _locate_handle: usize,

    /// Locates all devices on a device path that support a specified protocol
    /// and returns the handle to the device that is closes to the path.
    _locate_device_path: usize,

    /// Adds, updates, or removes a configuration table from the EFI System
    /// Table.
    _install_configuration_table: usize,

    /// Loads an EFI image into memory.
    _load_image: usize,

    /// Transfer control to a loaded image's entry poitn
    _start_image: usize,

    /// Exits the image's entry point.
    _exit: usize,

    /// Unloags an image.
    _unload_image: usize,

    /// Terminates boot serviceds.
    exit_boot_services: unsafe fn (
        image_handle: EfiHandle,
        map_key:      usize,
    ) -> EfiStatus,
}

/// This protocol is used ot obtain input form the ConsoleIn device. The EFI
/// specification requires that EFI_SIMPLE_TEXT_INPUT_PROTOCOL supports the
/// samle languages as the corresponding EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.
#[repr(C)]
struct EfiSimpleTextInputProtocol {
    /// Resets the input device hardware.
    reset: unsafe fn(
        this: *const EfiSimpleTextInputProtocol,
        extended_verification: bool
    ) -> EfiStatus,

    /// Reads the next keystroke from the input device.
    read_keystroke: unsafe fn(
        this: *const EfiSimpleTextInputProtocol,
        key: *mut EfiInputKey
    ) -> EfiStatus,

    /// Evento to use with EFI_BOOT_SERVICES.WaitForEvent() to wait for a key to
    /// to be available.
    /// We don't use the event API thus we don't expose this function pointer
    _wait_for_key: usize,
}

/// This protocol is used to control text-based output devices..
#[repr(C)]
struct EfiSimpleTextOutputProtocol {
    /// Resets the text output device hardware.
    reset: unsafe fn(
        this: *const EfiSimpleTextOutputProtocol,
        extended_verification: bool,
    ) -> EfiStatus,

    /// Writes a string to the output device.
    output_string: unsafe fn (
        this: *const EfiSimpleTextOutputProtocol,
        string: *const u16,
    ) -> EfiStatus,

    /// Verifies that all carachters in a string can be output to the target
    /// device.
    test_string: unsafe fn(
        this: *const EfiSimpleTextOutputProtocol,
        string: *const u16,
    ) -> EfiStatus,

    /// Returns information for an available text mode that the output
    /// device(s) supports.
    _query_mode: usize,

    /// Sets the output device(s) to a specified mode.
    _set_mode: usize,

    /// Sets the background and foreground colors for the OutputString() and
    /// ClearScreen() functions.
    _set_attribute: usize,

    /// Clears the output device(s) display to the currently selected
    /// background color
    _clear_screen: usize,

    /// Sets the current coordinates of the cursor position.
    _set_cursor_position: usize,
    
    /// Make the cursord visibile or invisible.
    _enable_cursor: usize,

    /// Pointer to SIMPLE_TEXT_OUTPUT_MODE data.
    _mode: usize,
}

/// Contains pointers to the runtime and boot services tables.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSystemTable {
    /// The common table header
    header: EfiTableHeader,

    /// A pointer to a null terminated string that identifies the vendor that
    /// produces the system firmware for the platform.
    firmware_vendor: *const u16,

    /// A firmware vendor specific value tat identifies the revision of the 
    /// system firmware for the platform.
    firmware_revision: u32,

    /// The handle tfor the active console input device. This handle must 
    /// support EFI_SIMPLE_TEXT_INPUT_PROTOCOL and
    /// EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL.
    console_in_handle: EfiHandle,
    
    /// A pointer ot the EFI_SIMPLE_TEXT_INPUT_PROTOCOL interface that is
    /// associated with ConsoleInHandle,
    console_in: *const EfiSimpleTextInputProtocol,

    /// A handle for the active console output device. This handle must
    /// supprot the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.
    console_out_handle: EfiHandle,

    /// A pointer to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL interface that is
    /// associated with ConsoleOutHandle.
    console_out: *const EfiSimpleTextOutputProtocol,

    /// The handle for the acrive standard error console device. This handle
    /// must support the EFIS_IMPLE_TEXT_OUTPUT_PROTOCOL.
    console_err_handle: EfiHandle,

    /// A pointer to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL interface that is
    /// associated with StandardErrorHandle.
    console_err: *const EfiSimpleTextOutputProtocol,

    /// A pointer to the EFI Runtime Services Table.
    _runtime_services: usize,

    /// A pointer to the EFI Boot Services Table.
    boot_services: *const EfiBootServices,

    /// Number of EFI tables
    number_of_tables: usize,

    /// Pointer to EFI table array
    tables: *const EfiConfigurationTable,
}

impl EfiSystemTable {
    pub fn get_acpi_table(&self) {
        let tables = unsafe {
            core::slice::from_raw_parts(
            self.tables, 
            self.number_of_tables
            )
        };

        print!("{:#4x?}\n", tables);
    }

    /// Get the memory map for the system from UEFI
    pub fn get_memory_map(&self) {

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

                print!("{:016x} {:016x} {:?}\n",
                    entry.physical_start,
                    entry.number_of_pages * 4096,
                    typ
                );
            }
        }

        print!("Total bytes free {}\n", free_memory);
    }
}

#[repr(C)]
#[derive(Debug)]
struct EfiConfigurationTable {
    /// The 128-bit GUID value that uniquely identifies the system
    /// configurationt able.
    guid: EfiGuid,

    /// A pointer to the table associated with `guid`
    table: usize,
}

/// An Efi guid representation
#[repr(C)]
#[derive(Debug)]
struct EfiGuid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}