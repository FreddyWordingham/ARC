//! Reaction set.

use crate::{
    chem::Reaction,
    ord::{ReactKey, Set, SpecKey},
};

/// Alias for the reaction set.
pub type ReactSet = Set<ReactKey, Reaction>;

impl ReactSet {
    /// Get a list of all species keys used by the reaction set.
    #[inline]
    #[must_use]
    pub fn spec_keys(&self) -> Vec<SpecKey> {
        let mut keys = Vec::new();

        for react in self.map.values() {
            for (key, _c) in react.reactants() {
                keys.push(key.clone());
            }

            for (key, _c) in react.products() {
                keys.push(key.clone());
            }
        }

        keys.sort();
        keys.dedup();

        keys
    }
}
