//! Cell implementation.

use crate::geom::{surf::collide::Collide, Aabb, Mesh, SmoothTriangle};
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
pub enum Cell<'a> {
    /// Root cell.
    Root {
        /// Boundary.
        boundary: Aabb,
        /// Children.
        children: [Box<Cell<'a>>; 8],
    },
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
        tris: Vec<(Group, &'a SmoothTriangle)>,
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
    pub fn new_root(settings: &GridSettings, surfaces: &'a [(Group, Vec<Mesh>)]) -> Self {
        let mut mins = None;
        let mut maxs = None;

        for (_group, meshes) in surfaces {
            for mesh in meshes {
                let (mesh_mins, mesh_maxs) = mesh.bounding_box().mins_maxs();

                if mins.is_none() {
                    mins = Some(mesh_mins);
                } else {
                    for (grid_min, mesh_min) in mins
                        .expect("Missing minimum point.")
                        .iter_mut()
                        .zip(mesh_mins.iter())
                    {
                        if mesh_min < grid_min {
                            *grid_min = *mesh_min;
                        }
                    }
                }

                if maxs.is_none() {
                    maxs = Some(mesh_maxs);
                } else {
                    for (grid_max, mesh_max) in maxs
                        .expect("Missing maximum point.")
                        .iter_mut()
                        .zip(mesh_maxs.iter())
                    {
                        if mesh_max > grid_max {
                            *grid_max = *mesh_max;
                        }
                    }
                }
            }
        }
        let boundary = Aabb::new(
            mins.expect("Missing minimum point."),
            maxs.expect("Missing minimum point."),
        );

        let mut tris = Vec::new();
        for (group, meshes) in surfaces {
            for mesh in meshes {
                tris.reserve(mesh.tris().len());
                for tri in mesh.tris() {
                    tris.push((*group, tri));
                }
            }
        }

        let children = Self::init_children(&settings, &boundary, 0, &tris);

        Self::Root { boundary, children }
    }

    /// Initialise the children of a branching cell.
    #[inline]
    #[must_use]
    fn init_children(
        settings: &GridSettings,
        parent_boundary: &Aabb,
        depth: u32,
        potential_tris: &[(Group, &'a SmoothTriangle)],
    ) -> [Box<Cell<'a>>; 8] {
        debug_assert!(depth <= settings.max_depth());
        debug_assert!(!potential_tris.is_empty());

        let hws = parent_boundary.half_widths();
        let make_child = |min_x: f64, min_y: f64, min_z: f64| {
            let min = Point3::new(min_x, min_y, min_z);
            Box::new(Self::new_child(
                settings,
                Aabb::new(min, min + hws),
                depth,
                potential_tris,
            ))
        };

        let mins = parent_boundary.mins();
        let min_x = mins.x;
        let min_y = mins.y;
        let min_z = mins.z;

        let nnn = make_child(min_x, min_y, min_z);
        let pnn = make_child(min_x + hws.x, min_y, min_z);
        let npn = make_child(min_x, min_y + hws.y, min_z);
        let ppn = make_child(min_x + hws.x, min_y + hws.y, min_z);
        let nnp = make_child(min_x, min_y, min_z + hws.z);
        let pnp = make_child(min_x + hws.x, min_y, min_z + hws.z);
        let npp = make_child(min_x, min_y + hws.y, min_z + hws.z);
        let ppp = make_child(min_x + hws.x, min_y + hws.y, min_z + hws.z);

        [nnn, pnn, npn, ppn, nnp, pnp, npp, ppp]
    }

    /// Initialise a child cell.
    #[inline]
    #[must_use]
    fn new_child(
        settings: &GridSettings,
        boundary: Aabb,
        depth: u32,
        potential_tris: &[(Group, &'a SmoothTriangle)],
    ) -> Self {
        debug_assert!(depth <= settings.max_depth());

        let mut detection_vol = boundary.clone();
        detection_vol.expand(settings.padding());

        let mut tris = Vec::new();
        for (group, tri) in potential_tris {
            if tri.overlap(&detection_vol) {
                tris.push((*group, *tri));
            }
        }

        if tris.is_empty() {
            return Self::Empty { boundary };
        }

        if (tris.len() <= settings.tar_tris()) || (depth >= settings.max_depth()) {
            return Self::Leaf { boundary, tris };
        }

        let children = Self::init_children(settings, &boundary, depth + 1, &tris);

        Self::Branch { boundary, children }
    }
}
