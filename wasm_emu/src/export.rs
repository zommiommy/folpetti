use super::*;

#[derive(Debug)]
pub struct Export {
    pub name: String,
    pub description: ExportDesc,
}

impl<'a> Parse<'a> for Export {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let name = parse!(String, data);
        let desc = parse!(ExportDesc, data);
        (
            data,
            Export {
                name: name,
                description: desc,
            },
        )
    }
}
