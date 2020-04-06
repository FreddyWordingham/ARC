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
}
