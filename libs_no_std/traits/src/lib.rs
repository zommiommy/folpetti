#![no_std]
#![cfg_attr(feature="atomic_from_mut", feature(atomic_from_mut))]
#![allow(non_snake_case)]

mod bitfields;
pub use bitfields::*;

mod atomics;
pub use atomics::*;