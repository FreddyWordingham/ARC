//! Reaction implementation.

use crate::{access, chem::Rate, ord::SpecKey};
use attr::json;

/// Species reaction structure.
#[json]
pub struct Reaction {
    /// List of reactant species buy id, and their stoichiometric coefficient.
    reactants: Vec<(SpecKey, i32)>,
    /// List of product species buy id, and their stoichiometric coefficient.
    products: Vec<(SpecKey, i32)>,
    /// Rate.
    rate: Rate,
}

impl Reaction {
    access!(reactants, Vec<(SpecKey, i32)>);
    access!(products, Vec<(SpecKey, i32)>);
    access!(rate, Rate);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(reactants: Vec<(SpecKey, i32)>, products: Vec<(SpecKey, i32)>, rate: Rate) -> Self {
        Self {
            reactants,
            products,
            rate,
        }
    }
}
