use std::convert::TryInto;

use super::*;

/// parse &[u8] data according to the endianess,
/// If no endianess is passed, it just get the first byte
/// this is ment to do strict sequential parsing (EL(1))
#[macro_export]
macro_rules! get_field {
    ($data:expr) => {{
        let (val, data) = $data.split_at(1);
        (data, val[0])
    }};

    ($data:expr, $t:ty, little) => {{
        // split the data
        let (val, data) = $data.split_at(std::mem::size_of::<$t>());

        // parse the current value
        let result = <$t>::from_le_bytes(val.try_into().unwrap());

        (data, result)
    }};

    ($data:expr, $t:ty, big) => {{
        // split the data
        let (val, data) = $data.split_at(std::mem::size_of::<$t>());

        // parse the current value
        let result = <$t>::from_be_bytes(val.try_into().unwrap());

        (data, result)
    }};
}

#[macro_export]
macro_rules! parse {
    ($type:ty, $data:expr) => {{
        let (temp_data, val) = <$type>::parse($data);
        $data = temp_data;
        val
    }};
}

pub fn get_delimiter_offset<'a>(data: &'a [u8], delimiter: u8) -> usize {
    let mut i = 0;
    while data[i] != delimiter {
        i += 1;
    }
    i
}

impl<'a> Parse<'a> for usize {
    /// For some reason the literal integers are encoded following LEB128
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let mut result: usize = 0;
        let mut shift = 0;
        loop {
            let (newdata, byte) = get_field!(data);
            data = newdata;

            result |= (byte as usize & 0x7f) << shift;
            shift += 7;

            if byte & 0x80 == 0 {
                break;
            }
        }

        (data, result)
    }
}

impl<'a> ParseSigned<'a> for isize {
    /// For some reason the literal integers are encoded following LEB128
    fn parse_signed(mut data: &[u8], bitness: usize) -> (&[u8], isize) {
        let mut result: isize = 0;
        let mut shift = 0;
        loop {
            let (newdata, byte) = get_field!(data);
            data = newdata;

            result |= (byte as isize & 0x7f) << shift;
            shift += 7;

            if byte & 0x80 == 0 {
                if shift < bitness && (byte & 0x40) != 0 {
                    result |= !0 << shift;
                }

                return (data, result);
            }
        }
    }
}

impl<'a> Parse<'a> for u32 {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (data, value) = usize::parse(data);
        (data, value.try_into().unwrap())
    }
}

impl<'a> Parse<'a> for i32 {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (data, value) = isize::parse_signed(data, 33);
        (data, value.try_into().unwrap())
    }
}

impl<'a> Parse<'a> for i64 {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (data, value) = isize::parse_signed(data, 64);
        (data, value.try_into().unwrap())
    }
}

impl<'a> Parse<'a> for f32 {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (value, data) = data.split_at(4);
        (data, f32::from_le_bytes(value.try_into().unwrap()))
    }
}

impl<'a> Parse<'a> for f64 {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (value, data) = data.split_at(8);
        (data, f64::from_le_bytes(value.try_into().unwrap()))
    }
}

impl<'a> Parse<'a> for bool {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (data, is_mut) = get_field!(data);
        (
            data,
            match is_mut {
                0x0 => false,
                0x1 => true,
                _ => panic!("Cannot parse {} as bool.", is_mut),
            },
        )
    }
}

impl<'a> Parse<'a> for String {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (data, len) = usize::parse(data);
        let name = String::from_utf8(data[..len].to_vec()).unwrap();
        (&data[len..], name)
    }
}

impl<'a> Parse<'a> for u8 {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (val, data) = data.split_at(1);
        (data, val[0])
    }
}

impl<'a, T> Parse<'a> for Vec<T>
where
    T: Parse<'a>,
{
    /// For some reason the literal integers are encoded following LEB128
    fn parse(data: &'a [u8]) -> (&'a [u8], Self) {
        let (mut global_data, len) = u32::parse(data);
        let mut result = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let (data, val_type) = T::parse(global_data);
            global_data = data;
            result.push(val_type);
        }
        (global_data, result)
    }
}
