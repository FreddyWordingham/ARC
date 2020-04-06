//! Camera implementation.

use crate::access;
use nalgebra::{Point3, Unit, Vector3};

/// Image building structure.
pub struct Camera {
    /// Position.
    pos: Point3<f64>,
    /// Forward direction.
    forward: Unit<Vector3<f64>>,
}

impl Camera {
    access!(pos, Point3<f64>);
    access!(forward, Unit<Vector3<f64>>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(pos: Point3<f64>, tar: Point3<f64>) -> Self {
        Self {
            pos,
            forward: Unit::new_normalize(tar - pos),
        }
    }
}
