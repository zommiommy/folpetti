use super::*;

#[derive(Debug)]
pub enum ImportDesc {
    Type(TypeIdx),
    TableType { ref_type: ValType, limits: Limits },
    MemType(Limits),
    GlobalType(GlobalType),
}

impl<'a> Parse<'a> for ImportDesc {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (mut global_data, byte) = get_field!(data);
        let result = match byte {
            // TypeIdx
            0x0 => ImportDesc::Type(parse!(usize, global_data) as _),
            // TableType
            0x1 => {
                let valtype = parse!(ValType, global_data);
                let limit = parse!(Limits, global_data);
                ImportDesc::TableType {
                    ref_type: valtype,
                    limits: limit,
                }
            }
            // MemType
            0x2 => ImportDesc::MemType(parse!(Limits, global_data)),
            // Global Type
            0x3 => ImportDesc::GlobalType(parse!(GlobalType, global_data)),

            _ => panic!("Cannot parse {} as an import desc", byte),
        };

        (global_data, result)
    }
}
