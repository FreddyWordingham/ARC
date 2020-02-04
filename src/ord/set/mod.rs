//! Set implementation.

use crate::{access, ord::Key};
use std::collections::BTreeMap;

/// Set mapping.
pub struct Set<T> {
    /// Internal map.
    map: BTreeMap<Key<T>, T>,
}

impl<T> Set<T> {
    access!(map, BTreeMap<Key<T>, T>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: BTreeMap<Key<T>, T>) -> Self {
        Self { map }
    }
}
