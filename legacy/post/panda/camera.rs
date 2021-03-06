//! Camera implementation.

use crate::{access, clone, geom::Ray, math::sample::golden};
use nalgebra::{Point3, Rotation3, Unit, Vector3};
use std::{
    f64::consts::PI,
    fmt::{Display, Formatter, Result},
};

/// Image forming camera.
pub struct Camera {
    /// Forward direction.
    forward: Ray,
    /// Up axis.
    up: Unit<Vector3<f64>>,
    /// Right axis.
    right: Unit<Vector3<f64>>,
    /// Target point.
    tar: Point3<f64>,
    /// Field of view.
    fov: (f64, f64),
    /// Image resolution.
    res: (usize, usize),
    /// Splits.
    splits: (usize, usize),
    /// Scanning deltas.
    delta: (f64, f64),
    /// Sub-sampling deltas.
    sub_delta: (f64, f64),
    /// Super sampling power.
    ss_power: i32,
    /// Depth of field samples.
    dof_samples: i32,
    /// Depth of field maximum jitter radius.
    dof_radius: f64,
    /// When true save each frame to a separate file.
    frame_saving: bool,
}

impl Camera {
    access!(forward, Ray);
    access!(up, Vector3<f64>);
    access!(right, Vector3<f64>);
    access!(tar, Point3<f64>);
    clone!(fov, (f64, f64));
    clone!(res, (usize, usize));
    clone!(splits, (usize, usize));
    clone!(delta, (f64, f64));
    clone!(sub_delta, (f64, f64));
    clone!(ss_power, i32);
    clone!(dof_samples, i32);
    clone!(dof_radius, f64);
    clone!(frame_saving, bool);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        pos: Point3<f64>,
        tar: Point3<f64>,
        fov_x: f64,
        res: (usize, usize),
        splits: (usize, usize),
        ss_power: i32,
        dof_samples: i32,
        dof_radius: f64,
        frame_saving: bool,
    ) -> Self {
        debug_assert!(fov_x > 0.0);
        debug_assert!(res.0 > 1);
        debug_assert!(res.1 > 1);
        debug_assert!(splits.0 > 0);
        debug_assert!(splits.1 > 0);
        debug_assert!(res.0 % splits.0 == 0);
        debug_assert!(res.1 % splits.1 == 0);
        debug_assert!(ss_power > 0);
        debug_assert!(dof_samples > 0);
        debug_assert!(dof_radius > 0.0);

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
            tar,
            fov,
            res,
            splits,
            delta,
            sub_delta,
            ss_power,
            dof_samples,
            dof_radius,
            frame_saving,
        }
    }

    /// Calculate the total number of pixels forming the image.
    #[inline]
    #[must_use]
    pub fn num_pix(&self) -> usize {
        self.res.0 * self.res.1
    }

    /// Calculate the sub-frame resolution.
    #[inline]
    #[must_use]
    pub fn frame_res(&self) -> (usize, usize) {
        (self.res.0 / self.splits.0, self.res.1 / self.splits.1)
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
    pub fn gen_ss_ray(&self, xi: usize, yi: usize, sample: i32) -> Ray {
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

    /// Generate a super-sampling depth-of-field ray for the corresponding pixel indices.
    #[inline]
    #[must_use]
    pub fn gen_ss_dof_ray(
        &self,
        xi: usize,
        yi: usize,
        sub_sample: i32,
        depth_sample: i32,
        offset: f64,
    ) -> Ray {
        debug_assert!(xi < self.res.0);
        debug_assert!(yi < self.res.1);
        debug_assert!(sub_sample < self.ss_power.pow(2));
        debug_assert!(offset >= 0.0);
        debug_assert!(offset <= (2.0 * PI));

        let mut theta = (xi as f64 * self.delta.0) - (self.fov.0 * 0.5);
        let mut phi = (yi as f64 * self.delta.1) - (self.fov.1 * 0.5);

        let sx = (sub_sample % self.ss_power) as f64 + 0.5;
        let sy = (sub_sample / self.ss_power) as f64 + 0.5;
        theta += (sx * self.sub_delta.0) - (self.delta.0 * 0.5);
        phi += (sy * self.sub_delta.1) - (self.delta.1 * 0.5);

        let (r, t) = golden::circle(depth_sample, self.dof_samples);
        let mut pos = self.forward.pos().clone();
        pos += self.right.as_ref() * (r * (t + offset).sin() * self.dof_radius);
        pos += self.up.as_ref() * (r * (t + offset).cos() * self.dof_radius);

        let forward = Unit::new_normalize(self.tar - pos);
        let up = Vector3::z_axis();
        let right = Unit::new_normalize(forward.cross(&up));

        let mut ray = Ray::new(pos, forward);
        *ray.dir_mut() = Rotation3::from_axis_angle(&up, theta)
            * Rotation3::from_axis_angle(&right, phi)
            * ray.dir();

        ray
    }
}

impl Display for Camera {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt)?;
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
            (self.ss_power.pow(2) * (self.res.0 * self.res.1) as i32) as f64 / 1e6
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
