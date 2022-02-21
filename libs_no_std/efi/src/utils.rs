
macro_rules! impl_enum {
    ($(#[$outer:meta])* $enum_name:ident, $enum_repr:ty, $($(#[$inner:ident $($args:tt)*])* $field:ident => $value:expr,)*) => {
        #[allow(non_camel_case_types)]
        #[repr(C)]
        #[derive(Debug, Clone, Copy, PartialEq)]
        $(#[$outer])*
        pub enum $enum_name {
            $(
                $(#[$inner $($args)*])*
                $field,
            )*
            /// An invalid value
            Invalid($enum_repr),
        }

        impl From<$enum_repr> for $enum_name {
            #[inline]
            fn from(item: $enum_repr) -> Self {
                match item {
                    $(
                        $value => $enum_name::$field,
                    )*
                    x @ _ => $enum_name::Invalid(x),
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
                    $enum_name::Invalid(x) => x,
                }
            }
        }
    };
}