//! Camera implementation.

use crate::{access, clone, rend::AspectRatio};
use nalgebra::{Point3, Unit, Vector3};

/// Image building structure.
pub struct Camera {
    /// Position.
    pos: Point3<f64>,
    /// Target point.
    tar: Point3<f64>,
    /// Forward direction.
    forward: Unit<Vector3<f64>>,
    /// Field of view.
    fov: (f64, f64),
    /// Image resolution.
    res: (usize, usize),
}

impl Camera {
    access!(pos, Point3<f64>);
    access!(tar, Point3<f64>);
    access!(forward, Unit<Vector3<f64>>);
    clone!(fov, (f64, f64));
    clone!(res, (usize, usize));

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        pos: Point3<f64>,
        tar: Point3<f64>,
        fov_hz: f64,
        aspect_ratio: AspectRatio,
        tar_pix: usize,
    ) -> Self {
        debug_assert!(fov_hz > 0.0);
        debug_assert!(tar_pix > 0);

        let fov = (fov_hz, fov_hz / aspect_ratio.ratio());
        let res = aspect_ratio.resolution(tar_pix);

        Self {
            pos,
            tar,
            forward: Unit::new_normalize(tar - pos),
            fov,
            res,
        }
    }

    /// Calculate the number of pixels in the final image.
    #[inline]
    #[must_use]
    pub fn total_pixels(&self) -> usize {
        self.res.0 * self.res.1
    }
}
