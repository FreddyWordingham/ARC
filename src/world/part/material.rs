//! Material implementation.

use crate::{access, phys::Optics};
use attr::json;

/// Material physical properties.
#[json]
pub struct Material {
    /// Optical properties.
    optics: Optics,
    /// Optional viscosity. [kg m s^-1]
    visc: Option<f64>,
    /// Optional reaction rate multiplier.
    reaction_multiplier: Option<f64>,
}

impl Material {
    access!(optics, Optics);
    access!(visc, Option<f64>);
    access!(reaction_multiplier, Option<f64>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(optics: Optics, visc: Option<f64>, reaction_multiplier: Option<f64>) -> Self {
        Self {
            visc,
            reaction_multiplier,
            optics,
        }
    }
}
