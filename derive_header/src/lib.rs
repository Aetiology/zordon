extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MutSlice)]
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
            if ty_string.starts_with("SimpleVal") {
                return quote! {let (#name, buf) = SimpleVal::new(buf)};
            } else if ty_string.starts_with("ArrayVal") {
                return quote! {let (#name, buf) = ArrayVal::new(buf)};
            } else if ty_string.starts_with("Option") {
                return quote! {#name: <#ty as ::core::default::Default>::default()};
            } else {
                return quote! {#name: <#ty>::new(buf)};
            }
        };

        converted_type(&ty_string)
    });

    let field_names = fields.iter().map(|f| {
        let name = &f.ident;

        quote! {#name}
    });

    let expanded = quote! {
        impl<'a> #struct_name<'a> {
            pub fn new(buf: &'a mut [u8]) -> (Self, &'a mut [u8]){
                    #(#new_fields;)*

                    (Self {
                       #(#field_names,)*
                    }, buf)
            }
        }
    };
    /*
        struct SimpleValTest<'a> {
        pub unsigned_8: SimpleVal<'a, u8>,
        pub unsigned_16: SimpleVal<'a, u16>,
        pub unsigned_32: SimpleVal<'a, u32>,
        pub unsigned_64: SimpleVal<'a, u64>,
        pub unsigned_arr: ArrayVal<'a, [u8; 4]>,
    }
    impl<'a> SimpleValTest<'a> {
        pub fn new(buf: &'a mut [u8]) -> (Self, &'a mut [u8]) {
            let (unsigned_8, leftovers) = SimpleVal::new(buf);
            let (unsigned_16, leftovers) = SimpleVal::new(leftovers);
            let (unsigned_32, leftovers) = SimpleVal::new(leftovers);
            let (unsigned_64, leftovers) = SimpleVal::new(leftovers);
            let (unsigned_arr, leftovers) = ArrayVal::new(leftovers);

            (
                Self {
                    unsigned_8,
                    unsigned_16,
                    unsigned_32,
                    unsigned_64,
                    unsigned_arr,
                },
                leftovers,
            )
        }
    }
        */

    expanded.into()
}
