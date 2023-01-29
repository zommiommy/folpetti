use super::*;

#[derive(Debug, Clone)]
pub struct FunctionType {
    pub args_type: Vec<ValType>,
    pub return_type: Option<ValType>,
}

impl<'a> Parse<'a> for FunctionType {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (mut data, byte) = get_field!(data);
        assert!(byte == 0x60);
        let args = parse!(ResultType, data);
        let returns = parse!(ResultType, data);

        let opt_returns = {
            if returns.len() == 0 {
                None
            } else {
                Some(returns[0].clone())
            }
        };

        (
            data,
            FunctionType {
                args_type: args,
                return_type: opt_returns,
            },
        )
    }
}

impl FunctionType {
    /// Check that the given arguments match the argument type definition.
    pub fn check_args(&self, args: &[Value]) -> EmuResultEmpty {
        if args.len() != self.args_type.len() {
            return Err(EmuError::TypeMismatch);
        }

        for i in 0..args.len() {
            self.args_type[i].check(&args[i])?;
        }

        Ok(())        
    }
}
