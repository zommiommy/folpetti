use super::*;

#[derive(Debug)]
pub struct TableType {
    element_ref: RefType,
    limits: Limits,
}

impl<'a> Parse<'a> for TableType {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let reft = parse!(RefType, data);
        let lim = parse!(Limits, data);

        (data, TableType{
            element_ref: reft,
            limits: lim,
        })
    }
}
