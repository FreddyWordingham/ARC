//! Settings implementation.

use crate::{access, clone};
use attr::json;
use nalgebra::Point3;

/// Runtime settings.
#[json]
pub struct Settings {
    /// Overhead sun position.
    sun_pos: Point3<f64>,
    /// Shadowing factor. Zero - no shadow, one - complete shadow.
    shadow: f64,
    /// Transparency factor. Zero - completely transparent, one - completely opaque.
    transparency: f64,
    /// Ambient lighting scaling factor.
    ambient: f64,
    /// Diffuse lighting scaling factor.
    diffuse: f64,
    /// Specular lighting scaling factor.
    specular: f64,
    /// Specular lighting power.
    specular_pow: i32,
}

impl Settings {
    access!(sun_pos, Point3<f64>);
    clone!(shadow, f64);
    clone!(transparency, f64);
    clone!(ambient, f64);
    clone!(diffuse, f64);
    clone!(specular, f64);
    clone!(specular_pow, i32);
}
