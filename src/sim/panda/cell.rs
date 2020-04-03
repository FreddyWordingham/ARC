//! Cell implementation.

use crate::geom::{Aabb, Mesh};
use crate::sim::panda::{GridSettings, Group};
// use crate::geom::{surf::collide::Collide, Aabb, Mesh, Ray, SmoothTriangle, Trace},
use nalgebra::Point3;
// use nalgebra::{Point3, Unit, Vector3};

/// Grid cell enumeration.
///
///         6npp   7ppp
///       4nnp   5pnp
/// z  y    2npn   3ppn
/// | /   0nnn   1pnn
/// |/__x
pub enum Cell {
    /// Root cell.
    Root {
        /// Boundary.
        boundary: Aabb,
        // /// Children.
        // children: [Box<Cell>; 8],
    },
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
    /// Terminal empty cell.
    Empty {
        /// Boundary.
        boundary: Aabb,
    },
}

impl Cell {
    /// Construct a new root cell.
    #[inline]
    #[must_use]
    pub fn new_root(grid_settings: &GridSettings, surfaces: &[(Group, Vec<Mesh>)]) -> Self {
        Self::Root {
            boundary: Aabb::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0)),
            // children: [],
        }
    }
}
