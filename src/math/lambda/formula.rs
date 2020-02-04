//! Formula implementation.

use attr::json;

/// Mathematical formulae accepting a single scalar argument.
#[json]
pub enum Formula {
    /// No-op. = x
    X,
    /// Constant value. = c
    Constant {
        /// Constant.
        c: f64,
    },
    /// Linear formula. = (x * m) + c
    Linear {
        /// Offset coefficient.
        c: f64,
        /// Gradient.
        m: f64,
    },
    /// Polynomial formula. = sum((x^i) * c[i])
    Polynomial {
        /// Constants starting with the zeroth order coefficient.
        cs: Vec<f64>,
    },
    /// Recovery formula. = (c - x) * r
    Recovery {
        /// Target value.
        c: f64,
        /// Recovery rate.
        r: f64,
    },
}

impl Formula {
    /// Determine the corresponding output value for the given input.
    #[inline]
    #[must_use]
    pub fn y(&self, x: f64) -> f64 {
        match self {
            Self::X {} => x,
            Self::Constant { c } => *c,
            Self::Linear { c, m } => (x * m) + c,
            Self::Polynomial { cs } => {
                let mut sum = 0.0;
                for (i, c) in cs.iter().enumerate() {
                    sum += c * x.powi(i as i32);
                }
                sum
            }
            Self::Recovery { c, r } => (c - x) * r,
        }
    }
}
