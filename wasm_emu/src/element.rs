use super::*;

#[derive(Debug)]
pub enum Element {
    Zero(Expression, Vec<FunctionIdx>),
    One(Vec<FunctionIdx>),
    Two(TableIdx, Expression, Vec<FunctionIdx>),
    Three(Vec<FunctionIdx>),
    Four(Expression, Vec<Expression>),
    Five(RefType, Vec<Expression>),
    Six(TableIdx, Expression, RefType, Vec<FunctionIdx>),
    Seven(RefType, Vec<Expression>),
}

// impl<'a> Parse<'a> for Element {
//     fn parse(data: &[u8]) -> (&[u8], Self) {
//         let (mut global_data, opcode) = get_field!(data);
//
//     }
// }
