use super::*;
use std::convert::TryInto;

#[derive(Debug)]
pub struct Limits {
    min: u32,
    max: u32,
}

impl<'a> Parse<'a> for Limits {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (mut global_data, byte) = get_field!(data);
        let result = match byte {
            // Only min
            0x0 => {
                let min = parse!(usize, global_data);
                Limits {
                    min: min.try_into().unwrap(),
                    max: u32::MAX,
                }
            }
            // both min and max
            0x1 => {
                let min = parse!(usize, global_data);
                let max = parse!(usize, global_data);
                Limits {
                    min: min.try_into().unwrap(),
                    max: max.try_into().unwrap(),
                }
            }
            _ => {
                panic!("Cannot interpret {} as limits type.", byte)
            }
        };

        (global_data, result)
    }
}
