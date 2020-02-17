//! Reaction implementation.

use crate::{access, chem::Rate, ord::SpecKey};
use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Species reaction structure.
#[json]
pub struct Reaction {
    /// List of reactant species buy key, and their stoichiometric coefficient.
    reactants: Vec<(SpecKey, i32)>,
    /// List of product species buy key, and their stoichiometric coefficient.
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

impl Display for Reaction {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        if let Some((name, coeff)) = self.reactants.first() {
            write!(fmt, "\tform: {}{}", coeff, name)?;
            for (name, coeff) in self.reactants.iter().skip(1) {
                write!(fmt, " + {}{}", coeff, name)?;
            }
        }

        write!(fmt, " -> ").expect("Could not write to formatter.");

        if let Some((name, coeff)) = self.products.first() {
            write!(fmt, "{}{}", coeff, name)?;
            for (name, coeff) in self.products.iter().skip(1) {
                write!(fmt, " + {}{}", coeff, name)?;
            }
        }

        write!(fmt, "\n\trate: {:<32} ", self.rate)
    }
}
