#![no_std]
#![cfg_attr(feature="atomic_from_mut", feature(atomic_from_mut))]
#![allow(non_snake_case)]
#![deny(unconditional_recursion)]

mod word;
pub use word::*;