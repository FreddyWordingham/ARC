//! Shader input settings implementation.

pub mod lighting_weights;
pub mod shadow_weights;

use self::lighting_weights::*;
use self::shadow_weights::*;

use crate::access;
use attr::json;
// use std::fmt::{Display, Formatter, Result};

/// Shader settings.
#[json]
pub struct Shader {
    /// Lighting weightings.
    light_weights: LightingWeights,
    /// Shadow weightings.
    shadow_weights: ShadowWeights,
}

impl Shader {
    access!(light_weights, LightingWeights);
    access!(shadow_weights, ShadowWeights);
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
