//! Camera implementation.

use crate::{access, clone, geom::Ray};
use nalgebra::{Point3, Rotation3, Unit, Vector3};

/// Image forming camera.
pub struct Camera {
    /// Forward direction.
    forward: Ray,
    /// Up axis.
    up: Unit<Vector3<f64>>,
    /// Right axis.
    right: Unit<Vector3<f64>>,
    /// Field of view.
    fov: (f64, f64),
    /// Image resolution.
    res: (usize, usize),
    /// Scanning deltas.
    delta: (f64, f64),
}

impl Camera {
    access!(forward, Ray);
    access!(up, Vector3<f64>);
    access!(right, Vector3<f64>);
    clone!(fov, (f64, f64));
    clone!(res, (usize, usize));
    clone!(delta, (f64, f64));

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(pos: Point3<f64>, tar: Point3<f64>, fov_x: f64, res: (usize, usize)) -> Self {
        debug_assert!(fov_x > 0.0);
        debug_assert!(res.0 > 1);
        debug_assert!(res.1 > 1);

        let fov = (fov_x, fov_x * (res.1 as f64 / res.0 as f64));
        let delta = (fov.0 / (res.0 - 1) as f64, fov.1 / (res.1 - 1) as f64);

        let forward = Ray::new(pos, Unit::new_normalize(tar - pos));
        let up = Vector3::z_axis();
        let right = Unit::new_normalize(forward.dir().cross(&up));

        Self {
            forward,
            up,
            right,
            fov,
            res,
            delta,
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

        let theta = (xi as f64 * self.delta.0) - (self.fov.0 / 2.0);
        let phi = (yi as f64 * self.delta.1) - (self.fov.1 / 2.0);

        let mut ray = self.forward.clone();

        *ray.dir_mut() = Rotation3::from_axis_angle(&self.up, theta)
            * Rotation3::from_axis_angle(&self.right, phi)
            * ray.dir();

        ray
    }
}
