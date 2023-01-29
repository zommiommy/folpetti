use super::*;

pub type EmuResult = Result<Option<Value>, EmuError>;
pub type EmuResultEmpty = Result<(), EmuError>;

#[derive(Debug)]
pub enum EmuError {
    End,
    InvalidLocalWrite,
    InvalidLocalRead,
    StackNotEmpty,
    NotEnoughtValuesInStack,
    InvalidType,
    TypeMismatch,
    Unreachable,
    ReturnTypeMissmatch,
}