use super::*;


macro_rules! pop {
    ($frame:expr) => {{
        match $frame.stack.pop() {
            Some(val) => Ok(val),
            None => Err(EmuError::NotEnoughtValuesInStack)
        }
    }};
}

macro_rules! push {
    ($frame:expr, $val:expr) => {{
        $frame.stack.push($val);
    }};
}

macro_rules! impl_unary_op {
    ($frame:expr, $val_t:ident, $lambda:expr) => {
        {
            let x = pop!($frame)?;
            match x {
                Value::$val_t(_x) => {
                    push!($frame, Value::$val_t($lambda(_x)));
                }
                _ => {
                    return Err(EmuError::TypeMismatch);
                }
            }
            $frame.inst_address += 1;
        }
    };
}

macro_rules! impl_binary_op {
    ($frame:expr, $val_t:ident, $lambda:expr) => {
        {
            let x = pop!($frame)?;
            let y = pop!($frame)?;

            match (x, y) {
                (Value::$val_t(_x), Value::$val_t(_y)) => {
                    push!($frame, Value::$val_t($lambda(_x, _y)));
                }
                _ => {
                    return Err(EmuError::TypeMismatch);
                }
            }
            $frame.inst_address += 1;
        }
    };
}


/// the main problem is how the hell we store the execution point in the 
/// current function to be able to return form func calls.
/// On normal schemes its trivial since you have a flat line of instructions
/// But for speed and correctness sake I group the blocks into intrcutions with
/// vector of instructions, so or we flat stuff
pub struct Frame {
    stack: Vec<Value>,
    locals: Vec<Value>,
    inst_address: usize,
}

pub struct Emu {
    globals: Vec<Value>,
    module: WasmModule,
}

impl Emu {
    pub fn new(module: WasmModule) -> Emu {
        Emu{
            globals: Vec::new(),
            module: module,
        }
    }

    pub fn emu_expression(&mut self, expr: &Expression, args: Vec<Value>) -> EmuResult {
        let mut frame = Frame{
            stack: Vec::new(),
            locals: args,
            inst_address: 0
        };

        let instructions = &expr.instructions;
        while frame.inst_address < instructions.len() {
            let instruction  = &instructions[frame.inst_address];
            match instruction {
                Instruction::Call(func_idx) => {
                    // setup the args
                    let mut args=  Vec::new();
                    let func_type_idx = self.module.function_types[*func_idx as usize];
                    let func_type = self.module.types[func_type_idx as usize].clone();

                    for _ in 0..func_type.args_type.len() {
                        args.push(pop!(frame)?);
                    }

                    // call the func
                    let res = self.call_unchecked(*func_idx, args)?;

                    // check that the return type match
                    match (res.clone(), &func_type.return_type) {
                        (Some(val), Some(ret_t)) => {
                            ret_t.check(&val)
                        },
                        _ => {
                            Err(EmuError::ReturnTypeMissmatch)
                        }
                    }?;
                    
                    // if there is a return, push it on the stack
                    if let Some(val) = res {
                        push!(frame, val);
                    }

                    // increase the PC
                    frame.inst_address += 1;
                }
                Instruction::Return => {
                    break
                }
                Instruction::Nop => {
                    frame.inst_address += 1;
                }
                Instruction::Unreachable => {
                    return Err(EmuError::Unreachable);
                }
                Instruction::BlockDelimiter => unreachable!("The emu should never se a delimiter. Something is wrong."),
                
                Instruction::ElseDelimiter => unreachable!("The emu should never se a delimiter. Something is wrong."),
                
                Instruction::LocalGet(local_idx) => {
                    let value = frame.locals[*local_idx as usize].clone();
                    push!(frame, value);
                    frame.inst_address += 1;
                },
                Instruction::LocalSet(local_idx) => {
                    let value = pop!(frame)?;
                    frame.locals[*local_idx as usize] = value;
                    frame.inst_address += 1;
                }
                Instruction::LocalTee(local_idx) => {
                    let value = pop!(frame)?;
                    frame.locals[*local_idx as usize] = value.clone();
                    push!(frame, value);
                    frame.inst_address += 1;
                }

                Instruction::GlobalGet(global_idx) => {
                    let value = self.globals[*global_idx as usize].clone();
                    push!(frame, value);
                    frame.inst_address += 1;
                },
                Instruction::GlobalSet(global_idx) => {
                    let value = pop!(frame)?;
                    self.globals[*global_idx as usize] = value;
                    frame.inst_address += 1;
                }

                Instruction::I32Const(x) => {
                    push!(frame, Value::I32(*x));
                    frame.inst_address += 1;
                }
                Instruction::I64Const(x) => {
                    push!(frame, Value::I64(*x));
                    frame.inst_address += 1;
                }
                Instruction::F32Const(x) => {
                    push!(frame, Value::F32(*x));
                    frame.inst_address += 1;
                }
                Instruction::F64Const(x) => {
                    push!(frame, Value::F64(*x));
                    frame.inst_address += 1;
                }
                Instruction::I32Clz => impl_unary_op!(frame, I32, |x:i32| x.leading_zeros() as i32),
                Instruction::I64Clz => impl_unary_op!(frame, I64, |x:i64| x.leading_zeros() as i64),
                Instruction::I32Ctz => impl_unary_op!(frame, I32, |x:i32| x.trailing_zeros() as i32),
                Instruction::I64Ctz => impl_unary_op!(frame, I64, |x:i64| x.trailing_zeros() as i64),
                Instruction::I32Popcnt => impl_unary_op!(frame, I32, |x:i32| x.count_ones() as i32),
                Instruction::I64Popcnt => impl_unary_op!(frame, I64, |x:i64| x.count_ones() as i64),
                Instruction::I32And => impl_binary_op!(frame, I32, |x, y| {x & y}),
                Instruction::I64And => impl_binary_op!(frame, I32, |x, y| {x & y}),
                Instruction::I32Or => impl_binary_op!(frame, I32, |x, y| {x | y}),
                Instruction::I64Or => impl_binary_op!(frame, I32, |x, y| {x | y}),
                Instruction::I32Xor => impl_binary_op!(frame, I32, |x, y| {x ^ y}),
                Instruction::I64Xor => impl_binary_op!(frame, I32, |x, y| {x ^ y}),
                Instruction::I32Shl => impl_binary_op!(frame, I32, |x, y| {x << y}),
                Instruction::I64Shl => impl_binary_op!(frame, I32, |x, y| {x << y}),

                Instruction::I32Add => impl_binary_op!(frame, I32, |x, y| {x + y}),
                Instruction::I64Add => impl_binary_op!(frame, I64, |x, y| {x + y}),
                Instruction::F32Add => impl_binary_op!(frame, F32, |x, y| {x + y}),
                Instruction::F64Add => impl_binary_op!(frame, F64, |x, y| {x + y}),

                Instruction::I32Sub => impl_binary_op!(frame, I32, |x, y| {x - y}),
                Instruction::I64Sub => impl_binary_op!(frame, I64, |x, y| {x - y}),
                Instruction::F32Sub => impl_binary_op!(frame, F32, |x, y| {x - y}),
                Instruction::F64Sub => impl_binary_op!(frame, F64, |x, y| {x - y}),

                Instruction::I32Mul => impl_binary_op!(frame, I32, |x, y| {x * y}),
                Instruction::I64Mul => impl_binary_op!(frame, I64, |x, y| {x * y}),
                Instruction::F32Mul => impl_binary_op!(frame, F32, |x, y| {x * y}),
                Instruction::F64Mul => impl_binary_op!(frame, F64, |x, y| {x * y}),

                _ => panic!("Instruction {:?} is not implemented yet.", instruction),
            }
        }

        match pop!(frame) {
            Ok(x) => Ok(Some(x)),
            Err(e) => Err(e)
        }
    }

    pub fn call(&mut self, function_idx: FunctionIdx, args: Vec<Value>) -> EmuResult {
        // check that the function and the args match.
        let func_type_idx = self.module.function_types[function_idx as usize];
        let func_type = &self.module.types[func_type_idx as usize];
        func_type.check_args(&args)?;

        self.call_unchecked(function_idx, args)
    }

    pub fn call_unchecked(&mut self, function_idx: FunctionIdx, args: Vec<Value>) -> EmuResult {
        // TODO! WTF WHY DO I NEED A CLONE HERE??
        let expr = &self.module.codes[function_idx as usize].code.clone();
        self.emu_expression(expr, args)
    }

}