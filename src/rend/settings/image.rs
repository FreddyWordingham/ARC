//! Image input settings implementation.

use crate::{
    access, clone,
    rend::{settings::Quality, AspectRatio},
};
use attr::json;
use nalgebra::Point3;
use std::fmt::{Display, Formatter, Result};

/// Image settings.
#[json]
pub struct Image {
    /// Quality level.
    quality: Quality,
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
    clone!(quality, Quality);
    access!(cam_pos, Point3<f64>);
    access!(tar_pos, Point3<f64>);
    clone!(fov, f64);
    clone!(aspect_ratio, AspectRatio);
}

impl Display for Image {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt)?;
        writeln!(fmt, "{:>30} : {}", "quality level", self.quality)?;
        writeln!(fmt, "{:>30} : {} [m]", "camera position", self.cam_pos)?;
        writeln!(fmt, "{:>30} : {} [m]", "max depth", self.tar_pos)?;
        writeln!(fmt, "{:>30} : {} [deg]", "field of view", self.fov)?;
        writeln!(fmt, "{:>30} : {}", "aspect ratio", self.aspect_ratio)
    }
}
