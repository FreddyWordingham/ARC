//! State set.

use crate::{
    ord::{Set, SpecKey, StateKey},
    world::State,
};

/// Alias for the state set.
pub type StateSet = Set<StateKey, State>;

impl StateSet {
    /// Get a list of all species keys used by the state set.
    #[inline]
    #[must_use]
    pub fn spec_keys(&self) -> Vec<SpecKey> {
        let mut keys = Vec::new();

        for state in self.map.values() {
            for conc_key in state.concs().map().keys() {
                keys.push(conc_key.clone());
            }

            for source_key in state.concs().map().keys() {
                keys.push(source_key.clone());
            }
        }

        keys.sort();
        keys.dedup();

        keys
    }
}
