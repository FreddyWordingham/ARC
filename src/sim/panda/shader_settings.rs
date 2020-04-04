//! Shader settings implementation.

use crate::{access, clone};
use attr::json;
use nalgebra::Point3;
use std::fmt::{Display, Formatter, Result};

/// Runtime shader settings.
#[json]
pub struct ShaderSettings {
    /// Position of the sun.
    sun_pos: Point3<f64>,
    /// Ambient lighting scaling factor.
    ambient: f64,
    /// Diffuse lighting scaling factor.
    diffuse: f64,
    /// Specular lighting scaling factor.
    specular: f64,
    /// Specular power factor.
    specular_pow: i32,
    /// Transparency.
    transparency: f64,
    /// Shadow.
    shadow: f64,
    /// Sunlight shadow samples.
    sunlight_samples: i32,
    /// Sunlight shadow sampling radius.
    sunlight_radius: f64,
}

impl ShaderSettings {
    access!(sun_pos, Point3<f64>);
    clone!(ambient, f64);
    clone!(diffuse, f64);
    clone!(specular, f64);
    clone!(specular_pow, i32);
    clone!(transparency, f64);
    clone!(shadow, f64);
    clone!(sunlight_samples, i32);
    clone!(sunlight_radius, f64);
}

impl Display for ShaderSettings {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt)?;
        writeln!(fmt, "{:>30} : {}", "sun position", self.sun_pos)
    }
}
