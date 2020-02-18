//! Reactor structure implementation.

use crate::{
    access,
    math::Multivariate,
    ord::{ReactSet, SpecSet},
};
use ndarray::{Array1, Array2};

/// Reaction driving structure.
#[derive(Debug)]
pub struct Reactor {
    /// Rate formulae.
    rates: Array1<Multivariate>,
    /// Coefficents.
    cs: Array2<f64>,
}

impl Reactor {
    access!(rates, Array1<Multivariate>);
    access!(cs, Array2<f64>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(reacts: &ReactSet, specs: &SpecSet) -> Self {
        let rates = reacts
            .map()
            .values()
            .map(|r| r.rate().create_lambda(specs))
            .collect();

        let mut cs = Array2::zeros((reacts.map().len(), specs.map().len()));

        for (i, react) in reacts.map().values().enumerate() {
            for (r, c) in react.reactants() {
                *cs.get_mut((i, specs.index_of_key(r)))
                    .expect("Invalid index.") -= f64::from(*c);
            }

            for (p, c) in react.products() {
                *cs.get_mut((i, specs.index_of_key(p)))
                    .expect("Invalid index.") += f64::from(*c);
            }
        }

        Self { rates, cs }
    }

    /// Determine the current reaction rates.
    #[inline]
    #[must_use]
    pub fn calc_rates(&self, concs: &Array1<f64>) -> Array1<f64> {
        let rs = self.rates.map(|lambda| lambda.y(concs));
        let mut rates = Array1::zeros(concs.len());

        for (i, r) in rs.iter().enumerate() {
            for (j, _c) in concs.iter().enumerate() {
                *rates.get_mut(j).unwrap() += r * self.cs.get((i, j)).unwrap();
            }
        }

        rates
    }
}
