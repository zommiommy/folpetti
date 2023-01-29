use super::*;

#[derive(Debug)]
pub struct Import {
    pub module_name: String,
    pub name: String,
    pub description: ImportDesc,
}

impl<'a> Parse<'a> for Import {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let module_name = parse!(String, data);
        let import_name = parse!(String, data);
        let importdesc = parse!(ImportDesc, data);
        (
            data,
            Import {
                module_name: module_name,
                name: import_name,
                description: importdesc,
            },
        )
    }
}
