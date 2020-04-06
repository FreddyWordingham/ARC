//! Palette colours input settings implementation.

use crate::{access, rend::Group};
use attr::json;

/// Palette settings.
#[json]
pub struct Palette {
    /// Group colours.
    cols: Vec<(Group, Vec<[f64; 4]>)>,
}

impl Palette {
    access!(cols, Vec<(Group, Vec<[f64; 4]>)>);
}
