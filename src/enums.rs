
pub use workflow_core_macros::describe_enum;
pub trait EnumTrait<T> {
    fn list()->Vec<T>;
    fn descr(&self)->&'static str;
    fn as_str(&self)->&'static str;
    fn as_str_ns(&self)->&'static str;
    fn from_str(str:&str)->Option<T>;
    fn from_str_ns(str:&str)->Option<T>;
}


#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum TryFromError {
    u32(&'static str, u32),
    u16(&'static str, u16),
    u8(&'static str, u8),
    usize(&'static str, usize),
}

#[macro_export]
macro_rules! u32_try_from {
        ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<u32> for $name {
            type Error = workflow_core::enums::TryFromError;

            fn try_from(v: u32) -> std::result::Result<Self, workflow_core::enums::TryFromError> {
                match v {
                    $(x if x == $name::$vname as u32 => Ok($name::$vname),)*
                    _ => {
                        Err(workflow_core::enums::TryFromError::u32(stringify!($name),v))
                    },
                }
            }
        }
    }
}

pub use u32_try_from;

#[macro_export]
macro_rules! u16_try_from {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
    $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<u16> for $name {
            type Error = workflow_core::enums::TryFromError;

            fn try_from(v: u16) -> std::result::Result<Self, workflow_core::enums::TryFromError> {
                match v {
                    $(x if x == $name::$vname as u16 => Ok($name::$vname),)*
                    _ => {
                        Err(workflow_core::enums::TryFromError::u16(stringify!($name),v))
                    },
                }
            }
        }

        impl std::convert::From<$name> for u16 {
            fn from(v: $name) -> u16 {
                v as u16
            }
        }
    }
}

pub use u16_try_from;

#[macro_export]
macro_rules! u8_try_from {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
    $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<u8> for $name {
            type Error = workflow_core::enums::TryFromError;

            fn try_from(v: u8) -> std::result::Result<Self, workflow_core::enums::TryFromError> {
                match v {
                    $(x if x == $name::$vname as u8 => Ok($name::$vname),)*
                    _ => {
                        Err(workflow_core::enums::TryFromError::u8(stringify!($name),v))
                    },
                }
            }
        }
    }
}

pub use u8_try_from;

#[macro_export]
macro_rules! usize_try_from {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
    $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<usize> for $name {
            type Error = workflow_core::enums::TryFromError;

            fn try_from(v: usize) -> std::result::Result<Self, workflow_core::enums::TryFromError> {
                match v {
                    $(x if x == $name::$vname as usize => Ok($name::$vname),)*
                    _ => {
                        Err(workflow_core::enums::TryFromError::u32(stringify!($name),v))
                    },
                }
            }
        }
    }
}

pub use usize_try_from;
