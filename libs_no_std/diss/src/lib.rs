#![cfg_attr(not(feature="std"), no_std)]
#[cfg(feature="std")]
extern crate std;

mod utils;
//pub mod armv8a_a32;
//pub mod armv8a_a64;
pub mod riscv64gc;