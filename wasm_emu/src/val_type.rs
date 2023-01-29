use super::*;

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum ValType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
    FuncRef = 0x70,
    ExternRef = 0x6F,
}

impl ValType {
    pub fn check(&self, value: &Value) -> EmuResultEmpty{
        match (self, value) {
            (ValType::I32, Value::I32(_)) => {},
            (ValType::I64, Value::I64(_)) => {},
            (ValType::F32, Value::F32(_)) => {},
            (ValType::F64, Value::F64(_)) => {},
            _ => {
                return Err(EmuError::TypeMismatch);
            }
        };
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
/// This is the concrete version of ValType.
pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    FuncRef(FunctionIdx),
    ExternRef(usize),
}


impl<'a> Parse<'a> for ValType {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        use ValType::*;
        let (data, byte) = get_field!(data);
        let result = match byte {
            0x7F => I32,
            0x7E => I64,
            0x7D => F32,
            0x7C => F64,
            0x70 => FuncRef,
            0x6F => ExternRef,
            _ => panic!("Cannot parse value type {}", byte),
        };
        (data, result)
    }
}
