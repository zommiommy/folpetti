use super::*;
use core::sync::atomic::Ordering;

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
    // We are using UCS-2 and not UTF-16, as that's what UEFI used. Thus, we 
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

/// A scan code and unicode value for a input keypress
#[repr(C)]
pub struct EfiInputKey {
    /// The scan code for the key press
    scan_code: u16,

    /// The unicode representation of the key
    unicode_char: u16,
}

/// This protocol is used ot obtain input form the ConsoleIn device. The EFI
/// specification requires that EFI_SIMPLE_TEXT_INPUT_PROTOCOL supports the
/// samle languages as the corresponding EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.
#[repr(C)]
pub struct EfiSimpleTextInputProtocol {
    /// Resets the input device hardware.
    pub reset: unsafe fn(
        this: *const EfiSimpleTextInputProtocol,
        extended_verification: bool
    ) -> EfiStatus,

    /// Reads the next keystroke from the input device.
    pub read_keystroke: unsafe fn(
        this: *const EfiSimpleTextInputProtocol,
        key: *mut EfiInputKey
    ) -> EfiStatus,

    /// Evento to use with EFI_BOOT_SERVICES.WaitForEvent() to wait for a key to
    /// to be available.
    /// We don't use the event API thus we don't expose this function pointer
    pub  _wait_for_key: usize,
}

/// This protocol is used to control text-based output devices..
#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    /// Resets the text output device hardware.
    pub reset: unsafe fn(
        this: *const EfiSimpleTextOutputProtocol,
        extended_verification: bool,
    ) -> EfiStatus,

    /// Writes a string to the output device.
    pub output_string: unsafe fn (
        this: *const EfiSimpleTextOutputProtocol,
        string: *const u16,
    ) -> EfiStatus,

    /// Verifies that all carachters in a string can be output to the target
    /// device.
    pub test_string: unsafe fn(
        this: *const EfiSimpleTextOutputProtocol,
        string: *const u16,
    ) -> EfiStatus,

    /// Returns information for an available text mode that the output
    /// device(s) supports.
    pub _query_mode: usize,

    /// Sets the output device(s) to a specified mode.
    pub _set_mode: usize,

    /// Sets the background and foreground colors for the OutputString() and
    /// ClearScreen() functions.
    pub _set_attribute: usize,

    /// Clears the output device(s) display to the currently selected
    /// background color
    pub _clear_screen: usize,

    /// Sets the current coordinates of the cursor position.
    pub _set_cursor_position: usize,
    
    /// Make the cursord visibile or invisible.
    pub _enable_cursor: usize,

    /// Pointer to SIMPLE_TEXT_OUTPUT_MODE data.
    pub _mode: usize,
}