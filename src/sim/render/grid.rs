//! Camera implementation.

use crate::geom::Mesh;
use crate::sim::render::Group;

/// Adaptive partitioning grid.
pub struct Grid {}

impl Grid {
    /// Construct a new grid.
    pub fn new(_meshes: &[(Mesh, Group)]) -> Self {
        Self {}
    }
}
