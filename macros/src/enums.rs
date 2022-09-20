use std::convert::Into;
use proc_macro::TokenStream;
use proc_macro2::{
    TokenTree //, Literal
};
use quote::{quote, ToTokens};
use syn::{
    Error, Ident, Variant,
};
use syn::{
    parse_macro_input,
    DeriveInput,
};
use workflow_macro_tools::attributes::*;

#[derive(Debug)]
struct Enum {
    pub args : Args,
    variant: Variant,
}

impl ToTokens for Enum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.variant.to_tokens(tokens);
    }
}

pub fn macro_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {

    let enum_decl_ast = item.clone();
    let ast = parse_macro_input!(enum_decl_ast as DeriveInput);

    let enum_attrs = &ast.attrs;

    let enum_name = &ast.ident;
    let _enum_params = &ast.generics;

    let enum_fields = if let syn::Data::Enum(syn::DataEnum {
        variants,
        ..
    }) = ast.data
    {
        variants
    } else {
        return Error::new_spanned(
            enum_name,
            format!("#[enums] macro only supports enums")
        )
        .to_compile_error()
        .into();
    };

    let mut enums : Vec<Enum> = Vec::new();
    for variant in enum_fields.iter() {
        let name: String = variant.to_token_stream().to_string();
        let attrs: Vec<_> =
            variant.attrs.iter().filter(|attr| attr.path.is_ident("descr") || attr.path.is_ident("describe")).collect();
        if attrs.len() > 1 {
            return Error::new_spanned(
                enum_name,
                format!("#[field]: more than one #[field()] attributes while processing {}", name)
            )
            .to_compile_error()
            .into();
        }

        let args = match attrs.first() {
            Some(attr) => {
                match get_attributes(attr) {
                    Some(attr) => attr,
                    _ => { Args::new() }
                }
            },
            _ => { Args::new() }
        };

        let mut docs = Vec::new();
        for attr in variant.attrs.iter() {

            let path_seg = attr.path.segments.last();
            if !path_seg.is_some() { continue }
            let segment = path_seg.unwrap();
            if segment.ident == "doc" {
                let mut tokens = attr.tokens.clone().into_iter();
                match tokens.next() {
                    Some(TokenTree::Punct(_punct)) => {
                        match tokens.next() {
                            Some(TokenTree::Literal(lit)) => {
                                docs.push(lit.clone());
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }

        let enum_instance = Enum {
            args,
            variant : variant.clone(),
        };

        enums.push(enum_instance);
    }

    
    let entries : Vec<Ident> = enums.iter().map(|e|e.variant.ident.clone()).collect();

    let strings : Vec<String> = entries.iter().map(|ident|{
        ident.to_string()    
    }).collect();
    let strings_ns : Vec<String> = entries.iter().map(|ident|{
        format!("{}::{}", enum_name, ident)
    }).collect();

    let mut descr : Vec<String> = Vec::new();
    for e in enums.iter() {
        let have_key = e.args.has("default");
        if !have_key{
            descr.push(format!("{}", e.variant.ident.clone()));
        }else if let Some(info) = e.args.get("default").unwrap(){
            descr.push(info.to_token_stream().to_string().replace('\"', ""));
        } else {
            descr.push(format!("{}", e.variant.ident.clone()));
        }
    }

    let enum_decl = quote!{
        # (#enum_attrs) *
        pub enum #enum_name {
            #( #entries ),*
        }
    };

    #[cfg(target_arch = "bpf")]
    let enum_impl = quote!{};

    #[cfg(not(target_arch = "bpf"))]
    let enum_impl = quote!{

        impl #enum_name {

            pub fn test() -> bool { true }

            pub fn list() -> Vec<#enum_name> {
                vec![#( #enum_name::#entries ),*]
            }
            pub fn as_str(&self)->&'static str{
                match self {
                    #( #enum_name::#entries => { #strings.into() }),*
                }
            }
            pub fn as_str_ns(&self)->&'static str{
                match self {
                    #( #enum_name::#entries => { #strings_ns.into() }),*
                }
            }
            pub fn from_str(str:&str)->Option<#enum_name>{
                match str {
                    #( #strings => { Some(#enum_name::#entries) }),*
                    _ => None
                }
            }
            pub fn from_str_ns(str:&str)->Option<#enum_name>{
                match str {
                    #( #strings_ns => { Some(#enum_name::#entries) }),*
                    _ => None
                }
            }

            pub fn describe(&self) -> &'static str { 
                match self {
                    #( #enum_name::#entries => { #descr.into() }),*
                }
            }

            pub fn descr(&self) -> &'static str { 
                match self {
                    #( #enum_name::#entries => { #descr.into() }),*
                }
            }
        }

        impl workflow_core::enums::EnumTrait<#enum_name> for #enum_name {
            fn list() -> Vec<#enum_name> {
                #enum_name::list()
            }
            fn descr(&self) -> &'static str {
                self.descr()
            }
            fn as_str(&self) -> &'static str {
                self.as_str()
            }
            fn as_str_ns(&self)->&'static str{
                match self {
                    #( #enum_name::#entries => { #strings_ns.into() }),*
                }
            }
            fn from_str(str:&str)->Option<#enum_name>{
                #enum_name::from_str(str)
            }
            fn from_str_ns(str:&str)->Option<#enum_name>{
                match str {
                    #( #strings_ns => { Some(#enum_name::#entries) }),*
                    _ => None
                }
            }
        }

        impl std::fmt::Display for #enum_name{
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

    };
    
    let result = quote!{
    
        #enum_decl

        #enum_impl

    }.into();

    result
}








