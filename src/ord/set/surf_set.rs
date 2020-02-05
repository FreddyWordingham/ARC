//! Surface set.

use crate::{
    geom::Mesh,
    ord::{Set, SurfKey},
};

/// Alias for the surface set.
pub type SurfSet = Set<SurfKey, Mesh>;
