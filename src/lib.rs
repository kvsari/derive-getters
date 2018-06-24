//! Derive Getters
//! ===
//!
//! Macro for autogenerating getters. Can only be used on named structs. Will generate
//! getters that will reside in the struct namespace through an impl. If the struct already
//! has a method defined with the same name as one of the fields, this crate will barrel on
//! and you'll end up with a duplicate method name.
//!
//! # Usage
//! Add to your project Cargo.toml;
//! ```toml
//! [dependencies]
//! derive-getters = "0.0.6"
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
//! # Example
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

#[macro_use] extern crate quote;
extern crate proc_macro;
extern crate syn;

use std::convert::From;

use proc_macro::TokenStream;

use syn::{Data, Type, Ident, Fields, Field, token::Comma};
use syn::punctuated::Punctuated;

static INVALID_STRUCT: &str = "Struct must be a named struct. Not unnamed or unit.";
static INVALID_VARIANT: &str = "Variant must be a struct. Not an enum or union.";
static GETTER_PREFIX: &str = "";

/// Derive getters into a seperate trait for the named struct.
#[proc_macro_derive(Getters)]
pub fn getters(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let impl_tokens = setup_getters_impl(&ast);

    let mut tokens = quote::Tokens::new();
    tokens.append_all(impl_tokens.into_iter());
    
    tokens.into()
}

// For building a list of methods that need to be in the struct.
struct StructSlot {
    label: Ident,
    _name: Ident,
    ty: Type,
}

impl StructSlot {
    fn new(label: Ident, name: Ident, ty: Type) -> StructSlot {
        StructSlot {
            label: label,
            _name: name,
            ty: ty,
        }
    }
}

fn field_to_struct_slot(field: &Field) -> StructSlot {
    let name = field.ident.as_ref().unwrap().clone();
    let label = Ident::from(format!("{}{}", GETTER_PREFIX, &name).as_str());
    StructSlot::new(label, name, field.ty.clone())
}

// Fetch the slots (aka fields) in a structure. If the passed in ast is not a `struct`, it
// return `None`.
fn get_slots<'a>(
    data: &'a syn::Data
) -> Result<&'a Punctuated<Field, Comma>, &'static str> {
    match data {
        &Data::Struct(ref body) => {
            match &body.fields {
                &Fields::Named(ref named) => Ok(&named.named),
                _ => Err(INVALID_STRUCT),
            }
        },
        _ => Err(INVALID_VARIANT),
    }
}

fn setup_getters_impl<'a>(ast: &'a syn::DeriveInput) -> quote::Tokens {
    let slots: Vec<StructSlot> = get_slots(&ast.data)
        .unwrap_or_else(|e| panic!("Couldn't autogenerate: {}", e))
        .iter()
        .map(field_to_struct_slot)
        .collect();
    
    let struct_methods: Vec<quote::Tokens> = slots
        .iter()
        .map(|slot| {
            let label = slot.label.clone();
            let ty = slot.ty.clone();
            quote! {
                fn #label(&self) -> &#ty {
                    &self.#label
                }
            }
        })
        .collect();

    let struct_name = ast.ident.clone();

    quote! {
        impl #struct_name {
            #(#struct_methods)*
        }
    }
}
