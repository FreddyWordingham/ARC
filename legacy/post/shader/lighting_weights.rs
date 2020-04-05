//! Lighting weights settings implementation.

use crate::clone;
use attr::json;
// use std::fmt::{Display, Formatter, Result};

/// Lighting weights settings.
#[json]
pub struct LightingWeights {
    ambient: f64,
    diffuse: f64,
    specular: f64,
}

impl LightingWeights {
    clone!(ambient, f64);
    clone!(diffuse, f64);
    clone!(specular, f64);
}

// impl Display for Shader {
//     fn fmt(&self, fmt: &mut Formatter) -> Result {
//         writeln!(fmt)?;
//         writeln!(fmt, "{:>30} : {}", "target triangles", self.tar_tris)?;
//         writeln!(fmt, "{:>30} : {}", "max depth", self.max_depth)?;
//         writeln!(
//             fmt,
//             "{:>30} : {}%",
//             "collision detection padding",
//             self.padding * 100.0
//         )
//     }
// }
