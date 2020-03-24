//! Multivariate implementation.

use attr::json;
use ndarray::Array1;

/// Mathematical multivariate formulae accepting an array of scalar arguments.
#[json]
pub enum Multivariate {
    /// Sum.
    Sum,
    /// Constant.
    Constant {
        /// Constant value.
        c: f64,
    },
    /// First order scaled function.
    ScaledFirstOrder {
        /// Scaling constant.
        k: f64,
        /// First order index.
        a: usize,
    },
    /// Second order scaled function.
    ScaledSecondOrder {
        /// Scaling constant.
        k: f64,
        /// First order index.
        a: usize,
        /// Second order index.
        b: usize,
    },
    /// Third order scaled function.
    ScaledThirdOrder {
        /// Scaling constant.
        k: f64,
        /// First order index.
        a: usize,
        /// Second order index.
        b: usize,
        /// Third order index.
        c: usize,
    },
    /// Weighting
    Weight {
        /// Weightings for each respective input value.
        ws: Array1<f64>,
    },
    /// Scaled weighting
    ScaledWeight {
        /// Scaling constant.
        k: f64,
        /// Weightings for each respective input value.
        ws: Array1<f64>,
    },
}

impl Multivariate {
    /// Construct a new sum instance.
    #[inline]
    #[must_use]
    pub fn new_sum() -> Self {
        Self::Sum {}
    }

    /// Construct a new constant instance.
    #[inline]
    #[must_use]
    pub fn new_constant(c: f64) -> Self {
        Self::Constant { c }
    }

    /// Construct a new scaled first order instance.
    #[inline]
    #[must_use]
    pub fn new_scaled_first_order(k: f64, a: usize) -> Self {
        Self::ScaledFirstOrder { k, a }
    }

    /// Construct a new scaled second order instance.
    #[inline]
    #[must_use]
    pub fn new_scaled_second_order(k: f64, a: usize, b: usize) -> Self {
        Self::ScaledSecondOrder { k, a, b }
    }

    /// Construct a new scaled third order instance.
    #[inline]
    #[must_use]
    pub fn new_scaled_third_order(k: f64, a: usize, b: usize, c: usize) -> Self {
        Self::ScaledThirdOrder { k, a, b, c }
    }

    /// Construct a new weighted instance.
    #[inline]
    #[must_use]
    pub fn new_weight(ws: Array1<f64>) -> Self {
        Self::Weight { ws }
    }

    /// Construct a new scaled weighted instance.
    #[inline]
    #[must_use]
    pub fn new_scaled_weight(k: f64, ws: Array1<f64>) -> Self {
        Self::ScaledWeight { k, ws }
    }

    /// Determine the corresponding output value for the given input.
    #[inline]
    #[must_use]
    pub fn y(&self, xs: &Array1<f64>) -> f64 {
        match self {
            Self::Sum {} => xs.sum(),
            Self::Constant { c } => *c,
            Self::ScaledFirstOrder { k, a } => *xs.get(*a).expect("Invalid index.") * k,
            Self::ScaledSecondOrder { k, a, b } => {
                *xs.get(*a).expect("Invalid index.") * *xs.get(*b).expect("Invalid index.") * k
            }
            Self::ScaledThirdOrder { k, a, b, c } => {
                *xs.get(*a).expect("Invalid index.")
                    * *xs.get(*b).expect("Invalid index.")
                    * *xs.get(*c).expect("Invalid index.")
                    * k
            }
            Self::Weight { ws } => {
                assert!(xs.len() == ws.len());

                xs.iter().zip(ws).map(|(x, w)| x * w).sum()
            }
            Self::ScaledWeight { k, ws } => {
                assert!(xs.len() == ws.len());

                xs.iter().zip(ws).map(|(x, w)| x * w).sum::<f64>() * k
            }
        }
    }
}
