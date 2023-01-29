use super::*;

#[derive(Debug)]
#[repr(u8)]
pub enum SectionType {
    CustomSection = 0,
    TypeSection = 1,
    ImportSection = 2,
    FunctionSection = 3,
    TableSection = 4,
    MemorySection = 5,
    GlobalSection = 6,
    ExportSection = 7,
    StartSection = 8,
    ElementSection = 9,
    CodeSection = 10,
    DataSection = 11,
    DataCountSection = 12,
}

impl<'a> Parse<'a> for SectionType {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        use SectionType::*;
        let (data, byte) = get_field!(data);
        let result = match byte {
            0 => CustomSection,
            1 => TypeSection,
            2 => ImportSection,
            3 => FunctionSection,
            4 => TableSection,
            5 => MemorySection,
            6 => GlobalSection,
            7 => ExportSection,
            8 => StartSection,
            9 => ElementSection,
            10 => CodeSection,
            11 => DataSection,
            12 => DataCountSection,
            _ => panic!("Cannot parse section type {}", byte),
        };
        (data, result)
    }
}
