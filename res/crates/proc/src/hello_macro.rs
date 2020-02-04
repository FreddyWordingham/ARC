//! Implementation function of the procedural macro `HelloMacro`.

use crate::proc_macro::TokenStream;
use quote::quote;

/// Add a function which reports the name of the type it is bound to.
#[inline]
#[must_use]
pub fn hello_derive_impl(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl #name {
            pub fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}
