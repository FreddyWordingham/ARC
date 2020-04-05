//! Probability distribution implementation.

use crate::math::rng::Probability as RngProb;
use attr::json;
use ndarray::Array1;

/// Probability distribution formulae.
#[json]
pub enum Probability {
    /// Point.
    Point {
        /// Constant value.
        c: f64,
    },
    /// Points.
    Points {
        /// Possible values.
        cs: Vec<f64>,
    },
    // /// Weighted points.
    // WeightedPoints {
    //     /// Possible values.
    //     cs: Vec<f64>,
    //     /// Relative weightings values.
    //     ws: Vec<f64>,
    // },
    /// Uniform range.
    Uniform {
        /// Minimum value.
        min: f64,
        /// Maximum value.
        max: f64,
    },
    /// Gaussian distribution.
    Gaussian {
        /// Average value.
        ave: f64,
        /// Variance.
        var: f64,
    },
    /// Linear distribution.
    Linear {
        /// Gradient.
        m: f64,
        /// Constant.
        c: f64,
    },
}

impl Probability {
    /// Build a random number generator probability distribution.
    #[inline]
    #[must_use]
    pub fn build(self) -> RngProb {
        match self {
            Self::Point { c } => RngProb::new_point(c),
            Self::Points { cs } => RngProb::new_points(Array1::from(cs)),
            // Self::WeightedPoints { cs, ws } => {
            // RngProb::new_weighted_points(Array1::from(cs), Array1::from(ws))
            // }
            Self::Uniform { min, max } => RngProb::new_uniform(min, max),
            Self::Gaussian { ave, var } => RngProb::new_gaussian(ave, var),
            Self::Linear { m, c } => RngProb::new_linear(m, c),
        }
    }
}
