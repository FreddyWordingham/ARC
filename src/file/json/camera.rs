//! Camera implementation.

use crate::{access, clone, sim::render::Camera as SimCam};
use attr::json;
use nalgebra::Point3;
use std::fmt::{Display, Formatter, Result};

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
    /// Super sampling power.
    ss_power: u64,
}

impl Camera {
    access!(pos, Point3<f64>);
    access!(tar, Point3<f64>);
    clone!(fov, f64);
    clone!(res, (usize, usize));
    clone!(ss_power, u64);

    /// Build a simulation camera.
    #[inline]
    #[must_use]
    pub fn build(&self) -> SimCam {
        SimCam::new(
            self.pos,
            self.tar,
            self.fov.to_radians(),
            self.res,
            self.ss_power,
        )
    }
}

impl Display for Camera {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt, "")?;
        writeln!(fmt, "{:>30} : {}", "position", self.pos)?;
        writeln!(fmt, "{:>30} : {}", "target", self.tar)?;
        writeln!(fmt, "{:>30} : {} [deg]", "field of view", self.fov)?;
        writeln!(
            fmt,
            "{:>30} : {} x {}",
            "image resolution", self.res.0, self.res.1
        )?;
        writeln!(fmt, "{:>30} : {}", "super sampling power", self.ss_power)
    }
}
