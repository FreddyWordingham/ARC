//! Interface implementation.

use crate::{
    access,
    ord::{Set, SpecKey},
};
use attr::json;

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
}
