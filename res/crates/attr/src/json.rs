//! Implementation function of the json related attribute macros.

use proc_macro::TokenStream;

/// Create the attribute macro form.
#[inline]
#[must_use]
pub fn form_impl(_metadata: &TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote::quote! {
        #[derive(Debug, serde::Serialize, serde::Deserialize, proc::Form)]
        #input
    };
    output.into()
}

/// Create the attribute macro form_load.
#[inline]
#[must_use]
pub fn form_load_impl(_metadata: &TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote::quote! {
        #[derive(Debug, serde::Deserialize, proc::FormLoad)]
        #input
    };
    output.into()
}

/// Create the attribute macro json.
#[inline]
#[must_use]
pub fn json_impl(_metadata: &TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote::quote! {
        #[derive(Debug, serde::Serialize, serde::Deserialize, proc::Json)]
        #input
    };
    output.into()
}

/// Create the attribute macro json_load.
#[inline]
#[must_use]
pub fn json_load_impl(_metadata: &TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote::quote! {
        #[derive(Debug, serde::Deserialize, proc::JsonLoad)]
        #input
    };
    output.into()
}
