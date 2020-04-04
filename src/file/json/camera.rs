//! Camera implementation.

use crate::{access, clone, sim::panda::Camera as SimCamera};
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
    /// Splits.
    splits: (usize, usize),
    /// Super sampling power.
    ss_power: usize,
    /// When true save each frame to a separate file.
    frame_saving: bool,
}

impl Camera {
    access!(pos, Point3<f64>);
    access!(tar, Point3<f64>);
    clone!(fov, f64);
    clone!(res, (usize, usize));
    clone!(splits, (usize, usize));
    clone!(ss_power, usize);
    clone!(frame_saving, bool);

    /// Build a panda simulation camera.
    #[inline]
    #[must_use]
    pub fn build(&self) -> SimCamera {
        crate::sim::panda::Camera::new(
            self.pos,
            self.tar,
            self.fov.to_radians(),
            self.res,
            self.splits,
            self.ss_power,
            self.frame_saving,
        )
    }

    /// Build a zebra simulation camera.
    #[inline]
    #[must_use]
    pub fn build_zebra_cam(&self) -> crate::sim::render::Camera {
        crate::sim::render::Camera::new(
            self.pos,
            self.tar,
            self.fov.to_radians(),
            self.res,
            self.ss_power as u64,
        )
    }
}

impl Display for Camera {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt)?;
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
