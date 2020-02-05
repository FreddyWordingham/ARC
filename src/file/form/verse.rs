//! Verse implementation.

use crate::{
    access,
    ord::{InterKey, LightKey, ReactKey},
    world::Verse as WorldVerse,
};
use attr::json;
use std::path::Path;

/// Verse construction form.
#[json]
pub struct Verse {
    /// List of interfaces.
    inters: Vec<InterKey>,
    /// List of reactions.
    reacts: Vec<ReactKey>,
    /// List of lights.
    lights: Vec<LightKey>,
}

impl Verse {
    access!(inters, Vec<InterKey>);
    access!(reacts, Vec<ReactKey>);
    access!(lights, Vec<LightKey>);

    /// Form a new instance.
    #[inline]
    #[must_use]
    pub fn form(&self, _in_dir: &Path) -> WorldVerse {
        WorldVerse::new()
    }
}
