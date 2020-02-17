//! Multivariate implementation.

use attr::json;
use ndarray::Array1;

/// Mathematical multivariate formulae accepting an array of scalar arguments.
#[json]
pub enum Multivariate {
    /// Sum
    Sum,
}

impl Multivariate {
    /// Construct a new sum instance.
    #[inline]
    #[must_use]
    pub fn new_sum() -> Self {
        Self::Sum {}
    }

    /// Determine the corresponding output value for the given input.
    #[inline]
    #[must_use]
    pub fn y(&self, xs: &Array1<f64>) -> f64 {
        match self {
            Self::Sum {} => xs.sum(),
        }
    }
}
