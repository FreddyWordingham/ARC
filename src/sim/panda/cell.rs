//! Cell implementation.

use crate::{
    geom::{surf::collide::Collide, Aabb, Mesh, Ray, SmoothTriangle, Trace},
    sim::panda::{GridSettings, Group, Hit, Scan},
};
use nalgebra::Point3;
use std::fmt::{Display, Formatter, Result};

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
                        .as_mut()
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
                        .as_mut()
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
            maxs.expect("Missing maximum point."),
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

        let children = Self::init_children(settings, &boundary, 0, &tris);

        Self::Root { boundary, children }
    }

    /// Initialise the children of a branching cell.
    #[allow(clippy::similar_names)]
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

    /// Determine the terminal cell containing the given position.
    #[inline]
    #[must_use]
    pub fn find_terminal_cell(&self, pos: &Point3<f64>) -> Option<&Self> {
        if !self.boundary().contains(pos) {
            return None;
        }

        match self {
            Self::Leaf { .. } | Self::Empty { .. } => Some(self),
            Self::Root { boundary, children } | Self::Branch { boundary, children } => {
                let mut index = 0;
                let c = boundary.centre();

                if pos.x >= c.x {
                    index += 1;
                }
                if pos.y >= c.y {
                    index += 2;
                }
                if pos.z >= c.z {
                    index += 4;
                }
                children
                    .get(index)
                    .expect("Invalid index")
                    .find_terminal_cell(pos)
            }
        }
    }

    /// Reference the cell's boundary.
    #[inline]
    #[must_use]
    pub fn boundary(&self) -> &Aabb {
        match self {
            Self::Root { boundary, .. }
            | Self::Branch { boundary, .. }
            | Self::Leaf { boundary, .. }
            | Self::Empty { boundary, .. } => boundary,
        }
    }

    /// Determine the number of cells used.
    #[inline]
    #[must_use]
    pub fn num_cells(&self) -> usize {
        match self {
            Self::Root { children, .. } | Self::Branch { children, .. } => {
                1 + children.iter().map(|ch| ch.num_cells()).sum::<usize>()
            }
            Self::Leaf { .. } | Self::Empty { .. } => 1,
        }
    }

    /// Determine the number of terminal cells used.
    #[inline]
    #[must_use]
    pub fn num_terminal_cells(&self) -> usize {
        match self {
            Self::Root { children, .. } | Self::Branch { children, .. } => children
                .iter()
                .map(|ch| ch.num_terminal_cells())
                .sum::<usize>(),
            Self::Leaf { .. } | Self::Empty { .. } => 1,
        }
    }

    /// Determine the number of branch cells used.
    #[inline]
    #[must_use]
    pub fn num_branch_cells(&self) -> usize {
        match self {
            Self::Root { children, .. } => children
                .iter()
                .map(|ch| ch.num_branch_cells())
                .sum::<usize>(),
            Self::Branch { children, .. } => {
                1 + children
                    .iter()
                    .map(|ch| ch.num_branch_cells())
                    .sum::<usize>()
            }
            Self::Leaf { .. } | Self::Empty { .. } => 0,
        }
    }

    /// Determine the total number of terminal cells used.
    #[inline]
    #[must_use]
    pub fn num_leaf_cells(&self) -> usize {
        match self {
            Self::Root { children, .. } | Self::Branch { children, .. } => {
                children.iter().map(|ch| ch.num_leaf_cells()).sum::<usize>()
            }
            Self::Leaf { .. } => 1,
            Self::Empty { .. } => 0,
        }
    }

    /// Determine the total number of empty cells used.
    #[inline]
    #[must_use]
    pub fn num_empty_cells(&self) -> usize {
        match self {
            Self::Root { children, .. } | Self::Branch { children, .. } => children
                .iter()
                .map(|ch| ch.num_empty_cells())
                .sum::<usize>(),
            Self::Leaf { .. } => 0,
            Self::Empty { .. } => 1,
        }
    }

    /// Determine the average number of triangles in each leaf cell.
    #[inline]
    #[must_use]
    pub fn num_tri_refs(&self) -> usize {
        match self {
            Self::Root { children, .. } | Self::Branch { children, .. } => {
                children.iter().map(|c| c.num_tri_refs()).sum()
            }
            Self::Leaf { tris, .. } => tris.len(),
            Self::Empty { .. } => 0,
        }
    }

    /// Determine the average number of triangles in each leaf cell.
    #[inline]
    #[must_use]
    pub fn ave_leaf_tris(&self) -> f64 {
        self.num_tri_refs() as f64 / self.num_leaf_cells() as f64
    }

    /// Determine the maximum depth used.
    #[inline]
    #[must_use]
    pub fn max_depth(&self) -> usize {
        match self {
            Self::Root { children, .. } => children
                .iter()
                .map(|c| c.max_depth())
                .max()
                .expect("Failed to determine depth."),
            Self::Branch { children, .. } => {
                1 + children
                    .iter()
                    .map(|c| c.max_depth())
                    .max()
                    .expect("Failed to determine depth.")
            }
            Self::Leaf { .. } | Self::Empty { .. } => 1,
        }
    }

    /// Determine what a ray would observe within the cell.
    #[inline]
    #[must_use]
    pub fn observe(&self, mut ray: Ray, bump_dist: f64) -> Option<Hit> {
        debug_assert!(bump_dist > 0.0);

        let mut dist_travelled = 0.0;

        // Move the ray to within the domain of the grid if it isn't already within it.
        if let Some(dist) = self.boundary().dist(&ray) {
            if !self.boundary().contains(ray.pos()) {
                let d = dist + bump_dist;
                ray.travel(d);
                dist_travelled += d;
            }
        } else {
            return None;
        }

        // Trace forward until leaving the grid or observing something.
        while let Some(cell) = self.find_terminal_cell(ray.pos()) {
            match cell.hit_scan(&ray) {
                Scan::Surface { hit } => {
                    return Some(Hit::new(
                        hit.group(),
                        hit.dist() + dist_travelled,
                        *hit.norm(),
                    ));
                }
                Scan::Boundary { dist } => {
                    let d = dist + bump_dist;
                    ray.travel(d);
                    dist_travelled += d;
                }
            }
        }

        None
    }

    /// Scan for hits within the cell.
    #[inline]
    #[must_use]
    pub fn hit_scan(&self, ray: &Ray) -> Scan {
        match self {
            Self::Leaf { boundary, tris } => {
                let mut nearest: Option<Hit> = None;
                for (group, tri) in tris {
                    if let Some((dist, norm)) = tri.dist_norm(ray) {
                        if nearest.is_none()
                            || (nearest
                                .as_ref()
                                .expect("Failed to resolve hit scan.")
                                .dist()
                                > dist)
                        {
                            nearest = Some(Hit::new(*group, dist, norm));
                        }
                    }
                }

                let boundary_dist = boundary.dist(ray).expect("Ray has escaped cell.");
                if let Some(hit) = nearest {
                    if hit.dist() < boundary_dist {
                        return Scan::new_surface_scan(hit);
                    }
                }

                Scan::new_boundary_scan(boundary_dist)
            }
            Self::Empty { boundary } => Scan::new_boundary_scan(
                boundary
                    .dist(ray)
                    .expect("Could not determine boundary distance."),
            ),
            Self::Root { .. } | Self::Branch { .. } => {
                panic!("Should not be performing hit scans on branching cells!");
            }
        }
    }
}

impl<'a> Display for Cell<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt)?;
        writeln!(fmt, "{:>30} : {}", "number of cells", self.num_cells())?;
        writeln!(
            fmt,
            "{:>30} : {}",
            "total terminal cells",
            self.num_terminal_cells()
        )?;
        writeln!(
            fmt,
            "{:>30} : {}",
            "total branches",
            self.num_branch_cells()
        )?;
        writeln!(
            fmt,
            "{:>30} : {}",
            "number of leaves",
            self.num_leaf_cells()
        )?;
        writeln!(
            fmt,
            "{:>30} : {}",
            "number of empty cells",
            self.num_empty_cells()
        )?;
        writeln!(
            fmt,
            "{:>30} : {}",
            "total triangle references",
            self.num_tri_refs()
        )?;
        writeln!(fmt, "{:>30} : {}", "maximum depth", self.max_depth())?;
        writeln!(
            fmt,
            "{:>30} : {}",
            "average triangles per leaf",
            self.ave_leaf_tris()
        )
    }
}
