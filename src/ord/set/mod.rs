//! Set implementation.

use crate::{access, file::Load, ord::Key};
use log::info;
use std::{collections::BTreeMap, path::Path};

/// Set mapping.
pub struct Set<T> {
    /// Internal map.
    map: BTreeMap<Key, T>,
}

impl<T> Set<T> {
    access!(map, BTreeMap<Key, T>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: BTreeMap<Key, T>) -> Self {
        Self { map }
    }

    /// Access a value within the map.
    #[inline]
    #[must_use]
    pub fn get(&self, key: &Key) -> &T {
        self.map
            .get(key)
            .unwrap_or_else(|| panic!("Key {} does not exist within the set.", key))
    }
}

impl<T: Load> Set<T> {
    /// Load a set of files.
    #[inline]
    #[must_use]
    pub fn load(dir: &Path, keys: &[Key], ext: &str) -> Set<T> {
        let mut map = BTreeMap::new();

        for key in keys {
            let path = dir.join(format!("{}.{}", key, ext));
            info!("Loading: {}", path.display());

            map.insert(key.clone(), T::load(&path));
        }

        Set::new(map)
    }
}
