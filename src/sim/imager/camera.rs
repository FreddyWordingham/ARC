//! Camera structure.

use crate::{access, clone, geom::Ray};
use attr::json;
use nalgebra::{Point3, Unit, Vector3};

/// Image forming structure.
#[json]
pub struct Camera {
    /// Image resolution.
    res: (u64, u64),
    /// Position.
    pos: Point3<f64>,
    /// Direction.
    dir: Unit<Vector3<f64>>,
    /// Field of view [deg].
    fov: f64,
}

impl Camera {
    clone!(res, (u64, u64));
    access!(pos, Point3<f64>);
    access!(fov, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: (u64, u64), pos: Point3<f64>, fov: f64) -> Self {
        debug_assert!(res.0 > 0);
        debug_assert!(res.1 > 0);
        debug_assert!(fov > 0.0);

        let dir = Unit::new_normalize(Point3::origin() - pos);

        Self { res, pos, dir, fov }
    }

    /// Calculate the total number of pixels.
    #[inline]
    #[must_use]
    pub fn num_pix(&self) -> u64 {
        self.res.0 * self.res.1
    }

    /// Generate the nth ray.
    #[inline]
    #[must_use]
    pub fn gen_ray(&self, n: u64) -> Ray {
        debug_assert!(n < self.num_pix());

        let xi = n % self.res.1;
        let yi = n / self.res.1;
    }
}
