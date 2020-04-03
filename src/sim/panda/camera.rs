//! Camera implementation.

use crate::{access, clone, geom::Ray};
use nalgebra::{Point3, Rotation3, Unit, Vector3};
use std::fmt::{Display, Formatter, Result};

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
    /// Sub-sampling deltas.
    sub_delta: (f64, f64),
    /// Super sampling power.
    ss_power: usize,
}

impl Camera {
    access!(forward, Ray);
    access!(up, Vector3<f64>);
    access!(right, Vector3<f64>);
    clone!(fov, (f64, f64));
    clone!(res, (usize, usize));
    clone!(delta, (f64, f64));
    clone!(sub_delta, (f64, f64));
    clone!(ss_power, usize);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        pos: Point3<f64>,
        tar: Point3<f64>,
        fov_x: f64,
        res: (usize, usize),
        ss_power: usize,
    ) -> Self {
        debug_assert!(fov_x > 0.0);
        debug_assert!(res.0 > 1);
        debug_assert!(res.1 > 1);
        debug_assert!(ss_power > 0);

        let fov = (fov_x, fov_x * (res.1 as f64 / res.0 as f64));
        let delta = (fov.0 / (res.0 - 1) as f64, fov.1 / (res.1 - 1) as f64);
        let sub_delta = (delta.0 / ss_power as f64, delta.1 / ss_power as f64);

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
            sub_delta,
            ss_power,
        }
    }

    /// Calculate the total number of pixels forming the image.
    #[inline]
    #[must_use]
    pub fn num_pix(&self) -> usize {
        self.res.0 * self.res.1
    }

    /// Generate the a ray for the corresponding pixel indices.
    #[inline]
    #[must_use]
    pub fn gen_ray(&self, xi: usize, yi: usize) -> Ray {
        debug_assert!(xi < self.res.0);
        debug_assert!(yi < self.res.1);

        let theta = (xi as f64 * self.delta.0) - (self.fov.0 / 2.0);
        let phi = (yi as f64 * self.delta.1) - (self.fov.1 / 2.0);

        let mut ray = self.forward.clone();

        *ray.dir_mut() = Rotation3::from_axis_angle(&self.up, theta)
            * Rotation3::from_axis_angle(&self.right, phi)
            * ray.dir();

        ray
    }

    /// Generate a super-sampling ray for the corresponding pixel indices.
    #[inline]
    #[must_use]
    pub fn gen_ss_ray(&self, xi: usize, yi: usize, sample: usize) -> Ray {
        debug_assert!(xi < self.res.0);
        debug_assert!(yi < self.res.1);
        debug_assert!(sample < self.ss_power.pow(2));

        let mut theta = (xi as f64 * self.delta.0) - (self.fov.0 * 0.5);
        let mut phi = (yi as f64 * self.delta.1) - (self.fov.1 * 0.5);

        let sx = (sample % self.ss_power) as f64 + 0.5;
        let sy = (sample / self.ss_power) as f64 + 0.5;
        theta += (sx * self.sub_delta.0) - (self.delta.0 * 0.5);
        phi += (sy * self.sub_delta.1) - (self.delta.1 * 0.5);

        let mut ray = self.forward.clone();

        *ray.dir_mut() = Rotation3::from_axis_angle(&self.up, theta)
            * Rotation3::from_axis_angle(&self.right, phi)
            * ray.dir();

        ray
    }
}

impl Display for Camera {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt, "")?;
        writeln!(fmt, "{:>30} : {}", "position", self.forward.pos())?;
        writeln!(
            fmt,
            "{:>30} : {{{:.3}, {:.3}, {:.3}}}",
            "forward direction",
            self.forward.dir().as_ref().x,
            self.forward.dir().as_ref().y,
            self.forward.dir().as_ref().z
        )?;
        writeln!(
            fmt,
            "{:>30} : {{{:.3}, {:.3}, {:.3}}}",
            "up direction",
            self.up.as_ref().x,
            self.up.as_ref().y,
            self.up.as_ref().z
        )?;
        writeln!(
            fmt,
            "{:>30} : {{{:.3}, {:.3}, {:.3}}}",
            "right direction",
            self.right.as_ref().x,
            self.right.as_ref().y,
            self.right.as_ref().z
        )?;
        writeln!(
            fmt,
            "{:>30} : {} x {}",
            "field of view (hr, vt) [deg]",
            self.fov.0.to_degrees(),
            self.fov.1.to_degrees()
        )?;
        writeln!(
            fmt,
            "{:>30} : {} x {}",
            "resolution (hr, vt)", self.res.0, self.res.1
        )?;
        writeln!(
            fmt,
            "{:>30} : {} [x10^6]",
            "total pixels",
            (self.res.0 * self.res.1) as f64 / 1e6
        )?;
        writeln!(fmt, "{:>30} : {}", "super sampling power", self.ss_power)?;
        writeln!(fmt, "{:>30} : {}", "sub-samples", self.ss_power.pow(2))?;
        writeln!(
            fmt,
            "{:>30} : {} [x10^6]",
            "total samples",
            (self.ss_power.pow(2) * (self.res.0 * self.res.1)) as f64 / 1e6
        )
        // /// Field of view.
        // fov: (f64, f64),
        // /// Image resolution.
        // res: (usize, usize),
        // /// Scanning deltas.
        // delta: (f64, f64),
        // /// Sub-sampling deltas.
        // sub_delta: (f64, f64),
        // /// Super sampling power.
        // ss_power: u64,
    }
}
