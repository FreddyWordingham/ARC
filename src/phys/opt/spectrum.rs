//! Spectrum implementation.

use attr::json;
use rand::rngs::ThreadRng;

/// Spectrum enumeration implementation.
#[json]
pub enum Spectrum {
    /// Single wavelength.
    Laser(f64),
}

impl Spectrum {
    /// Construct a new laser spectrum.
    #[inline]
    #[must_use]
    pub fn new_laser(wavelength: f64) -> Self {
        assert!(wavelength > 0.0);

        Self::Laser { 0: wavelength }
    }

    /// Sample the spectrum for a wavelength.
    #[inline]
    #[must_use]
    pub fn sample(&self, _rng: &mut ThreadRng) -> f64 {
        match self {
            Self::Laser(w) => *w,
        }
    }
}
