//! Reaction set.

use crate::{
    chem::Reaction,
    ord::{ReactKey, Set},
};

/// Alias for the reaction set.
pub type ReactSet = Set<ReactKey, Reaction>;
