//! https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf
//! https://msyksphinz-self.github.io/riscv-isadoc/html/index.html
//! https://github.com/gamozolabs/fuzz_with_emus/blob/master/src/emulator.rs
//! 
//! This should handle anything that targets `riscv64gc-unknown-linux-gnu`
use traits::*;

mod regs;
pub use regs::*;

mod types;
pub use types::*;

mod ass;
pub use ass::*;

mod user;
pub use user::*;

mod dissa;
pub use dissa::*;

#[cfg(feature="std")]
mod print;
#[cfg(feature="std")]
pub use print::*;