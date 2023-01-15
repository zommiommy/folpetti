#![feature(core_intrinsics)]
#![cfg_attr(not(feature="std"), no_std)]
//#![deny(unsafe_code)]
//#![deny(unreachable_patterns)]
//#![deny(unreachable_code)]

#[cfg(feature="std")]
extern crate std;

pub mod riscv64gc;