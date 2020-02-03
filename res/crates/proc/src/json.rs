//! Implementation function of the json related procedural macros.

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

/// Implement `Save` and `Load` traits using json parsing.
pub fn form_derive_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);

    let name = match input {
        Item::Struct(s) => s.ident,
        Item::Enum(e) => e.ident,
        Item::Union(u) => u.ident,
        _ => panic!("Can not derive json for this item."),
    };

    let output = quote! {
        impl arc::file::io::Save for #name {
            #[inline]
            fn save(&self, path: &std::path::Path) {
                arc::file::io::as_json(self, path);
            }
        }

        impl arc::file::io::Load for #name {
            #[inline]
            fn load(path: &std::path::Path) -> Self {
                arc::file::io::from_json(path)
            }
        }
    };

    TokenStream::from(output)
}

/// Implement `Save` and `Load` traits using json parsing.
pub fn json_derive_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);

    let name = match input {
        Item::Struct(s) => s.ident,
        Item::Enum(e) => e.ident,
        Item::Union(u) => u.ident,
        _ => panic!("Can not derive json for this item."),
    };

    let output = quote! {
        impl crate::file::io::Save for #name {
            #[inline]
            fn save(&self, path: &std::path::Path) {
                crate::file::io::as_json(self, path);
            }
        }

        impl crate::file::io::Load for #name {
            #[inline]
            fn load(path: &std::path::Path) -> Self {
                crate::file::io::from_json(path)
            }
        }
    };

    TokenStream::from(output)
}
