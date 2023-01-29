use super::*;

#[derive(Debug)]
pub struct GlobalType {
    pub val_type: ValType,
    pub is_mut: bool,
}

impl<'a> Parse<'a> for GlobalType {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let valtype = parse!(ValType, data);
        let is_mut = parse!(bool, data);
        (
            data,
            GlobalType {
                val_type: valtype,
                is_mut: is_mut,
            },
        )
    }
}
