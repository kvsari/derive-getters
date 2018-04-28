//! # Derive Getters
//!
//! Boilerplate macro that produces methods of the same name as struct fields for a struct.
//! These methods return lifetimed references to their respective field.

#[macro_use] extern crate quote;
extern crate proc_macro;
extern crate syn;

use std::convert::From;

use proc_macro::TokenStream;

use syn::{Data, Type, Ident, Fields, Field, token::Comma};
use syn::punctuated::Punctuated;

static INVALID_STRUCT: &str = "Struct must be a named struct. Not unnamed or unit.";
static INVALID_VARIANT: &str = "Variant must be a struct. Not an enum or union.";

/// For building a list of methods that need to be in the trait.
struct TraitSlot {
    label: Ident,
    ty: Type,
}

impl TraitSlot {
    fn new(label: Ident, ty: Type) -> TraitSlot {
        TraitSlot {
            label: label,
            ty: ty,
        }
    }
}

fn field_to_trait_slot(field: &Field) -> TraitSlot {
    let label = Ident::from(format!("get_{}", field.ident.as_ref().unwrap()).as_str());
    TraitSlot::new(label, field.ty.clone())
}

#[proc_macro_derive(Getters)]
pub fn getters(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let trait_gen = setup_getters_trait(&ast);

    // TODO
    // Add the trait_gen and impl_gen together before parsing.
    trait_gen.into()
}

/// Fetch the slots (aka fields) in a structure. If the passed in ast is not a `struct`, it
/// return `None`.
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

fn setup_getters_trait<'a>(ast: &'a syn::DeriveInput) -> quote::Tokens {
    let slots: Vec<TraitSlot> = get_slots(&ast.data)
        .unwrap_or_else(|e| panic!("Couldn't autogenerate: {}", e))
        .iter()
        .map(field_to_trait_slot)
        .collect();
    
    let trait_methods: Vec<quote::Tokens> = slots
        .into_iter()
        .map(|slot| {
            let label = slot.label;
            let ty = slot.ty;
            quote! {
                fn #label(&self) -> #ty
            }
        })
        .collect();

    let getter_trait_name = Ident::from(format!("{}Getters", &ast.ident).as_str());

    quote! {
        pub trait #getter_trait_name {
            #(#trait_methods);*
        }
    }
}
