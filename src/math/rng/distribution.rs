//! Distribution functions.

use rand::{rngs::ThreadRng, Rng};

/// Sample the Henyey-Greenstein phase function with a given asymmetry parameter.
#[inline]
#[must_use]
pub fn henyey_greenstein(rng: &mut ThreadRng, asym: f64) -> f64 {
    debug_assert!(asym.abs() <= 1.0);

    if asym.abs() < 1.0e-6 {
        return rng.gen_range(-1.0_f64, 1.0).acos();
    }

    ((1.0 + asym.powi(2)
        - ((1.0 - asym.powi(2)) / asym.mul_add(rng.gen_range(-1.0, 1.0), 1.0)).powi(2))
        / (2.0 * asym))
        .acos()
}
