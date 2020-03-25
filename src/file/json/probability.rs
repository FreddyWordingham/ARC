//! Probability distribution implementation.

use attr::json;
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
    /// Build a random number generator probability distribution.
    #[inline]
    #[must_use]
    pub fn build(&self) -> crate::math::rng::Probability {
        match self {
            Self::Point { c } => crate::math::rng::Probability::new_point(*c),
        }
    }
}
