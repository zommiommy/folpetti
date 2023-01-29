use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    // Control Instructions
    BlockDelimiter,
    ElseDelimiter,
    Unreachable,
    Nop,
    Block(BlockType),
    Loop(BlockType),
    IfElse(BlockType),
    Br(LabelIdx),
    BrIf(LabelIdx),
    BrTable {
        indices: Vec<LabelIdx>,
        default_idx: LabelIdx,
    },
    Return,
    Call(FunctionIdx),
    CallIndirect(TypeIdx, TableIdx),

    // Reference Instructions
    RefNull(RefType),
    RefIsNull,
    RefFunc(FunctionIdx),

    // Parametric Instructions
    Drop,
    Select,
    SelectVec(Vec<ValType>),

    // Variable Instructions
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),

    // Table Instructions
    TableGet(TableIdx),
    TableSet(TableIdx),
    TableInit(ElementIdx, TableIdx),
    ElemDrop(ElementIdx),
    TableCopy(TableIdx, TableIdx),
    TableGrow(TableIdx),
    TableSize(TableIdx),
    TableFill(TableIdx),

    // Memory Instructions
    I32Load(Memarg),
    I64Load(Memarg),
    F32Load(Memarg),
    F64Load(Memarg),
    I32Load8Signed(Memarg),
    I32Load8Unsigned(Memarg),
    I32Load16Signed(Memarg),
    I32Load16Unsigned(Memarg),
    I64Load8Signed(Memarg),
    I64Load8Unsigned(Memarg),
    I64Load16Signed(Memarg),
    I64Load16Unsigned(Memarg),
    I64Load32Signed(Memarg),
    I64Load32Unsigned(Memarg),
    I32Store(Memarg),
    I64Store(Memarg),
    F32Store(Memarg),
    F64Store(Memarg),
    I32Store8(Memarg),
    I32Store16(Memarg),
    I64Store8(Memarg),
    I64Store16(Memarg),
    I64Store32(Memarg),
    /// The memory.size instruction returns the current size of a memory.
    MemorySize,
    /// The memory.grow instruction grows memory by a given delta and returns
    /// the previous size, or −1 if enough memory cannot be allocated. This
    /// instruction opoerate in units of `page_size`.
    MemoryGrow,
    /// The memory.init instruction copies data from a passive data segment into a memory.
    MemoryInit(DataIdx),
    /// The data.drop instruction prevents further use of a passive data segment.
    /// This instruction is intended to be used as an optimization hint. After
    ///  a data segment is dropped its data can no longer be retrieved, so the
    /// memory used by this segment may be freed.
    DataDrop(DataIdx),
    /// The memory.copy instruction copies data from a source memory region to a possibly overlapping destination region.
    MemoryCopy,
    /// The memory.fill instruction sets all values in a region to a given byte
    MemoryFill,

    // Constants
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),

    // Numeric Instructions
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtSigned,
    I32LtUnsigned,
    I32GtSigned,
    I32GtUnsigned,
    I32LeSigned,
    I32LeUnsigned,
    I32GeSigned,
    I32GeUnsigned,

    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtSigned,
    I64LtUnsigned,
    I64GtSigned,
    I64GtUnsigned,
    I64LeSigned,
    I64LeUnsigned,
    I64GeSigned,
    I64GeUnsigned,

    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,

    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,

    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivSigned,
    I32DivUnsigned,
    I32ReminderSigned,
    I32ReminderUnsigned,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrSigned,
    I32ShrUnsigned,
    I32Rotl,
    I32Rotr,

    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivSigned,
    I64DivUnsigned,
    I64ReminderSigned,
    I64ReminderUnsigned,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrSigned,
    I64ShrUnsigned,
    I64Rotl,
    I64Rotr,

    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32CopySign,

    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64CopySign,

    I32WrapI64,
    I32TruncF32Signed,
    I32TruncF32Unsigned,
    I32TruncF64Signed,
    I32TruncF64Unsigned,

    I64ExtendI32Signed,
    I64ExtendI32Unsigned,
    I64TruncF32Signed,
    I64TruncF32Unsigned,
    I64TruncF64Signed,
    I64TruncF64Unsigned,
    F32ConvertI32Signed,
    F32ConvertI32Unsiged,
    F32ConvertI64Signed,
    F32ConvertI64Unsiged,
    F32DemoteF64,
    F64ConvertI32Signed,
    F64ConvertI32Unsigned,
    F64ConvertI64Signed,
    F64ConvertI64Unsigned,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,

    I32Extend8Signed,
    I32Extend16Signed,
    I64Extend8Signed,
    I64Extend16Signed,
    I64Extend32Signed,

    // Staturating instructions
    I32TruncSatF32Signed,
    I32TruncSatF32Unsigned,
    I32TruncSatF64Signed,
    I32TruncSatF64Unsigned,
    I64TruncSatF32Signed,
    I64TruncSatF32Unsigned,
    I64TruncSatF64Signed,
    I64TruncSatF64Unsigned,
}

impl<'a> Parse<'a> for Instruction {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (mut global_data, byte) = get_field!(data);
        use Instruction::*;
        let instruction = match byte {
            0x0B => BlockDelimiter,
            0x05 => ElseDelimiter,
            0x00 => Unreachable,
            0x01 => Nop,
            0x02 => {
                let block_type = parse!(BlockType, global_data);
                Block(block_type)
            }
            0x03 => {
                let block_type = parse!(BlockType, global_data);
                Loop(block_type)
            }
            0x04 => {
                let block_type = parse!(BlockType, global_data);
                IfElse(block_type)
            }
            0x0C => Br(parse!(LabelIdx, global_data)),
            0x0D => BrIf(parse!(LabelIdx, global_data)),
            0x0E => BrTable {
                indices: parse!(Vec::<LabelIdx>, global_data),
                default_idx: parse!(LabelIdx, global_data),
            },
            0x0F => Return,
            0x10 => Call(parse!(FunctionIdx, global_data)),
            0x11 => CallIndirect(parse!(TypeIdx, global_data), parse!(TableIdx, global_data)),

            // Reference Instructions
            0xD0 => RefNull(parse!(RefType, global_data)),
            0xD1 => RefIsNull,
            0xD2 => RefFunc(parse!(FunctionIdx, global_data)),

            // Parametric Instructions
            0x1A => Drop,
            0x1B => Select,
            0x1C => SelectVec(parse!(Vec::<ValType>, global_data)),

            // Variable Instructions
            0x20 => LocalGet(parse!(LocalIdx, global_data)),
            0x21 => LocalSet(parse!(LocalIdx, global_data)),
            0x22 => LocalTee(parse!(LocalIdx, global_data)),
            0x23 => GlobalGet(parse!(GlobalIdx, global_data)),
            0x24 => GlobalSet(parse!(GlobalIdx, global_data)),

            // Table Instructions
            0x25 => TableGet(parse!(TableIdx, global_data)),
            0x26 => TableSet(parse!(TableIdx, global_data)),

            // Memory Instructions
            0x28 => I32Load(parse!(Memarg, global_data)),
            0x29 => I64Load(parse!(Memarg, global_data)),
            0x2A => F32Load(parse!(Memarg, global_data)),
            0x2B => F64Load(parse!(Memarg, global_data)),
            0x2C => I32Load8Signed(parse!(Memarg, global_data)),
            0x2D => I32Load8Unsigned(parse!(Memarg, global_data)),
            0x2E => I32Load16Signed(parse!(Memarg, global_data)),
            0x2F => I32Load16Unsigned(parse!(Memarg, global_data)),
            0x30 => I64Load8Signed(parse!(Memarg, global_data)),
            0x31 => I64Load8Unsigned(parse!(Memarg, global_data)),
            0x32 => I64Load16Signed(parse!(Memarg, global_data)),
            0x33 => I64Load16Unsigned(parse!(Memarg, global_data)),
            0x34 => I64Load32Signed(parse!(Memarg, global_data)),
            0x35 => I64Load32Unsigned(parse!(Memarg, global_data)),
            0x36 => I32Store(parse!(Memarg, global_data)),
            0x37 => I64Store(parse!(Memarg, global_data)),
            0x38 => F32Store(parse!(Memarg, global_data)),
            0x39 => F64Store(parse!(Memarg, global_data)),
            0x3A => I32Store8(parse!(Memarg, global_data)),
            0x3B => I32Store16(parse!(Memarg, global_data)),
            0x3C => I64Store8(parse!(Memarg, global_data)),
            0x3D => I64Store16(parse!(Memarg, global_data)),
            0x3E => I64Store32(parse!(Memarg, global_data)),
            0x3F => {
                let (data, zero) = get_field!(global_data);
                global_data = data;
                assert_eq!(
                    zero, 0x00,
                    "After 0x3F there should always be 0x00 but there is {}",
                    zero
                );
                MemorySize
            }
            0x40 => {
                let (data, zero) = get_field!(global_data);
                global_data = data;
                assert_eq!(
                    zero, 0x00,
                    "After 0x3F there should always be 0x00 but there is {}",
                    zero
                );
                MemoryGrow
            }

            // Constants
            0x41 => I32Const(parse!(i32, global_data)),
            0x42 => I64Const(parse!(i64, global_data)),
            0x43 => F32Const(parse!(f32, global_data)),
            0x44 => F64Const(parse!(f64, global_data)),

            // Numeric Instructions
            0x45 => I32Eqz,
            0x46 => I32Eq,
            0x47 => I32Ne,
            0x48 => I32LtSigned,
            0x49 => I32LtUnsigned,
            0x4A => I32GtSigned,
            0x4B => I32GtUnsigned,
            0x4C => I32LeSigned,
            0x4D => I32LeUnsigned,
            0x4E => I32GeSigned,
            0x4F => I32GeUnsigned,

            0x50 => I64Eqz,
            0x51 => I64Eq,
            0x52 => I64Ne,
            0x53 => I64LtSigned,
            0x54 => I64LtUnsigned,
            0x55 => I64GtSigned,
            0x56 => I64GtUnsigned,
            0x57 => I64LeSigned,
            0x58 => I64LeUnsigned,
            0x59 => I64GeSigned,
            0x5A => I64GeUnsigned,

            0x5B => F32Eq,
            0x5C => F32Ne,
            0x5D => F32Lt,
            0x5E => F32Gt,
            0x5F => F32Le,
            0x60 => F32Ge,

            0x61 => F64Eq,
            0x62 => F64Ne,
            0x63 => F64Lt,
            0x64 => F64Gt,
            0x65 => F64Le,
            0x66 => F64Ge,

            0x67 => I32Clz,
            0x68 => I32Ctz,
            0x69 => I32Popcnt,
            0x6A => I32Add,
            0x6B => I32Sub,
            0x6C => I32Mul,
            0x6D => I32DivSigned,
            0x6E => I32DivUnsigned,
            0x6F => I32ReminderSigned,
            0x70 => I32ReminderUnsigned,
            0x71 => I32And,
            0x72 => I32Or,
            0x73 => I32Xor,
            0x74 => I32Shl,
            0x75 => I32ShrSigned,
            0x76 => I32ShrUnsigned,
            0x77 => I32Rotl,
            0x78 => I32Rotr,

            0x79 => I64Clz,
            0x7A => I64Ctz,
            0x7B => I64Popcnt,
            0x7C => I64Add,
            0x7D => I64Sub,
            0x7E => I64Mul,
            0x7F => I64DivSigned,
            0x80 => I64DivUnsigned,
            0x81 => I64ReminderSigned,
            0x82 => I64ReminderUnsigned,
            0x83 => I64And,
            0x84 => I64Or,
            0x85 => I64Xor,
            0x86 => I64Shl,
            0x87 => I64ShrSigned,
            0x88 => I64ShrUnsigned,
            0x89 => I64Rotl,
            0x8A => I64Rotr,

            0x8B => F32Abs,
            0x8C => F32Neg,
            0x8D => F32Ceil,
            0x8E => F32Floor,
            0x8F => F32Trunc,
            0x90 => F32Nearest,
            0x91 => F32Sqrt,
            0x92 => F32Add,
            0x93 => F32Sub,
            0x94 => F32Mul,
            0x95 => F32Div,
            0x96 => F32Min,
            0x97 => F32Max,
            0x98 => F32CopySign,

            0x99 => F64Abs,
            0x9A => F64Neg,
            0x9B => F64Ceil,
            0x9C => F64Floor,
            0x9D => F64Trunc,
            0x9E => F64Nearest,
            0x9F => F64Sqrt,
            0xA0 => F64Add,
            0xA1 => F64Sub,
            0xA2 => F64Mul,
            0xA3 => F64Div,
            0xA4 => F64Min,
            0xA5 => F64Max,
            0xA6 => F64CopySign,

            0xA7 => I32WrapI64,
            0xA8 => I32TruncF32Signed,
            0xA9 => I32TruncF32Unsigned,
            0xAA => I32TruncF64Signed,
            0xAB => I32TruncF64Unsigned,
            0xAC => I64ExtendI32Signed,
            0xAD => I64ExtendI32Unsigned,
            0xAE => I64TruncF32Signed,
            0xAF => I64TruncF32Unsigned,
            0xB0 => I64TruncF64Signed,
            0xB1 => I64TruncF64Unsigned,
            0xB2 => F32ConvertI32Signed,
            0xB3 => F32ConvertI32Unsiged,
            0xB4 => F32ConvertI64Signed,
            0xB5 => F32ConvertI64Unsiged,
            0xB6 => F32DemoteF64,
            0xB7 => F64ConvertI32Signed,
            0xB8 => F64ConvertI32Unsigned,
            0xB9 => F64ConvertI64Signed,
            0xBA => F64ConvertI64Unsigned,
            0xBB => F64PromoteF32,
            0xBC => I32ReinterpretF32,
            0xBD => I64ReinterpretF64,
            0xBE => F32ReinterpretI32,
            0xBF => F64ReinterpretI64,

            0xC0 => I32Extend8Signed,
            0xC1 => I32Extend16Signed,
            0xC2 => I64Extend8Signed,
            0xC3 => I64Extend16Signed,
            0xC4 => I64Extend32Signed,

            // SPECIAL INSTRUCTIONS
            0xFC => {
                let (data, opcode) = u32::parse(global_data);
                global_data = data;
                match opcode {
                    // Memory special instructions
                    8 => {
                        let (data, dataidx) = DataIdx::parse(global_data);
                        let (data, zero) = get_field!(data);
                        global_data = data;
                        assert_eq!(
                            zero, 0x00,
                            "In Memory init there should be a 0x00 after the data index."
                        );
                        MemoryInit(dataidx)
                    }
                    9 => {
                        let (data, dataidx) = DataIdx::parse(global_data);
                        global_data = data;
                        DataDrop(dataidx)
                    }
                    10 => {
                        let (data, zero1) = get_field!(global_data);
                        let (data, zero2) = get_field!(data);
                        global_data = data;
                        assert_eq!(zero1, 0x00, "Memory Copy should end with 0x00 0x00.");
                        assert_eq!(zero2, 0x00, "Memory Copy should end with 0x00 0x00.");
                        MemoryCopy
                    }
                    11 => MemoryFill,
                    // Table special instructions
                    12 => TableInit(
                        parse!(ElementIdx, global_data),
                        parse!(TableIdx, global_data),
                    ),
                    13 => ElemDrop(parse!(ElementIdx, global_data)),
                    14 => TableCopy(parse!(TableIdx, global_data), parse!(TableIdx, global_data)),
                    15 => TableGrow(parse!(TableIdx, global_data)),
                    16 => TableSize(parse!(TableIdx, global_data)),
                    17 => TableFill(parse!(TableIdx, global_data)),

                    _ => panic!("unkwno special instruction. 0xFC {:X}", opcode),
                }
            }

            _ => panic!(
                "The instruction which prefix {:X} is not implemented yet.",
                byte
            ),
        };
        (global_data, instruction)
    }
}
