//! Progress-Bar sub-module.

pub mod par_progress_bar;
pub mod progress_bar;

pub use self::{par_progress_bar::*, progress_bar::*};
