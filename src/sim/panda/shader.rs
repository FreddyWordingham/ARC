//! Shader implementation.

use crate::access;
use attr::json;
use nalgebra::Point3;
use std::fmt::{Display, Formatter, Result};

/// Runtime shader settings.
#[json]
pub struct Shader {
    /// Position of the sun.
    sun_pos: Point3<f64>,
}

impl Shader {
    access!(sun_pos, Point3<f64>);
}

impl Display for Shader {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt, "...")?;
        write!(fmt, "\tsun position: {}", self.sun_pos)
    }
}
