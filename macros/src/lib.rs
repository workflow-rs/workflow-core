use proc_macro::TokenStream;

mod enums;
mod seal;

#[proc_macro_attribute]
pub fn describe_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    enums::macro_handler(attr, item)
}

#[proc_macro]
pub fn seal(input: TokenStream) -> TokenStream {
    seal::seal(input)
}
