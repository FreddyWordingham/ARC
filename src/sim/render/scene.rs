//! Scene implementation.

use crate::{
    access,
    geom::{Aabb, Collide, Mesh},
    sim::render::Group,
};
use nalgebra::Point3;
use std::collections::BTreeMap;

/// Scene structure.
pub struct Scene {
    /// Sun position.
    sun_pos: Point3<f64>,
    /// Boundary.
    boundary: Aabb,
    /// Meshes forming the scene.
    groups: BTreeMap<Group, Vec<Mesh>>,
}

impl Scene {
    access!(sun_pos, Point3<f64>);
    access!(boundary, Aabb);
    access!(groups, BTreeMap<Group, Vec<Mesh>>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(sun_pos: Point3<f64>, groups: BTreeMap<Group, Vec<Mesh>>) -> Self {
        let boundary = Self::init_boundary(&groups);

        Self {
            sun_pos,
            boundary,
            groups,
        }
    }

    /// Initialise the boundary encompassing all of the mesh vertices.
    #[inline]
    #[must_use]
    fn init_boundary(groups: &BTreeMap<Group, Vec<Mesh>>) -> Aabb {
        let mut mins = None;
        let mut maxs = None;

        for meshes in groups.values() {
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

        Aabb::new(
            mins.expect("Missing minimum point."),
            maxs.expect("Missing maximum point."),
        )
    }

    /// Calculate the total number of triangles used by the group.
    #[inline]
    #[must_use]
    pub fn group_tris(&self, group: Group) -> usize {
        let mut total_tris = 0;
        if let Some(meshes) = self.groups.get(&group) {
            for mesh in meshes {
                total_tris += mesh.tris().len();
            }
        }

        total_tris
    }

    /// Calculate the total number of triangles used by the scene.
    #[inline]
    #[must_use]
    pub fn total_tris(&self) -> usize {
        self.groups
            .values()
            .map(|meshes| meshes.iter().map(|m| m.tris().len()).sum::<usize>())
            .sum()
    }
}
