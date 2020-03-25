//! Probability distribution implementation.

use crate::math::rng::Probability as RngProb;
use attr::json;

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
    pub fn build(&self) -> RngProb {
        match self {
            Self::Point { c } => RngProb::new_point(*c),
        }
    }
}
