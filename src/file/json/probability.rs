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
    /// Construct a new point instance.
    #[inline]
    #[must_use]
    pub fn new_point(c: f64) -> Self {
        Self::Point { c }
    }
}
