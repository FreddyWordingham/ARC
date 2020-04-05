//! Settings implementation.

use crate::clone;
use attr::json;
use nalgebra::{Unit, Vector3};

/// Runtime settings structure.
#[json]
pub struct Settings {
    /// Number of threads to use.
    num_threads: usize,
    /// Refractive index of water.
    water_ref_index: f64,
    /// Light direction.
    light_dir: Vector3<f64>,
}

impl Settings {
    clone!(num_threads, usize);
    clone!(water_ref_index, f64);

    /// Get the normalised light direction.
    #[inline]
    #[must_use]
    pub fn light_dir(&self) -> Unit<Vector3<f64>> {
        Unit::new_normalize(self.light_dir)
    }
}
