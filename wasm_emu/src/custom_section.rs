use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CustomSection {
    name: String,
    bytes: Vec<u8>,
}

impl<'a> Parse<'a> for CustomSection {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (bytes, name) = String::parse(data);
        (
            &[],
            CustomSection{
                name: name,
                bytes: bytes.to_vec(),
            }
        )
    }
}