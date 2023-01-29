use super::*;

pub trait Parse<'a> {
    fn parse(data: &'a [u8]) -> (&'a [u8], Self);
}
pub trait ParseSigned<'a> {
    fn parse_signed(data: &'a [u8], bitlen: usize) -> (&'a [u8], Self);
}

pub type TypeIdx = u32;
pub type FunctionIdx = u32;
pub type TableIdx = u32;
pub type MemIdx = u32;
pub type GlobalIdx = u32;
pub type ElementIdx = u32;
pub type DataIdx = u32;
pub type LocalIdx = u32;
pub type LabelIdx = u32;

pub type ResultType = Vec<ValType>;
