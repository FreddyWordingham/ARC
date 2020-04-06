//! Image implementation.

pub mod camera;
pub mod palette;

pub use self::camera::*;
pub use self::palette::*;

use crate::{
    access,
    rend::{
        settings::{Quality, Shader},
        AspectRatio,
    },
};

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
}
