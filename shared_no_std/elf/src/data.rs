
pub enum ParseError<'a> {
    OutOfBound{
        field_name: &'static str,
    },
    InvalidBytes{
        field_name: &'static str,
        value: &'a [u8],
        is_little_endian: bool,
    }
}

pub trait Parse<T> {
    fn inner_parse_be(&mut self) -> Result<T, ParseError<'_>>;
    fn inner_parse_le(&mut self) -> Result<T, ParseError<'_>>;
}

pub struct Data<'a> {
    data: &'a [u8],
    little_endian: bool
}

impl<'a> Data<'a> {
    #[inline]
    pub fn new(data: &'a [u8]) -> Data<'a> {
        Data{
            data,
            little_endian: false,
        }
    }

    #[inline]
    pub fn set_big_endian(&mut self) {
        self.little_endian = false;
    }

    #[inline]
    pub fn set_little_endian(&mut self) {
        self.little_endian = true;
    }

    #[inline]
    pub fn parse<T>(&mut self) -> Result<T, &'static str> 
    where
        Data<'a>: Parse<T> 
    {
        Parse::<T>::inner_parse(self)
    }
}

impl <'a> Parse<u8> for Data<'a> {
    #[inline]
    fn inner_parse(&mut self) -> Result<u8, &'static str> {
        let result = self.data[0];
        self.data = &self.data[1..];
        Ok(result)
    }
}

impl <'a> Parse<i8> for Data<'a> {
    #[inline]
    fn inner_parse(&mut self) -> i8 {
        let result = i8::from_ne_bytes(self.data[..1].try_into().unwrap());
        self.data = &self.data[1..];
        result
    }
}

impl <'a, const N: usize> Parse<[u8; N]> for Data<'a> {
    #[inline]
    fn inner_parse(&mut self) -> [u8; N] {
        let result = self.data[..N].try_into().unwrap();
        self.data = &self.data[N..];
        result
    }
}

macro_rules! impl_parse_primitive {
    ($($ty: ty)*) => {
        use core::mem::size_of;
        use core::convert::TryInto;

        $(
            impl<'a> Parse<$ty> for Data<'a> {
                #[inline]
                fn inner_parse(&mut self) -> $ty {
                    // create the slice that we want to parse
                    let slice = &self.data[..size_of::<$ty>()];

                    // handle endianess
                    let converter = match self.little_endian {
                        true  => <$ty>::from_le_bytes,
                        false => <$ty>::from_be_bytes,
                    };

                    // move forward the stream
                    self.data = &self.data[size_of::<$ty>()..];
                    
                    // convert the data
                    converter(
                        slice.try_into().unwrap()
                    )
                }
            }
        )*
    }
}

impl_parse_primitive!(u16 u32 u64 u128 i16 i32 i64 i128);

macro_rules! impl_enum {
    ($(#[$doc:meta])* $enum_name:ident, $enum_repr:ty, 
        $(
            $(#[$outer:meta])* $field:ident => $value:literal,
        )*
    ) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq)]
        $(
            #[$doc]
        )*
        pub enum $enum_name {
            $(  
                $(
                    #[$outer]
                )*
                $field,
            )*
        }

        impl From<$enum_repr> for $enum_name {
            #[inline]
            fn from(item: $enum_repr) -> Self {
                match item {
                    $(
                        $value => $enum_name::$field,
                    )*
                }
            }
        }

        impl From<$enum_name> for $enum_repr {
            #[inline]
            fn from(item: $enum_name) -> Self {
                match item {
                    $(
                        $enum_name::$field => $value,
                    )*
                }
            }
        }

        impl<'a> Parse<$enum_name> for Data<'a> {
            #[inline]
            fn inner_parse(&mut self) -> $enum_name {
                $enum_name::from(self.parse::<$enum_repr>())
            }
        }
    };
}