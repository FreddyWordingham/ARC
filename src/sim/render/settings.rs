//! Settings implementation.

use crate::access;
use attr::json;
use nalgebra::Point3;

/// Runtime settings.
#[json]
pub struct Settings {
    sun_pos: Point3<f64>,
}

impl Settings {
    access!(sun_pos, Point3<f64>);
}
