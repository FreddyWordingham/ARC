//! Shader input settings implementation.

pub mod lighting_weights;
pub mod shadow_weights;

use self::{lighting_weights::*, shadow_weights::*};

use crate::access;
use attr::json;

/// Shader settings.
#[json]
pub struct Shader {
    /// Lighting weightings.
    light_weights: LightingWeights,
    /// Shadow weightings.
    shadow_weights: ShadowWeights,
    /// Depth of field radius.
    dof_radius: f64,
}

impl Shader {
    access!(light_weights, LightingWeights);
    access!(shadow_weights, ShadowWeights);
    access!(dof_radius, f64);
}
