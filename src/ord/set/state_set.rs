//! State set.

use crate::{
    ord::{Set, StateKey},
    world::State,
};

/// Alias for the state set.
pub type StateSet = Set<StateKey, State>;
