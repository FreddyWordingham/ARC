//! Set implementation.

use crate::{
    access,
    file::Load,
    geom::Mesh,
    ord::{MatKey, MeshKey, SurfKey},
    world::Material,
};
use log::info;
use std::{collections::BTreeMap, fmt::Display, path::Path};

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
    pub fn load(dir: &Path, keys: &[K], ext: &str) -> Set<K, T> {
        let mut map = BTreeMap::new();

        for key in keys {
            let path = dir.join(format!("{}.{}", key, ext));
            info!("Loading: {}", path.display());

            map.insert((*key).clone(), T::load(&path));
        }

        Set::new(map)
    }
}

// /// Alias for the interface set.
// pub type InterSet = Set<InterKey, Interface>;

/// Alias for the material set.
pub type MatSet = Set<MatKey, Material>;

/// Alias for the mesh set.
pub type MeshSet = Set<MeshKey, Mesh>;

// /// Alias for the reaction set.
// pub type ReactSet = Set<ReactKey, Reaction>;

// /// Alias for the species set.
// pub type SpecSet = Set<SpecKey, Reaction>;

/// Alias for the surface set.
pub type SurfSet = Set<SurfKey, Mesh>;
