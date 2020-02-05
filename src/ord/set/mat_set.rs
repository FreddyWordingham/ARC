//! Material set.

use crate::{
    ord::{MatKey, Set},
    world::Material,
};

/// Alias for the material set.
pub type MatSet = Set<MatKey, Material>;
