//! Rendering settings sub-module.

pub mod grid;
pub mod image;
pub mod palette;
pub mod quality;
pub mod scene;
pub mod shader;

pub use self::{grid::*, image::*, palette::*, quality::*, scene::*, shader::*};

use crate::access;
use attr::json_load;
use std::collections::BTreeMap;

/// Rendering settings.
#[json_load]
pub struct Settings {
    /// Grit settings.
    grid: Grid,
    /// Scene settings.
    scene: String,
    /// Images.
    images: BTreeMap<String, Image>,
}

impl Settings {
    access!(grid, Grid);
    access!(scene, String);
    access!(images, BTreeMap<String, Image>);
}
