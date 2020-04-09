//! Shader lighting weightings settings implementation.

use crate::clone;
use attr::json;

/// Lighting weights settings.
#[json]
pub struct LightingWeights {
    /// Ambient lighting scalar.
    ambient: f64,
    /// Diffuse lighting scalar.
    diffuse: f64,
    /// Specular lighting scalar.
    specular: f64,
    /// Specular power factor.
    specular_power: i32,
}

impl LightingWeights {
    clone!(ambient, f64);
    clone!(diffuse, f64);
    clone!(specular, f64);
    clone!(specular_power, i32);
}
