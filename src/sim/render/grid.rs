//! Camera implementation.

use crate::geom::surf::collide::Collide;
use crate::geom::{Aabb, Mesh};
use crate::sim::render::Group;

/// Adaptive partitioning grid.
pub struct Grid {
    /// Outer-most boundary of the grid region.
    boundary: Aabb,
}

impl Grid {
    /// Construct a new grid.
    #[inline]
    #[must_use]
    pub fn new(
        min_depth: usize,
        max_depth: usize,
        tar_tris: usize,
        meshes: &[(Mesh, Group)],
    ) -> Self {
        debug_assert!(min_depth <= max_depth);
        debug_assert!(tar_tris > 1);

        let boundary = Self::init_boundary(meshes);

        Self { boundary }
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
