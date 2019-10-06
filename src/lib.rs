//! # Derive Getters
//! Macro for autogenerating getters. Can only be used on named structs. Will generate
//! getters that will reside in the struct namespace through an impl.
//!
//! ## Derives
//! Only named structs can derive `Getters`. Unit structs, unnamed structs, enums and
//! unions cannot derive `Getters`.
//!
//! ## Methods generated
//! The getter methods generated shall bear the same name as the struct fields and be
//! publicly visible. The methods return an immutable reference to the struct field of the
//! same name. If there is already a method defined with that name there'll be a collision.
//!
//! ## Usage
//! Add to your project Cargo.toml;
//! ```toml
//! [dependencies]
//! derive-getters = "0.0.8"
//! ```
//!
//! In lib.rs or main.rs;
//! ```
//! use derive_getters::Getters;
//! #
//! # fn main() { }
//! ```
//!
//! ### Named Structs
//! ```
//! use derive_getters::Getters;
//!
//! #[derive(Getters)]
//! struct Number {
//!     num: u64,    
//! }
//! 
//! fn main() {
//!     let number = Number { num: 655 };
//!     assert!(number.num() == &655);
//! }
//! ```
//!
//! Here, a method called `num()` has been created for the `Number` struct which gives a
//! reference to the `num` field.
//!
//! ### Generic Types
//! This macro can also derive on structs that have simple generic types. For example;
//! ```
//! # use derive_getters::Getters;
//! #[derive(Getters)]
//! struct Generic<T, U> {
//!     gen_t: T,
//!     gen_u: U,
//! }
//! #
//! # fn main() { }
//! ```
//!
//! The macro can also handle generic types with trait bounds. For example;
//! ```
//! # use derive_getters::Getters;
//! #[derive(Getters)]
//! struct Generic<T: Clone, U: Copy> {
//!     gen_t: T,
//!     gen_u: U,
//! }
//! #
//! # fn main() { }
//! ```
//! The trait bounds can also be declared in a `where` clause.
//!
//! Additionaly, simple lifetimes are OK too;
//! ```
//! # use derive_getters::Getters;
//! #[derive(Getters)]
//! struct Annotated<'a, 'b, T> {
//!     stuff: &'a T,
//!     comp: &'b str,
//!     num: u64,
//! }
//! #
//! # fn main() { }
//! ```
//!
//! ## Cannot Do
//! Const generics aren't handled by this macro nor are they tested.

extern crate proc_macro;

use std::convert::From;
use std::iter::Extend;

use quote::quote;
use syn::{
    Data,
    Fields,
    DeriveInput,
    parse_macro_input,
    FieldsNamed,
    Type,
};

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

fn getters_from_fields(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    fields.named
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap(); // Must never fail.
            let method_name = field_name; // It's a getter method.
            let returns = &field.ty;
            let maybie_lifetime = match &field.ty {
                Type::Reference(type_reference) => type_reference.lifetime.as_ref(),
                _ => None,
            };

            if let Some(lifetime) = maybie_lifetime {
                quote!(
                    pub fn #method_name(&#lifetime self) -> #returns {
                        self.#field_name
                    }
                )
            } else {
                quote!(
                    pub fn #method_name(&self) -> &#returns {
                        &self.#field_name
                    }
                )
            }
        })
        .collect()
}

/// # Derive getters
/// Generate getter methods for all named struct fields in a seperate struct `impl` block.
/// Getter methods share the name of the field they're 'getting'. Methods return an
/// immutable reference to the field.
#[proc_macro_derive(Getters)]
pub fn getters(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
 
    let struct_name = &ast.ident;
    let (impl_generics, struct_generics, where_clause) = ast.generics.split_for_impl();
    
    let fields = isolate_named_fields(&ast).unwrap();
    let methods = getters_from_fields(fields);
    
    quote!(
        impl #impl_generics #struct_name #struct_generics
            #where_clause
        {
            #(#methods)*
        }
    ).into()
}
