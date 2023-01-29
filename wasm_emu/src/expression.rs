use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub instructions: Vec<Instruction>,
}

impl<'a> Parse<'a> for Expression {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let mut result = Vec::new();
        while data[0] != 0x0B {
            let inst = parse!(Instruction, data);
            result.push(inst);
        }
        let _ = parse!(u8, data);
        (
            data,
            Expression {
                instructions: result,
            },
        )
    }
}
