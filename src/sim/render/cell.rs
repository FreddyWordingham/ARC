//! Cell implementation.

use crate::geom::surf::collide::Collide;
use crate::geom::{Aabb, Mesh, SmoothTriangle};
use crate::sim::render::Group;
use nalgebra::Point3;

// macro_rules! children {
//     () => {
//         nnn: Box<Cell<'a>>,
//         pnn: Box<Cell<'a>>,
//         npn: Box<Cell<'a>>,
//         ppn: Box<Cell<'a>>,
//         nnp: Box<Cell<'a>>,
//         pnp: Box<Cell<'a>>,
//         npp: Box<Cell<'a>>,
//         ppp: Box<Cell<'a>>,
//     };
// }

/// Grid cell enumeration.
///
///         6npp   7ppp
///       4nnp   5pnp
/// z  y    2npn   3ppn
/// | /   0nnn   1pnn
/// |/__x
pub enum Cell<'a> {
    /// Tree root cell.
    Root {
        /// Boundary.
        boundary: Aabb,
        /// Children.
        nnn: Box<Cell<'a>>,
        pnn: Box<Cell<'a>>,
        npn: Box<Cell<'a>>,
        ppn: Box<Cell<'a>>,
        nnp: Box<Cell<'a>>,
        pnp: Box<Cell<'a>>,
        npp: Box<Cell<'a>>,
        ppp: Box<Cell<'a>>,
    },
    /// Branching cell.
    Branch {
        /// Children.
        nnn: Box<Cell<'a>>,
        pnn: Box<Cell<'a>>,
        npn: Box<Cell<'a>>,
        ppn: Box<Cell<'a>>,
        nnp: Box<Cell<'a>>,
        pnp: Box<Cell<'a>>,
        npp: Box<Cell<'a>>,
        ppp: Box<Cell<'a>>,
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
    pub fn new_root(
        min_depth: usize,
        max_depth: usize,
        tar_tris: usize,
        meshes: &'a [(Mesh, Group)],
    ) -> Self {
        debug_assert!(min_depth <= max_depth);
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

        let mins = boundary.mins();
        let min_x = mins.x;
        let min_y = mins.y;
        let min_z = mins.z;

        let center = boundary.centre();
        let cen_x = center.x;
        let cen_y = center.y;
        let cen_z = center.z;

        let hws = boundary.half_widths();
        let hw_x = hws.x;
        let hw_y = hws.y;
        let hw_z = hws.z;

        let nnn = Box::new(Self::init_child(
            Aabb::new(
                Point3::new(min_x, min_y, min_z),
                Point3::new(min_x + hw_x, min_y + hw_y, min_z + hw_z),
            ),
            &tris,
        ));
        let pnn = Box::new(Self::init_child(
            Aabb::new(
                Point3::new(min_x + hw_x, min_y, min_z),
                Point3::new(cen_z + hw_x, min_y + hw_y, min_z + hw_z),
            ),
            &tris,
        ));
        let npn = Box::new(Self::init_child(
            Aabb::new(
                Point3::new(min_x, min_y + hw_x, min_z),
                Point3::new(min_x + hw_x, cen_y + hw_y, min_z + hw_z),
            ),
            &tris,
        ));
        let ppn = Box::new(Self::init_child(
            Aabb::new(
                Point3::new(min_x + hw_y, min_y + hw_x, min_z),
                Point3::new(cen_x + hw_x, cen_y + hw_y, min_z + hw_z),
            ),
            &tris,
        ));

        let nnp = Box::new(Self::init_child(
            Aabb::new(
                Point3::new(min_x, min_y, min_z + hw_z),
                Point3::new(min_x + hw_x, min_y + hw_y, cen_z + hw_z),
            ),
            &tris,
        ));
        let pnp = Box::new(Self::init_child(
            Aabb::new(
                Point3::new(min_x + hw_x, min_y, min_z + hw_z),
                Point3::new(cen_z + hw_x, min_y + hw_y, min_z + hw_z),
            ),
            &tris,
        ));
        let npp = Box::new(Self::init_child(
            Aabb::new(
                Point3::new(min_x, min_y + hw_x, min_z + hw_z),
                Point3::new(min_x + hw_x, cen_y + hw_y, cen_z + hw_z),
            ),
            &tris,
        ));
        let ppp = Box::new(Self::init_child(
            Aabb::new(
                Point3::new(min_x + hw_y, min_y + hw_x, min_z + hw_z),
                Point3::new(cen_x + hw_x, cen_y + hw_y, cen_z + hw_z),
            ),
            &tris,
        ));

        Self::Root {
            boundary,
            nnn,
            pnn,
            npn,
            ppn,
            nnp,
            pnp,
            npp,
            ppp,
        }
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

    /// Initialise the boundary of the root cell.
    #[inline]
    #[must_use]
    fn init_child(boundary: Aabb, potential_tris: &[(&'a SmoothTriangle, Group)]) -> Self {
        let mut tris = Vec::new();
        for (tri, group) in potential_tris {
            if tri.overlap(&boundary) {
                tris.push((tri, *group));
            }
        }

        if tris.is_empty() {
            return Self::Empty { boundary };
        }

        Self::Empty { boundary }
    }
}
