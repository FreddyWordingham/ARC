//! Interface implementation.

use crate::{
    access,
    ord::{Set, SpecKey, SpecSet},
};
use attr::json;
use ndarray::Array1;

/// State structure.
#[json]
pub struct State {
    /// Concentrations.
    concs: Set<SpecKey, f64>,
    /// Sources.
    sources: Set<SpecKey, f64>,
}

impl State {
    access!(concs, Set<SpecKey, f64>);
    access!(sources, Set<SpecKey, f64>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(concs: Set<SpecKey, f64>, sources: Set<SpecKey, f64>) -> Self {
        Self { concs, sources }
    }

    /// Create a new complete concentration array.
    #[inline]
    #[must_use]
    pub fn new_conc_arr(&self, specs: &SpecSet) -> Array1<f64> {
        let mut concs = Array1::zeros(specs.map().len());

        for (key, conc) in self.concs.map() {
            *concs.get_mut(specs.index_of_key(key)).unwrap() = *conc;
        }

        concs
    }

    /// Create a new complete source array.
    #[inline]
    #[must_use]
    pub fn new_source_arr(&self, specs: &SpecSet) -> Array1<f64> {
        let mut sources = Array1::zeros(specs.map().len());

        for (key, source) in self.sources.map() {
            *sources.get_mut(specs.index_of_key(key)).unwrap() = *source;
        }

        sources
    }
}
