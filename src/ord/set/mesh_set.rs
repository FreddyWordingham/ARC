//! Mesh set.

use crate::{
    geom::Mesh,
    ord::{MeshKey, Set},
};

/// Alias for the mesh set.
pub type MeshSet = Set<MeshKey, Mesh>;
