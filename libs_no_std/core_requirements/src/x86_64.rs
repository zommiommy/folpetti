use core::arch::asm;

/// libc `memcpy` implementation in Rust
///
/// # Parameters
/// * `dest` - Pointer to memory to copy to
/// * `src`  - Pointer to memory to copy from
/// * `n`    - Number of bytes to copy
///
/// # Return
///
/// Pointer to `dest`
#[no_mangle]
#[cfg(target_arch = "x86_64")]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8, n: usize)
    -> *mut u8 {
    asm!("rep movsb",
        inout("rcx") n => _,
        inout("rdi") dest => _,
        inout("rsi") src => _,
    );
    dest
}

/// libc `memmove` implementation in Rust
///
/// # Parameters
/// * `dest` - Pointer to memory to copy to
/// * `src`  - Pointer to memory to copy from
/// * `n`    - Number of bytes to copy
///
/// # Return
///
/// Pointer to `dest`
#[no_mangle]
pub unsafe extern fn memmove(dest: *mut u8, src: *const u8, mut n: usize)
    -> *mut u8 {
    if (dest as usize) > (src as usize) &&
        (src as usize).wrapping_add(n) > (dest as usize) {
        let overhang = dest as usize - src as usize;

        if overhang < 64 {
            // 8-byte align the dest with one bytes copies
            while n != 0 && (dest as usize).wrapping_add(n) & 0x7 != 0 {
                n = n.wrapping_sub(1);
                *dest.offset(n as isize) = *src.offset(n as isize);
            }

            // Do a reverse copy 8-bytes at a time
            while n >= 8 {
                n = n.wrapping_sub(8);

                // Read the value to copy
                let val = core::ptr::read_unaligned(
                    src.offset(n as isize) as *const u64
                );

                // Write out the value
                core::ptr::write(dest.offset(n as isize) as *mut u64, val);
            }

            // Just copy the remainder
            while n != 0 {
                n = n.wrapping_sub(1);
                *dest.offset(n as isize) = *src.offset(n as isize);
            }
            return dest;
        }

        while n >= overhang {
            //Update the length remaining
            n = n.wrapping_sub(overhang);

            // Copy the remaining parts
            let src  = src.offset(n as isize);
            let dest = dest.offset(n as isize);
            memcpy(dest, src, overhang);
        }

        // Check if we copied everything
        if n == 0 {
            return dest;
        }

        // At this point there is no longer anyu overlap that matters, just fall
        // through and ocpy the ramining parts
    }

    // Just copy forwards
    memcpy(dest, src, n);

    dest
}

/// libc `memset` implementation in Rust
///
/// # Parameters
/// * `s` - Pointer to memory to set
/// * `c` - Character to set `n` bytes in `s` to
/// * `n` - Number of bytes to set
///
/// # Return
///
/// Original pointer `s`
#[no_mangle]
#[cfg(target_arch = "x86_64")]
pub unsafe extern fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    asm!("rep stosb",
        inout("rcx") n => _,
        inout("rdi") s => _,
        in("eax") c as u32,
    );
    s
}

/// libc `memcmp` implementation in Rust
///
/// # Parameters
/// * `s1` - Pointer to memory to compare with s2
/// * `s2` - Pointer to memory to compare with s1
/// * `n`  - Number of bytes to compare
///
/// # Returns
///
/// The difference between the first unmatching byte between `s1` and `s2`, or
/// zero if the two memory regions are identical.
#[no_mangle]
#[cfg(target_arch = "x86_64")]
pub unsafe extern fn memcmp(s1: *mut u8, s2: *const u8, n: usize) -> i32 {
    let mut ii = 0;

    while ii < n {
        let a = *s1.offset(ii as isize);
        let b = *s2.offset(ii as isize);
        if a != b {
            return (a as i32).wrapping_sub(b as i32);
        }
        ii = ii.wrapping_add(1);
    }

    0
}
