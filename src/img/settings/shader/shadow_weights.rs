//! Shader shadow weightings settings implementation.

use crate::clone;
use attr::json;

/// Shadowing weights settings.
#[json]
pub struct ShadowWeights {
    /// Direct shadowing scalar.
    direct: f64,
    /// Local shadowing scalar.
    local: f64,
    /// Ambient shadowing scalar.
    ambient: f64,
    /// Transparency multiplier.
    transparency: f64,
}

impl ShadowWeights {
    clone!(direct, f64);
    clone!(local, f64);
    clone!(ambient, f64);
    clone!(transparency, f64);
}
