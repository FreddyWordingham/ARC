//! Shader shadow weightings settings implementation.

use crate::clone;
use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Shadowing weights settings.
#[json]
pub struct ShadowWeights {
    /// Direct shadowing scalar.
    direct: f64,
    /// Local shadowing scalar.
    local: f64,
    /// Ambient shadowing scalar.
    ambient: f64,
}

impl ShadowWeights {
    clone!(direct, f64);
    clone!(local, f64);
    clone!(ambient, f64);
}

impl Display for ShadowWeights {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt)?;
        writeln!(fmt, "{:>30} : {}", "direct scalar", self.direct)?;
        writeln!(fmt, "{:>30} : {}", "local scalar", self.local)?;
        writeln!(fmt, "{:>30} : {}", "ambient scalar", self.ambient)
    }
}
