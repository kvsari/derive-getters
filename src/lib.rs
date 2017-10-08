//! # Derive Getters
//!
//! Boilerplate macro that produces methods of the same name as struct fields for a struct.
//! These methods return lifetimed references to their respective field.

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;

use syn::{Body, VariantData};
use quote::ToTokens;

#[proc_macro_derive(Getters)]
pub fn getters(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = impl_getters(&ast);
    gen.parse().unwrap()
}

fn impl_getters(ast: &syn::MacroInput) -> quote::Tokens {
    let ast = ast.clone();
    let structure = &ast.ident;

    let slots = match ast.body {
        Body::Struct(variants) => {
            match variants {
                VariantData::Struct(fields) => fields,
                _ => { return quote::Tokens::new(); },
            }
        },
        _ => { return quote::Tokens::new(); },
    };
    
    let mut tokens = quote::Tokens::new();

    for field in slots {
        let ty = field.ty;
        let ident = field.ident.unwrap();
        let impl_tokens = quote! {
            impl #structure {
                #[allow(dead_code)]
                pub fn #ident<'a>(&'a self) -> &'a #ty {
                    &self.#ident
                }
            }
        };

        impl_tokens.to_tokens(&mut tokens);
    }

    tokens
}
