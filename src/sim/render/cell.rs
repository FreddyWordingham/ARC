//! Cell implementation.

use crate::geom::surf::collide::Collide;
use crate::geom::{Aabb, Mesh, SmoothTriangle};
use crate::sim::render::Group;
use nalgebra::Point3;

/// Grid cell enumeration.
///
///         6npp   7ppp
///       4nnp   5pnp
/// z  y    2npn   3ppn
/// | /   0nnn   1pnn
/// |/__x
pub enum Cell<'a> {
    /// Branching cell.
    Branch {
        /// Boundary.
        boundary: Aabb,
        /// Children.
        children: [Box<Cell<'a>>; 8],
    },
    /// Terminal populated cell.
    Leaf {
        /// Boundary.
        boundary: Aabb,
        /// Intersecting triangles.
        tris: Vec<(&'a SmoothTriangle, Group)>,
    },
    /// Terminal empty cell.
    Empty {
        /// Boundary.
        boundary: Aabb,
    },
}

impl<'a> Cell<'a> {
    /// Construct a new root cell.
    #[inline]
    #[must_use]
    pub fn new_root(max_depth: usize, tar_tris: usize, meshes: &'a [(Mesh, Group)]) -> Self {
        debug_assert!(tar_tris > 1);

        let boundary = Self::init_boundary(meshes);

        let mut tris = Vec::new();
        for (mesh, group) in meshes {
            for tri in mesh.tris() {
                if tri.overlap(&boundary) {
                    tris.push((tri, *group));
                }
            }
        }

        let children = Self::init_children(0, max_depth, tar_tris, &boundary, &tris);

        Self::Branch { boundary, children }
    }

    /// Initialise the boundary of the root cell.
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

    /// Initialise the children for a parent cell.
    #[inline]
    #[must_use]
    fn init_children(
        parent_depth: usize,
        max_depth: usize,
        tar_tris: usize,
        parent_boundary: &Aabb,
        potential_tris: &[(&'a SmoothTriangle, Group)],
    ) -> [Box<Cell<'a>>; 8] {
        let mins = parent_boundary.mins();
        let min_x = mins.x;
        let min_y = mins.y;
        let min_z = mins.z;

        let center = parent_boundary.centre();
        let cen_x = center.x;
        let cen_y = center.y;
        let cen_z = center.z;

        let hws = parent_boundary.half_widths();
        let hw_x = hws.x;
        let hw_y = hws.y;
        let hw_z = hws.z;

        let nnn = Box::new(Self::init_child(
            parent_depth,
            max_depth,
            tar_tris,
            Aabb::new(
                Point3::new(min_x, min_y, min_z),
                Point3::new(min_x + hw_x, min_y + hw_y, min_z + hw_z),
            ),
            potential_tris,
        ));
        let pnn = Box::new(Self::init_child(
            parent_depth,
            max_depth,
            tar_tris,
            Aabb::new(
                Point3::new(min_x + hw_x, min_y, min_z),
                Point3::new(cen_z + hw_x, min_y + hw_y, min_z + hw_z),
            ),
            potential_tris,
        ));
        let npn = Box::new(Self::init_child(
            parent_depth,
            max_depth,
            tar_tris,
            Aabb::new(
                Point3::new(min_x, min_y + hw_x, min_z),
                Point3::new(min_x + hw_x, cen_y + hw_y, min_z + hw_z),
            ),
            potential_tris,
        ));
        let ppn = Box::new(Self::init_child(
            parent_depth,
            max_depth,
            tar_tris,
            Aabb::new(
                Point3::new(min_x + hw_y, min_y + hw_x, min_z),
                Point3::new(cen_x + hw_x, cen_y + hw_y, min_z + hw_z),
            ),
            potential_tris,
        ));

        let nnp = Box::new(Self::init_child(
            parent_depth,
            max_depth,
            tar_tris,
            Aabb::new(
                Point3::new(min_x, min_y, min_z + hw_z),
                Point3::new(min_x + hw_x, min_y + hw_y, cen_z + hw_z),
            ),
            potential_tris,
        ));
        let pnp = Box::new(Self::init_child(
            parent_depth,
            max_depth,
            tar_tris,
            Aabb::new(
                Point3::new(min_x + hw_x, min_y, min_z + hw_z),
                Point3::new(cen_z + hw_x, min_y + hw_y, min_z + hw_z),
            ),
            potential_tris,
        ));
        let npp = Box::new(Self::init_child(
            parent_depth,
            max_depth,
            tar_tris,
            Aabb::new(
                Point3::new(min_x, min_y + hw_x, min_z + hw_z),
                Point3::new(min_x + hw_x, cen_y + hw_y, cen_z + hw_z),
            ),
            potential_tris,
        ));
        let ppp = Box::new(Self::init_child(
            parent_depth,
            max_depth,
            tar_tris,
            Aabb::new(
                Point3::new(min_x + hw_y, min_y + hw_x, min_z + hw_z),
                Point3::new(cen_x + hw_x, cen_y + hw_y, cen_z + hw_z),
            ),
            potential_tris,
        ));

        [nnn, pnn, npn, ppn, nnp, pnp, npp, ppp]
    }

    /// Initialise the boundary of the root cell.
    #[inline]
    #[must_use]
    fn init_child(
        parent_depth: usize,
        max_depth: usize,
        tar_tris: usize,
        boundary: Aabb,
        potential_tris: &[(&'a SmoothTriangle, Group)],
    ) -> Self {
        debug_assert!(parent_depth < max_depth);
        let depth = parent_depth + 1;

        let mut tris = Vec::new();
        for (tri, group) in potential_tris {
            if tri.overlap(&boundary) {
                tris.push((*tri, *group));
            }
        }

        if tris.is_empty() {
            return Self::Empty { boundary };
        }

        if (tris.len() <= tar_tris) || (depth >= max_depth) {
            return Self::Leaf { boundary, tris };
        }

        let children = Self::init_children(0, max_depth, tar_tris, &boundary, &tris);

        Self::Branch { boundary, children }
    }

    /// Determine the number of leaf cells used by the grid.
    #[inline]
    #[must_use]
    pub fn num_leaves(&self) -> usize {
        match self {
            Self::Branch { children, .. } => children.iter().map(|c| c.num_leaves()).sum(),
            Self::Leaf { .. } => 1,
            Self::Empty { .. } => 0,
        }
    }

    /// Determine the number of leaf cells used by the grid.
    #[inline]
    #[must_use]
    pub fn num_empty(&self) -> usize {
        match self {
            Self::Branch { children, .. } => children.iter().map(|c| c.num_empty()).sum(),
            Self::Leaf { .. } => 0,
            Self::Empty { .. } => 1,
        }
    }

    /// Determine the number of branch cells used by the grid.
    #[inline]
    #[must_use]
    pub fn num_branches(&self) -> usize {
        match self {
            Self::Branch { children, .. } => {
                children.iter().map(|c| c.num_empty()).sum::<usize>() + 1
            }
            Self::Leaf { .. } => 0,
            Self::Empty { .. } => 0,
        }
    }

    /// Determine the total number of cells used by the grid.
    #[inline]
    #[must_use]
    pub fn num_cells(&self) -> usize {
        match self {
            Self::Branch { children, .. } => {
                children.iter().map(|c| c.num_cells()).sum::<usize>() + 1
            }
            Self::Leaf { .. } => 1,
            Self::Empty { .. } => 1,
        }
    }

    /// Determine the average number of triangles in each leaf cell.
    #[inline]
    #[must_use]
    pub fn num_tri_refs(&self) -> usize {
        match self {
            Self::Branch { children, .. } => children.iter().map(|c| c.num_tri_refs()).sum(),
            Self::Leaf { tris, .. } => tris.len(),
            Self::Empty { .. } => 0,
        }
    }

    /// Determine the average number of triangles in each leaf cell.
    #[inline]
    #[must_use]
    pub fn ave_leaf_tris(&self) -> f64 {
        self.num_tri_refs() as f64 / self.num_leaves() as f64
    }
}
