//! Frame input settings implementation.

use crate::{access, clone, img::AspectRatio};
use attr::json;
use nalgebra::Point3;

/// Frame settings.
#[json]
pub struct Frame {
    /// Quality file.
    quality: String,
    /// Shader file.
    shader: String,
    /// Palette file.
    palette: String,
    /// Aspect ratio.
    aspect_ratio: AspectRatio,
    /// Horizontal field of view.
    fov: f64,
    /// Position of the camera.
    cam_pos: Point3<f64>,
    /// Target of the camera.
    tar_pos: Point3<f64>,
}

impl Frame {
    access!(quality, String);
    access!(shader, String);
    access!(palette, String);
    clone!(aspect_ratio, AspectRatio);
    clone!(fov, f64);
    access!(cam_pos, Point3<f64>);
    access!(tar_pos, Point3<f64>);
}
