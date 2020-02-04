//! Optics implementation.

use crate::{access, math::Formula, phys::Environment};
use attr::json;

/// Optical properties structure.
#[json]
pub struct Optics {
    /// Refractive index.
    ref_index: Formula,
    /// Scattering coefficient. [m^-1]
    scat_coeff: Formula,
    /// Absorption coefficient. [m^-1]
    abs_coeff: Formula,
    /// Shift coefficient. [m^-1]
    shift_coeff: Formula,
    /// Asymmetry parameter.
    asym: Formula,
}

impl Optics {
    access!(ref_index, Formula);
    access!(scat_coeff, Formula);
    access!(abs_coeff, Formula);
    access!(shift_coeff, Formula);
    access!(asym, Formula);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        ref_index: Formula,
        scat_coeff: Formula,
        abs_coeff: Formula,
        shift_coeff: Formula,
        asym: Formula,
    ) -> Self {
        Self {
            ref_index,
            scat_coeff,
            abs_coeff,
            shift_coeff,
            asym,
        }
    }

    /// Determine the local optical properties for a given wavelength.
    #[inline]
    #[must_use]
    pub fn env(&self, w: f64) -> Environment {
        Environment::new(
            self.ref_index.y(w),
            self.scat_coeff.y(w),
            self.abs_coeff.y(w),
            self.shift_coeff.y(w),
            self.asym.y(w),
        )
    }
}
