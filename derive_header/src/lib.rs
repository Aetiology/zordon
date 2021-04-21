extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(GenValNew)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        panic!("Macro must be applied to struct")
    };

    let new_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let ty_string = ty.to_token_stream().to_string();

        //TODO: Match using regex
        let converted_type = |ty_string: &str| {
            if ty_string.starts_with("GenVal") {
                return quote! {#name: GenVal::new(reader)?};
            } else if ty_string.starts_with("Option") {
                return quote! {#name: <#ty as ::core::default::Default>::default()};
            } else {
                return quote! {#name: <#ty>::new(reader)?};
            }
        };

        converted_type(&ty_string)
    });

    let expanded = quote! {
        impl #struct_name {
            pub fn new<R: Read + Seek>(reader: &mut R) -> Result<#struct_name, String> {
                Ok(
                    #struct_name {
                       #(#new_fields,)*
                    }
                )
            }
        }
    };

    expanded.into()
}
