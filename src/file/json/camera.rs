//! Camera implementation.

use crate::{access, clone, sim::Camera as SimCam};
use attr::json;
use nalgebra::{Point3, Vector3};

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

    /// Build a simulation camera.
    pub fn build(&self) -> SimCam {
        SimCam::new()
    }
}
