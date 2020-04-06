//! Rendering settings sub-module.

pub mod grid;
pub mod image;
pub mod palette;
pub mod quality;
pub mod scene;
pub mod shader;

pub use self::{grid::*, image::*, palette::*, quality::*, scene::*, shader::*};

use crate::{
    access,
    file::Load,
    rend::{Grid as RenderGrid, Image as RenderImage, Scene as RenderScene},
};
use attr::json_load;
use std::{collections::BTreeMap, path::Path};

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

    /// Load the scene.
    #[inline]
    #[must_use]
    pub fn load_scene(&self, in_dir: &Path) -> RenderScene {
        Scene::load(&in_dir.join(&self.scene)).build(in_dir)
    }

    /// Build the grid.
    #[inline]
    #[must_use]
    pub fn build_grid(&self, scene: &RenderScene) -> RenderGrid {
        RenderGrid::new_root()
    }

    /// Build the images.
    #[inline]
    #[must_use]
    pub fn build_images(&self) -> Vec<RenderImage> {
        vec![]
    }

    /// Load the described rendering components.
    #[inline]
    #[must_use]
    pub fn build(&self, in_dir: &Path) -> (RenderScene, RenderGrid, Vec<RenderImage>) {
        let scene = self.load_scene(in_dir);
        let grid = self.build_grid(&scene);
        let images = self.build_images();

        (scene, grid, images)
    }
}
