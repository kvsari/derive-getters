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
//! derive-getters = "0.0.9"
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

use proc_macro2::{TokenTree, TokenStream, Delimiter};
use quote::quote;
use syn::{
    Data,
    Fields,
    DeriveInput,
    FieldsNamed,
    Type,
    AttrStyle,
    Ident,
    Lit,
    parse_macro_input,
    parse_str,
};

static INVALID_STRUCT: &str = "Struct must be a named struct. Not unnamed or unit.";
static INVALID_VARIANT: &str = "Variant must be a struct. Not an enum or union.";
static VALID_ATTR: &str = "Either #[getter(skip)] or #[getter(rename=\"name\")].";

enum FieldAttribute {
    Skip,
    Rename(Ident),
}

fn parse_attribute_tokens(token_stream: TokenStream) -> FieldAttribute {
    println!("ATTRIBUTE TOKENS: {:?}", token_stream);

    // There must be tokens
    let first_token_tree = token_stream
        .into_iter()
        .next()
        .expect(&format!("The getter attribute has no tokens. {}", VALID_ATTR));

    // First token tree needs to be a parentheses grouping
    let mut inner_token_iter = match first_token_tree {
        TokenTree::Group(group) => match group.delimiter() {
            Delimiter::Parenthesis => group
                .stream()
                .into_iter(),
            _ => panic!("The getter attribute grouping must be parentheses. {}",
                        VALID_ATTR),
        },
        _ => panic!("The getter attribute must have a grouping. {}", VALID_ATTR),
    };

    let second_token_tree = inner_token_iter
        .next()
        .expect(&format!("No getter option has been specified. {}", VALID_ATTR));

    let third_token_tree = inner_token_iter.next();
    let fourth_token_tree = inner_token_iter.next();
    let fifth_token_tree = inner_token_iter.next();

    // Second token needs to be either skip or rename
    match second_token_tree {
        TokenTree::Ident(ident) => if ident == "skip" {
            // Check if more tokens follow.
            if third_token_tree.is_some() {
                panic!("No further tokens must follow skip. {}", VALID_ATTR);
            }
            return FieldAttribute::Skip;
        } else if ident != "rename" {
            panic!("Invalid attribute {}. {}", &ident, VALID_ATTR);
        },
        _ => panic!("No identifier found. {}", VALID_ATTR),
    }
    
    println!("3TT: {:?}", &fourth_token_tree);
    match third_token_tree {
        Some(TokenTree::Punct(p)) => if p.as_char() != '=' {
            panic!("Punctuation must be '='. {}", VALID_ATTR);
        },
        _ => panic!("rename must be followed by '=' punctuation. {}", VALID_ATTR),
    }

    let name = match fourth_token_tree {
        Some(TokenTree::Literal(l)) => match Lit::new(l) {
            Lit::Str(lstr) => lstr.value(),
            _ => panic!("Name litera must be a string. {}", VALID_ATTR),
        },
        _ => panic!("Name must be a literal. {}", VALID_ATTR),
    };

    println!("RENAME TO: {}", &name);
    
    if fifth_token_tree.is_some() {
        panic!("No futher tokens must follow the literal in rename. {}", VALID_ATTR);
    }
    
    println!("Parsing {} into identifer...", &name);
    let new_name = match parse_str::<Ident>(&name) {
        Ok(nn) => nn,
        Err(e) => panic!("{}", e),
    };
    println!("Parsed");
    
    FieldAttribute::Rename(new_name)
}

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
            let returns = &field.ty;
            let maybie_lifetime = match &field.ty {
                Type::Reference(type_reference) => type_reference.lifetime.as_ref(),
                _ => None,
            };

            // Check for skip or rename field attributes. We deal with the last attribute.
            //let maybie_attribute: Option<&proc_macro2::TokenStream> = field.attrs
            let mf_attribute: Option<FieldAttribute> = field.attrs
                .iter()
                .fold(None, |m_last, attr| match (attr.path.is_ident("getter"), m_last) {
                    (true, _) => {
                        match attr.style {
                            AttrStyle::Outer => (),
                            AttrStyle::Inner(_) => {
                                panic!("The getter attribute is an outer not inner \
                                        attribute.");
                            }
                        }

                        Some(parse_attribute_tokens(attr.tokens.to_owned()))
                    },
                    (false, Some(last)) => Some(last),
                    (false, None) => None,
                });            

            let maybie_getter_name: Option<&Ident> = match mf_attribute {
                Some(FieldAttribute::Rename(ref name)) => Some(name),
                Some(FieldAttribute::Skip) => None,
                None => Some(field_name)
            };

            match (maybie_lifetime, maybie_getter_name) {
                (Some(lifetime), Some(getter_name)) => quote!(
                    pub fn #getter_name(&#lifetime self) -> #returns {
                        self.#field_name
                    }
                ),
                (None, Some(getter_name)) => quote!(
                    pub fn #getter_name(&self) -> &#returns {
                        &self.#field_name
                    }
                ),
                (_, None) => quote!(),
            }
        })
        .collect()
}

/// # Derive getters
/// Generate getter methods for all named struct fields in a seperate struct `impl` block.
/// Getter methods share the name of the field they're 'getting'. Methods return an
/// immutable reference to the field.
#[proc_macro_derive(Getters, attributes(getter))]
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
