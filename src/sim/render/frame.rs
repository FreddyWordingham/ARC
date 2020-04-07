//! Frame implementation.

use crate::{
    access,
    img::{AspectRatio, Quality, Shader},
    sim::render::{Camera, Group},
};
use ndarray::Array2;
use palette::{Gradient, LinSrgba};
use std::collections::HashMap;

/// Frame structure.
pub struct Frame {
    /// Target aspect ratio.
    aspect_ratio: AspectRatio,
    /// Quality settings.
    quality: Quality,
    /// Shader settings.
    shader: Shader,
    /// Palette colours.
    palette: HashMap<Group, Gradient<LinSrgba>>,
    /// Camera.
    camera: Camera,
}

impl Frame {
    access!(aspect_ratio, AspectRatio);
    access!(quality, Quality);
    access!(shader, Shader);
    access!(palette, HashMap<Group, Gradient<LinSrgba>>);
    access!(camera, Camera);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        aspect_ratio: AspectRatio,
        quality: Quality,
        shader: Shader,
        palette: HashMap<Group, Gradient<LinSrgba>>,
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
