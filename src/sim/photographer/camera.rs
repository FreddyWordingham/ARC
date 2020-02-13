//! Camera implementation.

use crate::geom::Ray;
use nalgebra::{Point3, Vector3};

/// Image forming camera.
pub struct Camera {}

impl Camera {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    /// Generate the nth ray.
    #[inline]
    #[must_use]
    pub fn gen_ray(n: usize, res: (usize, usize)) -> Ray {
        debug_assert!(n < (res.0 * res.1));

        let _xi = n % res.0;
        let _yi = n / res.0;

        Ray::new(Point3::origin(), Vector3::z_axis())
    }
}
