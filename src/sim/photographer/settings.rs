//! Settings implementation.

use crate::clone;
use attr::json;

/// Runtime settings structure.
#[json]
pub struct Settings {
    /// Number of threads to use.
    num_threads: usize,
    /// Refractive index of water.
    water_ref_index: f64,
}

impl Settings {
    clone!(num_threads, usize);
    clone!(water_ref_index, f64);
}
