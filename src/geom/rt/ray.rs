//! Ray implementation.

use crate::{access, geom::Emit};
use nalgebra::{Point3, Rotation3, Unit, Vector3};
use rand::rngs::ThreadRng;

/// Ray structure.
#[derive(Debug, Clone)]
pub struct Ray {
    /// Ray origin.
    pos: Point3<f64>,
    /// Ray direction.
    dir: Unit<Vector3<f64>>,
}

impl Ray {
    access!(pos, Point3<f64>);
    access!(dir, dir_mut, Unit<Vector3<f64>>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(pos: Point3<f64>, dir: Unit<Vector3<f64>>) -> Self {
        Self { pos, dir }
    }

    /// Move along the direction of travel a given distance.
    #[inline]
    pub fn travel(&mut self, dist: f64) {
        self.pos += self.dir.as_ref() * dist;
    }

    /// Rotate the photon with a given pitch and subsequent roll manoeuvre.
    #[inline]
    pub fn rotate(&mut self, pitch: f64, roll: f64) {
        let arbitrary_axis = if (self.dir.z.abs() - 1.0) >= 1.0e-1 {
            Vector3::z_axis()
        } else {
            Vector3::y_axis()
        };

        let pitch_axis = Unit::new_normalize(self.dir.cross(&arbitrary_axis));
        let pitch_rot = Rotation3::from_axis_angle(&pitch_axis, pitch);

        let roll_rot = Rotation3::from_axis_angle(&self.dir, roll);

        self.dir = roll_rot * pitch_rot * self.dir;
        self.dir.renormalize();
    }
}

impl Emit for Ray {
    #[inline]
    #[must_use]
    fn cast(&self, _rng: &mut ThreadRng) -> Ray {
        self.clone()
    }
}
