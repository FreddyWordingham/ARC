//! Key alias.

use crate::file::{Load, Save};
use json5::to_string;
use std::{
    fs::{read_to_string, write},
    path::Path,
};

use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// Set key structure.
#[derive(Clone, PartialOrd, Eq, PartialEq, Ord, Serialize, Deserialize)]
pub struct Key<T>(String, PhantomData<T>);

impl<T> Save for Key<T> {
    #[inline]
    fn save(&self, path: &Path) {
        write(
            path,
            to_string(&self.0).expect("Unable to serialise object."),
        )
        .expect("Unable to write json file.");
    }
}

impl<T> Load for Key<T> {
    #[inline]
    #[must_use]
    fn load(path: &Path) -> Self {
        let obj: String = json5::from_str(&read_to_string(path).expect("Unable to read file."))
            .expect("Unable to parse json file.");

        Self {
            0: obj,
            1: PhantomData,
        }
    }
}
