//! Verse implementation.

use crate::{
    access,
    ord::{InterKey, InterSet, LightKey, ReactKey, Set},
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
    pub fn form(&self, in_dir: &Path) -> WorldVerse {
        let mut inter_names = self.inters.clone();
        inter_names.sort();
        inter_names.dedup();
        let inters: InterSet = Set::load(&in_dir.join("interfaces"), &inter_names, "json");

        WorldVerse::new(inters)
    }
}
