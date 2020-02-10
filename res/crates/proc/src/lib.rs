//! Procedural macros library.

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

mod hello_macro;
mod json;
mod new;

use crate::proc_macro::TokenStream;
use hello_macro::*;
use json::*;
use new::*;

/// Create the procedural macro `HelloMacro`.
#[proc_macro_derive(HelloMacro)]
#[inline]
#[must_use]
pub fn hello_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Unable to parse token stream.");
    hello_derive_impl(&ast)
}

/// Create the procedural macro New.
#[proc_macro_derive(New)]
#[inline]
#[must_use]
pub fn new_derive(input: TokenStream) -> TokenStream {
    new_derive_impl(input)
}

/// Create the procedural macro Form.
#[proc_macro_derive(Form)]
#[inline]
#[must_use]
pub fn form_derive(input: TokenStream) -> TokenStream {
    form_derive_impl(input)
}

/// Create the procedural macro Json.
#[proc_macro_derive(Json)]
#[inline]
#[must_use]
pub fn json_derive(input: TokenStream) -> TokenStream {
    json_derive_impl(input)
}
