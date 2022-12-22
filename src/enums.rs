//!
//! Ruse enum conversion utilities
//! 


pub use workflow_core_macros::describe_enum;
/// Enum trait used by the [`macro@describe_enum`] macro
pub trait EnumTrait<T> {
    /// return all permutations of the enum
    fn list()->Vec<T>;
    /// return `rustdoc` text describing the enum value
    fn descr(&self)->&'static str;
    /// return enum value as a string without namespace (i.e. `Value`)
    fn as_str(&self)->&'static str;
    /// return enum value as a string with namespace (i.e. `Enum::Value`)
    fn as_str_ns(&self)->&'static str;
    /// get enum value from the value string without namespace (i.e. `Value`)
    fn from_str(str:&str)->Option<T>;
    /// get enum value from the value string with namespace (i.e. `Enum::Value`)
    fn from_str_ns(str:&str)->Option<T>;
}

/// Error produced by the enum `try_from` macros
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum TryFromError {
    u32(&'static str, u32),
    u16(&'static str, u16),
    u8(&'static str, u8),
    usize(&'static str, usize),
}

///
/// Associates u32 values to each enum value and declares
/// a `TryFrom<u32>` implementation for this enum allowing
/// a `try_from(u32)` to enum conversion.
/// 
/// Example:
/// ```rust
/// u32_try_from!{
///     enum MyEnum {
///         A,  // 0u32
///         B,  // 1u32
///         C,  // 2u32
///     }
/// }
/// ...
/// let v1 = MyEnum::B;
/// let n: u32 = v.into();
/// let v2 = MyEnum::try_from(n);
/// assert_eq!(v1, v2);
/// ```
/// 
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

// pub use u32_try_from;

/// 
/// Associates u16 values to each enum value and declares
/// a `TryFrom<u16>` implementation for this enum allowing
/// a `try_from(u16)` to enum conversion.
/// 
/// Example:
/// ```rust
/// u16_try_from!{
///     enum MyEnum {
///         A,  // 0u16
///         B,  // 1u16
///         C,  // 2u16
///     }
/// }
/// ...
/// let v1 = MyEnum::B;
/// let n: u16 = v.into();
/// let v2 = MyEnum::try_from(n);
/// assert_eq!(v1, v2);
/// ```
/// 

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

// pub use u16_try_from;

///
///  Associates u8 values to each enum value and declares
/// a `TryFrom<u8>` implementation for this enum allowing
/// a `try_from(u8)` to enum conversion.
///
/// Example:
/// ```rust
/// u8_try_from!{
///     enum MyEnum {
///         A,  // 0u8
///         B,  // 1u8
///         C,  // 2u8
///     }
/// }
/// ...
/// let v1 = MyEnum::B;
/// let n: u8 = v.into();
/// let v2 = MyEnum::try_from(n);
/// assert_eq!(v1, v2);
/// ```
/// 

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

// pub use u8_try_from;

///
///  Associates usize values to each enum value and declares
/// a `TryFrom<usize>` implementation for this enum allowing
/// a `try_from(usize)` to enum conversion.
/// 
/// Example:
/// ```rust
/// usize_try_from!{
///     enum MyEnum {
///         A,  // 0usize
///         B,  // 1usize
///         C,  // 2usize
///     }
/// }
/// ...
/// let v1 = MyEnum::B;
/// let n: usize = v.into();
/// let v2 = MyEnum::try_from(n);
/// assert_eq!(v1, v2);
/// ```
/// 
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

// pub use usize_try_from;
