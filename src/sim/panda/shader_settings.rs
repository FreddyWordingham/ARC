//! Shader settings implementation.

use crate::access;
use attr::json;
use nalgebra::Point3;
use std::fmt::{Display, Formatter, Result};

/// Runtime shader settings.
#[json]
pub struct ShaderSettings {
    /// Position of the sun.
    sun_pos: Point3<f64>,
}

impl ShaderSettings {
    access!(sun_pos, Point3<f64>);
}

impl Display for ShaderSettings {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt, "")?;
        writeln!(fmt, "{:>30} : {}", "sun position", self.sun_pos)
    }
}
