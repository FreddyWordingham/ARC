//! Settings implementation.

use crate::access;
use attr::json;

/// Runtime settings structure.
#[json]
pub struct Settings {
    /// Example.
    example: i32,
}

impl Settings {
    access!(example, i32);
}
