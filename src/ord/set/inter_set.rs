//! Intersection set.

use crate::{
    ord::{InterKey, Set},
    world::Interface,
};

/// Alias for the interface set.
pub type InterSet = Set<InterKey, Interface>;
