//! Image implementation.

pub mod camera;
pub mod palette;

pub use self::camera::*;
pub use self::palette::*;

use crate::{
    access,
    rend::settings::{Quality, Shader},
};

/// Image structure.
pub struct Image {
    /// Quality settings.
    quality: Quality,
    /// Shader settings.
    shader: Shader,
    /// Palette colours.
    palette: Palette,
    /// Camera.
    cam: Camera,
}

impl Image {
    access!(quality, Quality);
    access!(shader, Shader);
    access!(palette, Palette);
    access!(cam, Camera);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(quality: Quality, shader: Shader, palette: Palette, cam: Camera) -> Self {
        Self {
            quality,
            shader,
            palette,
            cam,
        }
    }
}
