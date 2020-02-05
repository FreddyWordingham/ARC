//! Light set.

use crate::{
    ord::{LightKey, Set, SurfKey},
    world::Light,
};

/// Alias for the light set.
pub type LightSet = Set<LightKey, Light>;

impl LightSet {
    /// Get a list of all surface keys used by the light set.
    #[inline]
    #[must_use]
    pub fn surf_keys(&self) -> Vec<SurfKey> {
        self.map
            .values()
            .map(|light| light.surf().clone())
            .collect()
    }
}
