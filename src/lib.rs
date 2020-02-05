//! Library core.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
#![allow(
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
    clippy::option_expect_used,
    clippy::panic,
    clippy::print_stdout,
    clippy::result_expect_used,
    clippy::unreachable,
    clippy::wildcard_enum_match_arm
)]

pub mod chem;
pub mod data;
pub mod file;
pub mod geom;
pub mod list;
pub mod math;
pub mod ord;
pub mod phys;
// pub mod sim;
pub mod util;
pub mod world;
