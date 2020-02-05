//! Set implementation.

use crate::{access, file::Load};
use log::info;
use std::{collections::BTreeMap, fmt::Display, path::Path};

pub mod inter_set;
pub mod light_set;
pub mod mat_set;
pub mod mesh_set;
pub mod react_set;
pub mod spec_set;
pub mod surf_set;

pub use self::{
    inter_set::*, light_set::*, mat_set::*, mesh_set::*, react_set::*, spec_set::*, surf_set::*,
};

/// Set mapping.
pub struct Set<K, T> {
    /// Internal map.
    map: BTreeMap<K, T>,
}

impl<K: Display + Ord, T> Set<K, T> {
    access!(map, BTreeMap<K, T>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: BTreeMap<K, T>) -> Self {
        Self { map }
    }

    /// Access a value within the map.
    #[inline]
    #[must_use]
    pub fn get(&self, key: &K) -> &T {
        self.map
            .get(key)
            .unwrap_or_else(|| panic!("Key {} does not exist within the set.", key))
    }
}

impl<K: Display + Clone + Ord, T: Load> Set<K, T> {
    /// Load a set of files.
    #[inline]
    #[must_use]
    pub fn load(dir: &Path, keys: &[K], ext: &str) -> Self {
        let mut map = BTreeMap::new();

        for key in keys {
            let path = dir.join(format!("{}.{}", key, ext));
            info!("Loading: {}", path.display());

            map.insert((*key).clone(), T::load(&path));
        }

        Set::new(map)
    }
}

// /// Alias for the light set.
// pub type LightSet = Set<LightKey, Light>;

// /// Alias for the reaction set.
// pub type ReactSet = Set<ReactKey, Reaction>;

// /// Alias for the species set.
// pub type SpecSet = Set<SpecKey, Reaction>;
