use std::convert::TryInto;

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Memarg {
    pub align: u32,
    pub offset: u32,
}

impl<'a> Parse<'a> for Memarg {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let align = parse!(u32, data);
        let offset = parse!(u32, data);
        (
            data,
            Memarg {
                align: align,
                offset: offset,
            },
        )
    }
}
