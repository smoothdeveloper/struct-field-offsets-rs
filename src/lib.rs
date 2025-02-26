//! This crate provides a `field_offsets` member over `struct` declaration
//! that adorns the `FieldOffsets` macro.

//! For example, this can be used in FFI scenarios where asserting the offset
//! of each field among the various languages struct becomes a concern.

//! ```rust
//! use struct_field_offsets::FieldOffsets;
//!
//! // at declaration site
//! #[derive(FieldOffsets)]
//! #[repr(C)]
//! struct Data {
//!     x: i32,
//!     y: i32,
//!     label: [u8;8]
//! }
//! 
//! // in the code
//! let offsets = Data::field_offsets();
//! for (name,offset) in offsets {
//! println!("field {name} offset is {offset}.");
//! }
//! // prints:
//! // > field x offset is 0.
//! // > field y offset is 4.
//! // > field label offset is 8.
//! ```
//! 
//! In your Cargo.toml:
//! ```toml
//! [dependencies]
//! struct-field-offsets = "*"
//! ```
//!
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Ident};

/// ```rust
/// use struct_field_offsets::FieldOffsets;
///
/// // at declaration site
/// #[derive(FieldOffsets)]
/// #[repr(C)]
/// struct Data {
///     x: i32,
///     y: i32,
///     label: [u8;8]
/// }
///
/// // in the code
/// let offsets = Data::field_offsets();
/// for (name,offset) in offsets {
///     println!("field {name} offset is {offset}.");
/// }
/// // prints:
/// // > field x offset is 0.
/// // > field y offset is 4.
/// // > field label offset is 8.
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