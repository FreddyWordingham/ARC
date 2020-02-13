//! Camera implementation.

use crate::{access, clone, sim::Camera as SimCamera};
use attr::json;
use nalgebra::{Point3, Unit, Vector3};

/// Camera building structure.
#[json]
pub struct Camera {
    /// Position.
    pos: Point3<f64>,
    /// Direction.
    dir: Vector3<f64>,
    /// Field of view [deg].
    fov: f64,
}

impl Camera {
    access!(pos, Point3<f64>);
    access!(dir, Vector3<f64>);
    clone!(fov, f64);

    /// Build a camera.
    #[inline]
    #[must_use]
    pub fn build(&self) -> SimCamera {
        SimCamera::new(
            self.pos,
            Unit::new_normalize(self.dir),
            self.fov.to_radians(),
        )
    }
}
