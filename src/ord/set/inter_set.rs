//! Intersection set.

use crate::{
    ord::{InterKey, MatKey, Set, SurfKey},
    world::Interface,
};

/// Alias for the interface set.
pub type InterSet = Set<InterKey, Interface>;

impl InterSet {
    /// Get a list of all surface keys used by the interface set.
    #[inline]
    #[must_use]
    pub fn surf_keys(&self) -> Vec<SurfKey> {
        self.map
            .values()
            .map(|inter| inter.surf().clone())
            .collect()
    }

    /// Get a list of all material keys used by the interface set.
    #[inline]
    #[must_use]
    pub fn mat_keys(&self) -> Vec<MatKey> {
        let in_mats: Vec<_> = self
            .map
            .values()
            .map(|inter| inter.in_mat().clone())
            .collect();

        let mut out_mats: Vec<_> = self
            .map
            .values()
            .map(|inter| inter.out_mat().clone())
            .collect();

        let mut mats = in_mats;
        mats.append(&mut out_mats);

        mats
    }
}
