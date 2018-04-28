//! Derive Getters
//! ===
//!
//! Macro for autogenerating getters. Can only be used on named structs. Will generate
//! getters that will reside in a new trait so as not to pollute the struct namespace.
//!
//! # Usage
//! Add to your project Cargo.toml;
//! ```toml
//! [dependencies]
//! derive-getters = "0.0.4"
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
//!    assert!(number.get_num() == &655);
//! }
//! ```

#[macro_use] extern crate quote;
extern crate proc_macro;
extern crate syn;

use std::convert::From;

use proc_macro::TokenStream;

use syn::{Data, Type, Ident, Fields, Field, token::Comma};
use syn::punctuated::Punctuated;

static INVALID_STRUCT: &str = "Struct must be a named struct. Not unnamed or unit.";
static INVALID_VARIANT: &str = "Variant must be a struct. Not an enum or union.";
static GETTER_PREFIX: &str = "get_";

type TraitSlots = Vec<TraitSlot>;
type TraitName = Ident;
type StructName = Ident;

/// Derive getters into a seperate trait for the named struct.
#[proc_macro_derive(Getters)]
pub fn getters(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let (trait_tokens, struct_name, trait_name, trait_slots) = setup_getters_trait(&ast);
    let impl_tokens = setup_trait_impl(struct_name, trait_name, trait_slots);

    let mut tokens = quote::Tokens::new();
    tokens.append_all(trait_tokens.into_iter());
    tokens.append_all(impl_tokens.into_iter());
    
    //println!("Tokens: {}", &tokens);
    tokens.into()
}

// For building a list of methods that need to be in the trait.
struct TraitSlot {
    label: Ident,
    name: Ident,
    ty: Type,
}

impl TraitSlot {
    fn new(label: Ident, name: Ident, ty: Type) -> TraitSlot {
        TraitSlot {
            label: label,
            name: name,
            ty: ty,
        }
    }
}

fn field_to_trait_slot(field: &Field) -> TraitSlot {
    let name = field.ident.as_ref().unwrap().clone();
    let label = Ident::from(format!("{}{}", GETTER_PREFIX, &name).as_str());
    TraitSlot::new(label, name, field.ty.clone())
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

fn setup_getters_trait<'a>(
    ast: &'a syn::DeriveInput
) -> (quote::Tokens, StructName, TraitName, TraitSlots)  {
    let slots: Vec<TraitSlot> = get_slots(&ast.data)
        .unwrap_or_else(|e| panic!("Couldn't autogenerate: {}", e))
        .iter()
        .map(field_to_trait_slot)
        .collect();
    
    let trait_methods: Vec<quote::Tokens> = slots
        .iter()
        .map(|slot| {
            let label = slot.label.clone();
            let ty = slot.ty.clone();
            quote! {
                fn #label(&self) -> &#ty;
            }
        })
        .collect();

    let struct_name = ast.ident.clone();
    let trait_name = Ident::from(format!("{}Getters", &struct_name).as_str());

    let tokens = quote! {
        pub trait #trait_name {
            #(#trait_methods)*
        }
    };

    (tokens, struct_name, trait_name, slots)
}

fn setup_trait_impl(
    struct_name: StructName, trait_name: TraitName, slots: TraitSlots
) -> quote::Tokens {
    let trait_methods: Vec<quote::Tokens> = slots
        .into_iter()
        .map(|slot| {
            let label = slot.label.clone();
            let name = slot.name.clone();
            let ty = slot.ty.clone();
            quote! {
                fn #label(&self) -> &#ty {
                    &self.#name
                }
            }
        })
        .collect();

    quote! {
        impl #trait_name for #struct_name {
            #(#trait_methods)*
        }
    }
}
