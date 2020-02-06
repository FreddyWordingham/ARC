//! Interface implementation.

use crate::{
    access,
    ord::{StateKey, SurfKey},
};
use attr::json;

/// Region boundary structure.
#[json]
pub struct Region {
    /// Surface mesh.
    surf: SurfKey,
    /// Inside initial state.
    in_state: StateKey,
    /// Outside initial state.
    out_state: StateKey,
}

impl Region {
    access!(surf, SurfKey);
    access!(in_state, StateKey);
    access!(out_state, StateKey);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(surf: SurfKey, in_state: StateKey, out_state: StateKey) -> Self {
        Self {
            surf,
            in_state,
            out_state,
        }
    }
}
