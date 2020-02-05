//! Load trait.

use json5;
use serde::Deserialize;
use std::{collections::BTreeMap, fs::read_to_string, path::Path};

/// Types implementing this trait can be loaded from a file.
pub trait Load {
    /// Deserialize the type from a given file.
    fn load(path: &Path) -> Self;
}

/// Deserialise the type in json format.
#[inline]
#[must_use]
pub fn from_json<T>(path: &Path) -> T
where
    for<'de> T: Deserialize<'de>,
{
    json5::from_str(
        &read_to_string(path).unwrap_or_else(|_| panic!("Unable to read file: {}", path.display())),
    )
    .expect("Unable to parse json file.")
}

/// Load a map of instances.
#[inline]
#[must_use]
pub fn map<T: Load>(dir: &Path, names: &[String], ext: &str) -> BTreeMap<String, T> {
    let mut map = BTreeMap::new();

    for name in names {
        let path = dir.join(name).with_extension(ext);
        map.insert(name.to_string(), T::load(&path));
    }

    map
}
