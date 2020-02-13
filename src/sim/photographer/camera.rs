//! Camera implementation.

use crate::{access, clone, geom::Ray};
use nalgebra::{Point3, Unit, Vector3};

/// Image forming camera.
pub struct Camera {
    /// Camera viewing position.
    pos: Point3<f64>,
    /// Camera viewing direction.
    dir: Unit<Vector3<f64>>,
    /// Field of view.
    fov: (f64, f64),
    /// Image resolution.
    res: (usize, usize),
}

impl Camera {
    access!(pos, Point3<f64>);
    access!(dir, Unit<Vector3<f64>>);
    clone!(fov, (f64, f64));
    clone!(res, (usize, usize));

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(pos: Point3<f64>, tar: Point3<f64>, fov_x: f64, res: (usize, usize)) -> Self {
        debug_assert!(fov_x > 0.0);
        debug_assert!(res.0 > 0);
        debug_assert!(res.1 > 0);

        Self {
            pos,
            dir: Unit::new_normalize(tar - pos),
            fov: (fov_x, fov_x * (res.1 as f64 / res.0 as f64)),
            res,
        }
    }

    /// Calculate the total number of pixels forming the image.
    #[inline]
    #[must_use]
    pub fn num_pix(&self) -> usize {
        self.res.0 * self.res.1
    }

    /// Generate the nth ray.
    #[inline]
    #[must_use]
    pub fn gen_ray(&self, n: usize) -> Ray {
        debug_assert!(n < self.num_pix());

        let xi = n % self.res.0;
        let yi = n / self.res.0;

        Ray::new(Point3::origin(), Vector3::z_axis())
    }
}
