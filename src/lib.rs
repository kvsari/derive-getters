//! # Derive Getters
//!
//! Boilerplate macro that produces methods of the same name as struct fields for a struct.
//! These methods return lifetimed references to their respective field.

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;

/*
#[proc_macro_derive(Getters)]
pub fn getters(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = impl_getters(&ast);
    gen.parse().unwrap()
}*/

#[proc_macro_derive(Testing)]
pub fn getters(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = impl_test(&ast);
    gen.parse().unwrap()
}

fn impl_test(ast: &syn::MacroInput) -> quote::Tokens {
    let structure = &ast.ident;
    quote! {
        impl #structure {
            fn bongle() -> bool {
                println!("Test");
                true
            }
        }
    }
}
