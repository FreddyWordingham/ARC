//! Image implementation.

pub mod camera;
pub mod palette;

pub use self::{camera::*, palette::*};

use crate::{
    access,
    rend::{
        settings::{Quality, Shader},
        AspectRatio, Grid, Scene,
    },
};
use ::palette::LinSrgba;
use ndarray::Array2;

/// Image structure.
pub struct Image {
    /// Target aspect ratio.
    aspect_ratio: AspectRatio,
    /// Quality settings.
    quality: Quality,
    /// Shader settings.
    shader: Shader,
    /// Palette colours.
    palette: Palette,
    /// Camera.
    camera: Camera,
}

impl Image {
    access!(aspect_ratio, AspectRatio);
    access!(quality, Quality);
    access!(shader, Shader);
    access!(palette, Palette);
    access!(camera, Camera);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        aspect_ratio: AspectRatio,
        quality: Quality,
        shader: Shader,
        palette: Palette,
        camera: Camera,
    ) -> Self {
        Self {
            aspect_ratio,
            quality,
            shader,
            palette,
            camera,
        }
    }

    /// Create a rendering of a given scene.
    #[inline]
    #[must_use]
    pub fn render(&self, _scene: &Scene, _grid: &Grid) -> Array2<LinSrgba> {
        Array2::default(self.camera.res())
    }
}
