//! Redirection during json file loading.

use crate::file::Load;
use std::path::Path;

/// Redirection wrapper structure.
#[derive(Debug)]
pub struct Redirect<T> {
    /// Wrapped data.
    data: T,
}

impl<T> Redirect<T> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<'de, T: Load> serde::Deserialize<'de> for Redirect<T> {
    #[inline]
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path = Path::new("beans.txt");
        Ok(Self::new(T::load(path)))
    }
}
