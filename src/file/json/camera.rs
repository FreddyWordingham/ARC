//! Camera implementation.

use crate::{access, clone, sim::photographer::Camera as SimCam};
use attr::json;
use nalgebra::Point3;

/// Camera building structure.
#[json]
pub struct Camera {
    /// Viewing position.
    pos: Point3<f64>,
    /// Target viewing point.
    tar: Point3<f64>,
    /// Field of view [deg].
    fov: f64,
    /// Image resolution.
    res: (usize, usize),
}

impl Camera {
    access!(pos, Point3<f64>);
    access!(tar, Point3<f64>);
    clone!(fov, f64);
    clone!(res, (usize, usize));

    /// Build a simulation camera.
    #[inline]
    #[must_use]
    pub fn build(&self) -> SimCam {
        SimCam::new(self.pos, self.tar, self.fov.to_radians(), self.res)
    }
}
