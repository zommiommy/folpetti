use super::*;

#[derive(Debug)]
struct Locals {
    pub count: u32,
    pub val_type: ValType,
}

impl<'a> Parse<'a> for Locals {
    fn parse(mut data: &'a [u8]) -> (&'a [u8], Locals) {
        let count = parse!(u32, data);
        let val_type = parse!(ValType, data);
        (data, Locals { count, val_type })
    }
}

#[derive(Debug)]
/// A wiert quirks is that the locals are actually composed as:
/// Params i32s i64s f32s f64s
pub struct Function {
    pub number_of_locals_i32: u32,
    pub number_of_locals_i64: u32,
    pub number_of_locals_f32: u32,
    pub number_of_locals_f64: u32,
    pub code: Expression,
}

impl<'a> Parse<'a> for Function {
    fn parse(mut data: &'a [u8]) -> (&'a [u8], Function) {
        // Like with sections, the code size is not needed for decoding, but can
        // be used to skip functions when navigating through a binary. The
        // module is malformed if a size does not match the length of the
        // respective function code.
        let _func_len = parse!(u32, data);
        let locals = parse!(Vec::<Locals>, data);
        println!("locals {:X?}", &locals);
        let expression = parse!(Expression, data);
        println!("expression {:?}", &expression);

        let mut result = Function {
            number_of_locals_i32: 0,
            number_of_locals_i64: 0,
            number_of_locals_f32: 0,
            number_of_locals_f64: 0,
            code: expression,
        };

        for local in locals {
            match local.val_type {
                ValType::I32 => {
                    result.number_of_locals_i32 += local.count;
                }
                ValType::I64 => {
                    result.number_of_locals_i64 += local.count;
                }
                ValType::F32 => {
                    result.number_of_locals_f32 += local.count;
                }
                ValType::F64 => {
                    result.number_of_locals_f64 += local.count;
                }
                x @ _ => panic!(
                    "Local value of type {:?} is not a possible function local.",
                    x
                ),
            }
        }

        (data, result)
    }
}
