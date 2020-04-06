//! Palette colours input settings implementation.

use crate::{
    access,
    rend::{image::Palette as ImagePalette, Group},
};
use attr::json;

/// Palette settings.
#[json]
pub struct Palette {
    /// Group colours.
    cols: Vec<(Group, Vec<[f64; 4]>)>,
}

impl Palette {
    access!(cols, Vec<(Group, Vec<[f64; 4]>)>);

    /// Build a complete instance.
    #[inline]
    #[must_use]
    pub fn build(&self) -> ImagePalette {
        ImagePalette::new()
    }
}
