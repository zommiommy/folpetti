use super::*;

/// Contains a table header and pointers to all of the boot services.
#[repr(C)]
pub struct EfiBootServices {
    /// The table HEader for the EFI Boot Services Table. This header contains 
    /// the EFI_BOOT_SERVICES_SIGNATURE and EFI_BOOT_SERVICES_REVISION values
    /// along with the size of the EFI_BOOT_SERVICES structure and a 32-bit CRC
    /// to verify that the content of the EFI Boot Services Table are valid.
    pub header: EfiTableHeader,

    /// Raises the task priority level.
    pub _raise_tpl: usize,
    
    /// Restores/lowers the task priority level.
    pub _restore_tpl: usize,
    
    /// Allocates pages of a particular type.
    pub _allocate_pages: usize,

    /// Frees allocated pages.
    pub _free_pages: usize,

    /// Returns the courrent boot services memory map and memory map key.
    pub get_memory_map: unsafe fn(
        memory_map_size:    &mut usize,
        memory_map:         *mut u8,
        map_key:            &mut usize,
        descriptor_size:    &mut usize,
        descriptor_version: &mut u32
    ) -> EfiStatus,

    /// Allocates a pool of a particular type
    pub _allocate_pool: usize,

    /// Frees allocated pool.
    pub _free_pool: usize,

    /// Creates a general-purpose even structure.
    pub _create_event: usize,

    /// Sets an event to be signaled at a particular time.
    pub _set_timer: usize,

    /// Stops execution until an event is signaled.
    pub _wait_for_event: usize,

    /// Signals an event.
    pub _signal_event: usize,

    /// Closes and frees an event structure.
    pub _close_event: usize,

    /// Checks whether an event is in the signaled state.
    pub _check_event: usize,

    /// Install a protocol interface on a device handle.
    pub _install_protocol_interface: usize,

    /// Reinstalls a protocol interface on a device handle.
    pub _reinstall_protocol_interface: usize,

    /// Removes a protocol interface from a device handle.
    pub _uninstall_protocol_interface: usize,

    /// Queries a handle to determine if it supports a specified protocol.
    pub _handle_protocol: usize,

    /// Reserved
    pub _reserved: usize,

    /// Registers an event that is to be signaled whenever an interface is 
    /// installed for a specified protocol.
    pub _register_protocol_notify: usize,

    /// Returns an array of handles that support a specified protocol.
    pub _locate_handle: usize,

    /// Locates all devices on a device path that support a specified protocol
    /// and returns the handle to the device that is closes to the path.
    pub _locate_device_path: usize,

    /// Adds, updates, or removes a configuration table from the EFI System
    /// Table.
    pub _install_configuration_table: usize,

    /// Loads an EFI image into memory.
    pub _load_image: usize,

    /// Transfer control to a loaded image's entry poitn
    pub _start_image: usize,

    /// Exits the image's entry point.
    pub _exit: usize,

    /// Unloags an image.
    pub _unload_image: usize,

    /// Terminates boot serviceds.
    pub exit_boot_services: unsafe fn (
        image_handle: EfiHandle,
        map_key:      usize,
    ) -> EfiStatus,
}
