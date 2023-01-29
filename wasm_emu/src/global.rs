use super::*;

#[derive(Debug)]
pub struct Global {
    pub global_type: GlobalType,
    pub expr: Expression,
}

impl<'a> Parse<'a> for Global {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let global_type = parse!(GlobalType, data);
        let expression = parse!(Expression, data);
        (
            data,
            Global {
                global_type: global_type,
                expr: expression,
            },
        )
    }
}
