//! Redirection during json file loading.

use crate::file::Load;
use std::path::Path;

/// Redirection wrapper structure.
#[derive(Debug)]
pub struct Redirect<T> {
    data: T,
}

impl<T> Redirect<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<'de, T: Load> serde::Deserialize<'de> for Redirect<T> {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path = Path::new("beans.txt");
        Ok(Redirect::new(T::load(path)))
    }
}
