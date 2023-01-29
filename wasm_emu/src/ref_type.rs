use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

impl<'a> Parse<'a> for RefType {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (data, byte) = get_field!(data);
        let result = match byte {
            0x70 => RefType::FuncRef,
            0x6F => RefType::ExternRef,
            _ => panic!("Cannot parse {} as RefType.", byte),
        };
        (data, result)
    }
}
