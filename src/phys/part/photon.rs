//! Photon implementation.

use crate::{access, clone, geom::Ray};

/// Photon structure.
#[derive(Debug, Clone)]
pub struct Photon {
    /// Statistical weight.
    weight: f64,
    /// Wavelength [m].
    wavelength: f64,
    /// Power [J/s].
    power: f64,
    /// Ray of travel.
    ray: Ray,
}

impl Photon {
    clone!(weight, weight_mut, f64);
    clone!(wavelength, f64);
    clone!(power, f64);
    access!(ray, ray_mut, Ray);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(wavelength: f64, power: f64, ray: Ray) -> Self {
        debug_assert!(wavelength > 0.0);
        debug_assert!(power > 0.0);

        Self {
            weight: 1.0,
            wavelength,
            power,
            ray,
        }
    }
}
