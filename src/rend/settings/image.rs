//! Image input settings implementation.

use crate::{access, rend::AspectRatio};
use attr::json;
// use std::fmt::{Display, Formatter, Result};
use nalgebra::Point3;

/// Image settings.
#[json]
pub struct Image {
    /// Position of the camera.
    cam_pos: Point3<f64>,
    /// Target of the camera.
    tar_pos: Point3<f64>,
    /// Horizontal field of view.
    fov: f64,
    /// Aspect ratio.
    aspect_ratio: AspectRatio,
}

impl Image {
    access!(cam_pos, Point3<f64>);
    access!(tar_pos, Point3<f64>);
}

// impl Display for Image {
//     fn fmt(&self, fmt: &mut Formatter) -> Result {
//         writeln!(fmt)?;
//         writeln!(fmt, "{:>30} : {}", "target triangles", self.tar_tris)?;
//         writeln!(fmt, "{:>30} : {}", "max depth", self.max_depth)?;
//         writeln!(
//             fmt,
//             "{:>30} : {}%",
//             "collision detection padding",
//             self.padding * 100.0
//         )
//     }
// }
