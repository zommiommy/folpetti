use super::*;

#[derive(Debug)]
pub struct Section<'a> {
    pub section_type: SectionType,
    pub section_size: usize,
    pub section_data: &'a [u8],
}

impl<'a> Parse<'a> for Section<'a> {
    fn parse(mut data: &'a [u8]) -> (&'a [u8], Section<'a>) {
        let section_type = parse!(SectionType, data);
        let section_size = parse!(usize, data);
        let (section_data, data) = data.split_at(section_size as usize);

        (
            data,
            Section {
                section_type: section_type,
                section_size: section_size as usize,
                section_data: section_data,
            },
        )
    }
}
