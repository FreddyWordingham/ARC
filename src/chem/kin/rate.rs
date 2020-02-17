//! Rate implementation.

use crate::{
    math::Multivariate,
    ord::{SpecKey, SpecSet},
};
use attr::json;
use ndarray::Array1;
use std::fmt::{Display, Formatter, Result};

/// Rates that accept a single scalar value, and return a single scalar value.
#[json]
pub enum Rate {
    /// Niladic function. f(cs) = k
    Zeroth(f64),
    /// Monadic. f(cs) = k[A]
    First(f64, SpecKey),
    /// Dyadic. f(cs) = k[A][B]
    Second(f64, SpecKey, SpecKey),
    /// Triadic. f(cs) = k[A][B][C]
    Third(f64, SpecKey, SpecKey, SpecKey),
    /// Polyadic. f(cs) = prod(k[n])
    Poly(f64, Vec<SpecKey>),
}

impl Rate {
    /// Create a multivariate lambda function to apply the reaction to a concentration array.
    pub fn create_lambda(&self, specs: SpecSet) -> Multivariate {
        match self {
            Self::Zeroth(k) => Multivariate::new_constant(*k),
            Self::First(k, a) => Multivariate::new_scaled_first_order(*k, specs.index_of_key(a)),
            Self::Second(k, a, b) => Multivariate::new_scaled_second_order(
                *k,
                specs.index_of_key(a),
                specs.index_of_key(b),
            ),
            Self::Third(k, a, b, c) => Multivariate::new_scaled_third_order(
                *k,
                specs.index_of_key(a),
                specs.index_of_key(b),
                specs.index_of_key(c),
            ),
            Self::Poly(k, cs) => {
                let mut ws = Array1::zeros(specs.map().len());
                for c in cs {
                    *ws.get_mut(specs.index_of_key(c)).expect("Invalid index.") = 1.0;
                }
                Multivariate::new_scaled_weight(*k, ws)
            }
        }
    }
}

impl Display for Rate {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            Self::Zeroth(k) => write!(fmt, "{}", k),
            Self::First(k, a) => write!(fmt, "{} [{}]", k, a),
            Self::Second(k, a, b) => write!(fmt, "{} [{}] [{}]", k, a, b),
            Self::Third(k, a, b, c) => write!(fmt, "{} [{}] [{}] [{}]", k, a, b, c),
            Self::Poly(k, cs) => {
                write!(fmt, "{}", k)?;
                for c in cs {
                    write!(fmt, " [{}]", c)?;
                }
                write!(fmt, "")
            }
        }
    }
}
