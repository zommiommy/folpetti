use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum BlockType {
    Empty,
    ValueType(ValType),
    Integer(isize),
}

impl<'a> Parse<'a> for BlockType {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        // TODO!: Clean up this part.
        match data[0] {
            0x40 => (&data[1..], BlockType::Empty),
            0x7F => (&data[1..], BlockType::ValueType(ValType::I32)),
            0x7E => (&data[1..], BlockType::ValueType(ValType::I64)),
            0x7D => (&data[1..], BlockType::ValueType(ValType::F32)),
            0x7C => (&data[1..], BlockType::ValueType(ValType::F64)),
            0x70 => (&data[1..], BlockType::ValueType(ValType::FuncRef)),
            0x6F => (&data[1..], BlockType::ValueType(ValType::ExternRef)),
            _ => {
                let (data, value) = isize::parse_signed(data, 33);
                (data, BlockType::Integer(value))
            }
        }
    }
}
