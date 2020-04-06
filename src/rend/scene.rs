//! Scene implementation.

use crate::{access, geom::Mesh, rend::Group};
use nalgebra::Point3;
use std::collections::BTreeMap;

/// Scene structure.
pub struct Scene {
    /// Sun position.
    sun_pos: Point3<f64>,
    /// Meshes forming the scene.
    groups: BTreeMap<Group, Vec<Mesh>>,
}

impl Scene {
    access!(sun_pos, Point3<f64>);
    access!(groups, BTreeMap<Group, Vec<Mesh>>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(sun_pos: Point3<f64>, groups: BTreeMap<Group, Vec<Mesh>>) -> Self {
        Self { sun_pos, groups }
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
