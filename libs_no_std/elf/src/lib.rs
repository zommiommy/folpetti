//! # Elf mainpulation crate
//! The goal of this crate is to provvide an easy-to use set of structs to 
//! read - modify - write ELf files.
//!
//! This is will be the base of a rust implemnetation of a debugger, strip,
//! and a linker.
//!
//! ```ignore
//! // Parse the elf from a slice &[u8]
//! let mut elf = ELF::parse(&buffer);
//! 
//! // Change the entrypoint 
//! elf.e_entry = 0xc0febabe;
//! 
//! // Dump the new elf with the modification
//! elf.write_file("modified");
//! ```
//! For more examples see the usage in the strip program / the debugger 
#![no_std]


#[macro_use] mod data;
pub use data::*;

mod errors;
pub use errors::*;

#[macro_use] mod utils;

mod elf;
pub use elf::ELF;

mod elf_header;
pub use elf_header::ELFHeader;

mod elf_header_enums;
pub use elf_header_enums::*;

mod section;
pub use section::Section;

mod section_enums;
pub use section_enums::*;

mod segment;
pub use segment::Segment;

mod segment_enums;
pub use segment_enums::*;