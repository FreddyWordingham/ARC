//! Probability distribution implementation.

use attr::json;
use rand::rngs::ThreadRng;
// use ndarray::Array1;

/// Probability distribution formulae.
#[json]
pub enum Probability {
    /// Point.
    Point {
        /// Constant value.
        c: f64,
    },
}

impl Probability {
    /// Generate a random number from the described distribution.
    #[inline]
    #[must_use]
    pub fn gen(&self, _rng: &mut ThreadRng) -> f64 {
        match self {
            Self::Point { c } => *c,
        }
    }
}
