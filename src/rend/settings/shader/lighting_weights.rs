//! Shader lighting weightings settings implementation.

use crate::clone;
use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Lighting weights settings.
#[json]
pub struct LightingWeights {
    /// Ambient lighting scalar.
    ambient: f64,
    /// Diffuse lighting scalar.
    diffuse: f64,
    /// Specular lighting scalar.
    specular: f64,
}

impl LightingWeights {
    clone!(ambient, f64);
    clone!(diffuse, f64);
    clone!(specular, f64);
}

impl Display for LightingWeights {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt, "{:>30} : {}", "ambient scalar", self.ambient)?;
        writeln!(fmt, "{:>30} : {}", "diffuse scalar", self.diffuse)?;
        writeln!(fmt, "{:>30} : {}", "specular scalar", self.specular)
    }
}
