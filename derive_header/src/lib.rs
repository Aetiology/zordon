extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HeaderNew)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        panic!("Macro must be applied to struct")
    };

    

    let field_idents: Vec<syn::Ident> = fields
        .iter()
        .map(|f| f.ident.clone().expect("Empty field name"))
        .collect();

    let expanded = quote! {
        impl #name {
            pub fn new<R: Read + Seek>(reader: &mut R) -> Result<#name, String> {
                Ok(
                    #name {
                       #(#field_idents: GenVal::new(reader)?,)*
                    }
                )
            }
        }
    };

    expanded.into()
}
