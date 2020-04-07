//! Rendering settings sub-module.

pub mod frame;
pub mod grid;
pub mod quality;
pub mod scene;
pub mod scheme;
pub mod shader;

pub use self::{frame::*, grid::*, quality::*, scene::*, scheme::*, shader::*};

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
    /// Frames.
    frames: BTreeMap<String, Frame>,
}

impl Settings {
    access!(grid, Grid);
    access!(scene, String);
    access!(frames, BTreeMap<String, Frame>);
}
