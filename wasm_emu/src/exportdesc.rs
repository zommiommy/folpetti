use super::*;

#[derive(Debug)]
#[repr(u8)]
pub enum ExportDesc {
    Function(FunctionIdx),
    Table(TableIdx),
    Memory(MemIdx),
    Global(GlobalIdx),
}

impl<'a> Parse<'a> for ExportDesc {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        use ExportDesc::*;
        let (mut global_data, byte) = get_field!(data);
        let result = match byte {
            0x00 => Function(parse!(FunctionIdx, global_data)),
            0x01 => Table(parse!(TableIdx, global_data)),
            0x02 => Memory(parse!(MemIdx, global_data)),
            0x03 => Global(parse!(GlobalIdx, global_data)),
            _ => panic!("Cannot parse Export desc {}", byte),
        };
        (global_data, result)
    }
}
