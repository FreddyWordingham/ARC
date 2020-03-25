//! Probability distribution implementation.

use attr::json;
use rand::{rngs::ThreadRng, Rng};

/// Probability distribution formulae.
#[json]
#[derive(Clone)]
pub enum Probability {
    /// Point.
    Point {
        /// Constant value.
        c: f64,
    },
    /// Uniform range.
    Uniform {
        /// Minimum value.
        min: f64,
        /// Maximum value.
        max: f64,
    },
}

impl Probability {
    /// Construct a new point instance.
    #[inline]
    #[must_use]
    pub fn new_point(c: f64) -> Self {
        Self::Point { c }
    }

    /// Construct a new uniform instance.
    #[inline]
    #[must_use]
    pub fn new_uniform(min: f64, max: f64) -> Self {
        debug_assert!(min < max);
        Self::Uniform { min, max }
    }

    /// Generate a random number from the described distribution.
    #[inline]
    #[must_use]
    pub fn gen(&self, rng: &mut ThreadRng) -> f64 {
        match self {
            Self::Point { c } => *c,
            Self::Uniform { min, max } => rng.gen_range(*min, *max),
        }
    }
}
