//! Cell implementation.

use crate::geom::{surf::collide::Collide, Aabb, Mesh};
// use crate::geom::{surf::collide::Collide, Aabb, Mesh, Ray, SmoothTriangle, Trace};
use crate::sim::panda::{GridSettings, Group};
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
        let boundary = Self::init_boundary(grid_settings, surfaces);

        Self::Root {
            boundary, // children: [],
        }
    }

    /// Create a Aabb encompassing all the given meshes.
    #[inline]
    #[must_use]
    fn init_boundary(grid_settings: &GridSettings, surfaces: &[(Group, Vec<Mesh>)]) -> Aabb {
        let mut mins = Point3::new(0.0, 0.0, 0.0);
        let mut maxs = mins;

        for (_group, meshes) in surfaces {
            for mesh in meshes {
                let (mesh_mins, mesh_maxs) = mesh.bounding_box().mins_maxs();

                for (grid_min, mesh_min) in mins.iter_mut().zip(mesh_mins.iter()) {
                    if mesh_min < grid_min {
                        *grid_min = *mesh_min;
                    }
                }

                for (grid_max, mesh_max) in maxs.iter_mut().zip(mesh_maxs.iter()) {
                    if mesh_max > grid_max {
                        *grid_max = *mesh_max;
                    }
                }
            }
        }

        let mut boundary = Aabb::new(mins, maxs);

        boundary.expand(grid_settings.padding());

        boundary
    }
}
