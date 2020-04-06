//! Scene input settings implementation.

use crate::{access, file::Transform, rend::Group};
use attr::json;
use nalgebra::Point3;
use std::path::Path;

/// Scene settings.
#[allow(clippy::type_complexity)]
#[json]
pub struct Scene {
    /// Sun position.
    sun_pos: Point3<f64>,
    /// Traceable surface groups.
    groups: Vec<(Group, Vec<(String, Option<Transform>)>)>,
}

impl Scene {
    access!(sun_pos, Point3<f64>);
    access!(groups, Vec<(Group, Vec<(String, Option<Transform>)>)>);

    /// Build a rendering scene.
    #[inline]
    #[must_use]
    pub fn build(&self, _in_dir: &Path) -> crate::rend::Scene {
        crate::rend::Scene::new()
    }
}
