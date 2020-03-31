//! Camera implementation.

use crate::geom::surf::collide::Collide;
use crate::geom::{Aabb, Mesh, Triangle};
use crate::sim::render::Group;

/// Grid cell enumeration.
pub enum Grid<'a> {
    /// Tree root cell.
    Root {
        /// Boundary.
        boundary: Aabb,
    },
    /// Terminal populated cell.
    Leaf {
        /// Boundary.
        boundary: Aabb,
        /// Intersecting triangles.
        tris: Vec<(Group, &'a Triangle)>,
    },
    /// Terminal empty cell.
    Empty {
        /// Boundary.
        boundary: Aabb,
    },
}

impl<'a> Grid<'a> {
    /// Construct a new grid.
    #[inline]
    #[must_use]
    pub fn new_root(
        min_depth: usize,
        max_depth: usize,
        tar_tris: usize,
        meshes: &[(Mesh, Group)],
    ) -> Self {
        debug_assert!(min_depth <= max_depth);
        debug_assert!(tar_tris > 1);

        let boundary = Self::init_boundary(meshes);

        Self::Root { boundary }
    }

    /// Initialise the boundary of the grid.
    #[inline]
    #[must_use]
    fn init_boundary(meshes: &[(Mesh, Group)]) -> Aabb {
        let (mut grid_mins, mut grid_maxs) = meshes
            .get(0)
            .expect("Grid contains no meshes.")
            .0
            .bounding_box()
            .mins_maxs();

        for (mesh, _group) in meshes.iter().skip(1) {
            let (mesh_mins, mesh_maxs) = mesh.bounding_box().mins_maxs();

            for (grid_min, mesh_min) in grid_mins.iter_mut().zip(mesh_mins.iter()) {
                if mesh_min < grid_min {
                    *grid_min = *mesh_min;
                }
            }

            for (grid_max, mesh_max) in grid_maxs.iter_mut().zip(mesh_maxs.iter()) {
                if mesh_max > grid_max {
                    *grid_max = *mesh_max;
                }
            }
        }

        Aabb::new(grid_mins, grid_maxs)
    }
}
