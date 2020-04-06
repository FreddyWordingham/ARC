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
    clippy::as_conversions,
    clippy::cargo_common_metadata,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::else_if_without_else,
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::missing_const_for_fn,
    clippy::missing_inline_in_public_items,
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions,
    clippy::multiple_inherent_impl,
    clippy::option_expect_used,
    clippy::panic,
    clippy::print_stdout,
    clippy::result_expect_used,
    clippy::unreachable,
    clippy::wildcard_dependencies,
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

/// Create the attribute macro form_load.
#[proc_macro_attribute]
#[inline]
#[must_use]
pub fn form_load(metadata: TokenStream, input: TokenStream) -> TokenStream {
    form_load_impl(&metadata, input)
}

/// Create the attribute macro json.
#[proc_macro_attribute]
#[inline]
#[must_use]
pub fn json(metadata: TokenStream, input: TokenStream) -> TokenStream {
    json_impl(&metadata, input)
}

/// Create the attribute macro json_load.
#[proc_macro_attribute]
#[inline]
#[must_use]
pub fn json_load(metadata: TokenStream, input: TokenStream) -> TokenStream {
    json_load_impl(&metadata, input)
}
