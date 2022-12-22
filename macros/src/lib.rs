use proc_macro::TokenStream;

mod enums;
mod seal;

/// 
/// Attribute macro for automatic conversion of enums to their string representation
/// 
/// This macro works only with pure enums (it does not support enums that have 
/// values represented as structs)
/// 
/// This macro implements the following methods:
/// 
/// ```rust
///     // returns a Vec of all enum permutations
///     fn list() -> Vec<enum>;
///     // returns the `rustdoc` description of the enum
///     fn descr(&self) -> &'static str;
///     // return the name of the value i.e. `Value`
///     fn as_str(&self) -> &'static str;
///     // return the the namespaced enum value i.e. `Enum::Value`
///     fn as_str_ns(&self)->&'static str;
///     // get enum value from the name i.e. `Value`
///     fn from_str(str:&str)->Option<enum>;
///     // get enum value from the namespaced value name i.e. `Enum::Value`
///     fn from_str_ns(str:&str)->Option<#enum_name>;
///```
/// 
#[proc_macro_attribute]
pub fn describe_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    enums::macro_handler(attr, item)
}

#[proc_macro]
pub fn seal(input: TokenStream) -> TokenStream {
    seal::seal(input)
}
