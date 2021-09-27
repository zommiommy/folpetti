#[derive(Debug)]
pub enum Error {
    /// This error is returned whenever a section is
    /// searched by name but is not found
    SectionNotFound{section_name: String},
}

impl std::string::ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::SectionNotFound{section_name} => {
                format!(
                    "The section with name {} was not found in the ELF.", 
                    section_name
                )
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;