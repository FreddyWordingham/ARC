//! Cell implementation.

use crate::geom::Mesh;
use crate::sim::panda::{GridSettings, Group};
// use crate::geom::{surf::collide::Collide, Aabb, Mesh, Ray, SmoothTriangle, Trace},
// use nalgebra::{Point3, Unit, Vector3};

/// Grid cell enumeration.
///
///         6npp   7ppp
///       4nnp   5pnp
/// z  y    2npn   3ppn
/// | /   0nnn   1pnn
/// |/__x
pub enum Cell<'a> {
    // /// Root cell.
// Root {
//     /// Boundary.
//     boundary: Aabb,
//     /// Children.
//     children: [Box<Cell<'a>>; 8],
// },
// /// Branching cell.
// Branch {
//     /// Boundary.
//     boundary: Aabb,
//     /// Children.
//     children: [Box<Cell<'a>>; 8],
// },
// /// Terminal populated cell.
// Leaf {
//     /// Boundary.
//     boundary: Aabb,
//     /// Intersecting triangles.
//     tris: Vec<(&'a SmoothTriangle, Group)>,
// },
// /// Terminal empty cell.
// Empty {
//     /// Boundary.
//     boundary: Aabb,
// },
}

impl<'a> Cell<'a> {
    /// Construct a new root cell.
    #[inline]
    #[must_use]
    pub fn new(grid_settings: &GridSettings, surfaces: &'a [(Group, Vec<Mesh>)]) -> Self {}
}
