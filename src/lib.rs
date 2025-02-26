extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Ident};

#[proc_macro_derive(FieldOffsets)]
pub fn field_offsets_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    // Extract the fields from the struct
    let fields: Vec<Ident> = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields_named) => {
                fields_named.named.iter().map(|f| f.ident.clone().unwrap()).collect()
            }
            _ => panic!("FieldOffsets can only be used with structs with named fields"),
        },
        Data::Enum(_) => panic!("FieldOffsets can only be used with structs"),
        Data::Union(_) => panic!("FieldOffsets can only be used with structs"),
    };
    let offsets = fields.iter().map(|field| {
        quote! {
            (stringify!(#field), std::mem::offset_of!(#name, #field))
        }
    });
    let len = offsets.len();
    let expanded = quote! {
        impl #name {
            pub fn field_offsets() -> [(&'static str, usize); #len] {
                [
                    #(#offsets),*
                ]
            }
        }
    };
    TokenStream::from(expanded)
}