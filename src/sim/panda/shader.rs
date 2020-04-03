//! Shader implementation.

use crate::access;
use attr::json;
use nalgebra::Point3;

/// Runtime shader settings.
#[json]
pub struct Shader {
    /// Position of the sun.
    sun_pos: Point3<f64>,
}

impl Shader {
    access!(sun_pos, Point3<f64>);
}
