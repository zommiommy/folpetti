// TODO! rewrite all this shit

#[macro_export]
/// parse &[u8] data according to the endianess,
/// If no endianess is passed, it just get the first byte
/// this is ment to do strict sequential parsing (EL(1))
macro_rules! get_field {
    ($data:expr) => {{  
        let (val, data) = $data.split_at(1);
        (data, val[0])
    }};

    ($data:expr, $t:ty, $endianess:expr) => {{  
        // split the data
        let (val, data) = $data.split_at(std::mem::size_of::<$t>());

        // parse the current value
        let result = match $endianess {
            ELFData::ELFDATA2LSB => <$t>::from_le_bytes(val.try_into().unwrap()),
            ELFData::ELFDATA2MSB => <$t>::from_be_bytes(val.try_into().unwrap()),
            _ => panic!("Unknown endianess"),
        };

        (data, result)
    }};
}

#[macro_export]
/// cast to bytes the field using the correct endianess
macro_rules! write_field {
    ($data:expr, $val:expr) => {{  
        // assign the value
        $data[0] = $val;
        // move the reference forward
        let (_, data) = $data.split_at_mut(1);

        data
    }};

    ($data:expr, $t:ty, $endianess:expr, $val:expr) => {{
        // convert the value to bytes using the correct endianess
        let result = match $endianess {
            ELFData::ELFDATA2LSB => <$t>::to_le_bytes($val),
            ELFData::ELFDATA2MSB => <$t>::to_be_bytes($val),
            _ => panic!("Unknown endianess"),
        };
        // assign the bytes
        $data[..std::mem::size_of::<$t>()].clone_from_slice(&result);
        // move the reference forward
        let (_, data) = $data.split_at_mut(std::mem::size_of::<$t>());

        data
    }};
}