//! Region set.

use crate::{
    ord::{RegionKey, Set, StateKey, SurfKey},
    world::Region,
};

/// Alias for the region set.
pub type RegionSet = Set<RegionKey, Region>;

impl RegionSet {
    /// Get a list of all surface keys used by the interface set.
    #[inline]
    #[must_use]
    pub fn surf_keys(&self) -> Vec<SurfKey> {
        self.map
            .values()
            .map(|inter| inter.surf().clone())
            .collect()
    }

    /// Get a list of all state keys used by the interface set.
    #[inline]
    #[must_use]
    pub fn state_keys(&self) -> Vec<StateKey> {
        let in_states: Vec<_> = self
            .map
            .values()
            .map(|inter| inter.in_state().clone())
            .collect();

        let mut out_states: Vec<_> = self
            .map
            .values()
            .map(|inter| inter.out_state().clone())
            .collect();

        let mut states = in_states;
        states.append(&mut out_states);

        states
    }
}
