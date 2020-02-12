//! Camera structure.

use crate::{access, clone};
use attr::json;
use nalgebra::Point3;

/// Image forming structure.
#[json]
pub struct Camera {
    /// Image resolution.
    res: (usize, usize),
    /// Position.
    pos: Point3<f64>,
    /// Field of view [deg].
    fov: f64,
}

impl Camera {
    clone!(res, (usize, usize));
    access!(pos, Point3<f64>);
    access!(fov, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: (usize, usize), pos: Point3<f64>, fov: f64) -> Self {
        debug_assert!(res.0 > 0);
        debug_assert!(res.1 > 0);
        debug_assert!(fov > 0.0);

        Self { res, pos, fov }
    }
}
