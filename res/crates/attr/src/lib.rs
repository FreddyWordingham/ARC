//! Attribute macros library.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
#![allow(
    clippy::implicit_return,
    clippy::integer_arithmetic,
    clippy::module_name_repetitions,
    clippy::panic,
    clippy::result_expect_used,
    clippy::wildcard_enum_match_arm
)]

extern crate proc_macro;
extern crate proc_macro2;

mod json;

use json::*;
use proc_macro::TokenStream;

/// Create the attribute macro form.
#[proc_macro_attribute]
#[inline]
#[must_use]
pub fn form(metadata: TokenStream, input: TokenStream) -> TokenStream {
    form_impl(&metadata, input)
}

/// Create the attribute macro json.
#[proc_macro_attribute]
#[inline]
#[must_use]
pub fn json(metadata: TokenStream, input: TokenStream) -> TokenStream {
    json_impl(&metadata, input)
}
