//! https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf
//! https://msyksphinz-self.github.io/riscv-isadoc/html/index.html
//! https://github.com/gamozolabs/fuzz_with_emus/blob/master/src/emulator.rs
//! 
//! This should handle anything that targets `riscv64gc-unknown-linux-gnu`
use crate::utils::*;

mod regs;
pub use regs::*;

mod types;
pub use types::*;

mod user;
pub use user::*;

mod diss;
pub use diss::*;

#[cfg(feature="std")]
mod print;
#[cfg(feature="std")]
pub use print::*;