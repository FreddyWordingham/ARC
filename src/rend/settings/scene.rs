//! Scene input settings implementation.

use crate::{access, file::Transform, rend::Group};
use attr::json;
use nalgebra::Point3;

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
}
