//! # Derive Getters
//!
//! Macro for autogenerating getters. Can only be used on named structs. Will generate
//! getters that will reside in the struct namespace through an impl. If the struct already
//! has a method defined with the same name as one of the fields, this crate will barrel on
//! and you'll end up with a duplicate method name.
//!
//! ## Usage
//! Add to your project Cargo.toml;
//! ```toml
//! [dependencies]
//! derive-getters = "0.0.7"
//! ```
//!
//! In lib.rs or main.rs;
//! ```
//! #[macro_use]
//! extern crate derive_getters;
//! #
//! # fn main() { }
//! ```
//!
//! ## Example
//! ```
//! #[macro_use]
//! extern crate derive_getters;
//!
//! #[derive(Getters)]
//! struct Number {
//!    num: u64,    
//! }
//! 
//! fn main() {
//!    let number = Number { num: 655 };
//!    assert!(number.num() == &655);
//! }
//! ```
//!
//! Here, a method called `num()` has been created for the `Number` struct which gives a
//! reference to the `num` field.

extern crate proc_macro;

use std::convert::From;
use std::iter::Extend;

use quote::quote;
use syn::{Data, Fields, DeriveInput, parse_macro_input, FieldsNamed};

static INVALID_STRUCT: &str = "Struct must be a named struct. Not unnamed or unit.";
static INVALID_VARIANT: &str = "Variant must be a struct. Not an enum or union.";

fn isolate_named_fields<'a>(ast: &'a DeriveInput) -> Result<&'a FieldsNamed, &'static str> {
    match ast.data {
        Data::Struct(ref structure) => {
            match structure.fields {
                Fields::Named(ref fields) => Ok(fields),
                Fields::Unnamed(_) | Fields::Unit => Err(INVALID_STRUCT),
            }
        },
        Data::Enum(_) | Data::Union(_) => Err(INVALID_VARIANT),
    }
}

fn getters_from_fields<'a>(fields: &'a FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    fields.named
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap(); // Must never fail.
            let method_name = field_name; // It's a getter method.
            let return_type = &field.ty;

            quote!(
                pub fn #method_name(&self) -> &#return_type {
                    &self.#field_name
                }
            )
        })
        .collect()
}

/// Derive getters into a seperate trait for the named struct.
#[proc_macro_derive(Getters)]
pub fn getters(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
 
    let struct_name = &ast.ident;
    let fields = isolate_named_fields(&ast).unwrap();
    let methods = getters_from_fields(fields);

    quote!(
        impl #struct_name {
            #(#methods)*
        }
    ).into()
}
