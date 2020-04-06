//! Rendering settings sub-module.

pub mod grid;
pub mod image;
pub mod palette;
pub mod quality;
pub mod scene;
pub mod shader;

pub use self::{grid::*, image::*, palette::*, quality::*, scene::*, shader::*};

use crate::{access, file::Load};
use attr::json_load;
use std::collections::BTreeMap;

// #[derive(Debug)]
// pub struct Fi<T> {
//     data: T,
// }

// impl<T> Fi<T> {
//     pub fn new(data: T) -> Self {
//         Self { data }
//     }
// }

// impl<'de, T: Load> serde::Deserialize<'de> for Fi<T> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         Ok(Fi::new(T::load(std::path::Path::new("hello.txt"))))
//     }
// }

/// Rendering settings.
#[json_load]
pub struct Settings {
    // /// Grit settings.
// grid: Grid,
// // /// Scene settings.
// // scene: Fi<Scene>,
// /// Images.
// images: BTreeMap<String, Image>,
}

impl Settings {
    // access!(grid, Grid);
    // // access!(scene, Scene);
    // access!(images, BTreeMap<String, Image>);
}
