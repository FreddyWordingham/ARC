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
    access!(pos, pos_mut, Point3<f64>);
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
        debug_assert!(dist > 0.0);

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

    /// Reflect the ray from a given normal surface.
    #[inline]
    pub fn reflect(&mut self, norm: &Unit<Vector3<f64>>) {
        self.dir =
            Unit::new_normalize(self.dir.as_ref() + (2.0 * (-self.dir.dot(norm)) * norm.as_ref()));
    }

    /// Refract the ray from a given normal surface from a given refracting index to another.
    #[inline]
    pub fn refract(&mut self, norm: &Unit<Vector3<f64>>, n0: f64, n1: f64) {
        // TODO: Check for critical angle.
        let inc = self.dir();
        let ci = -inc.dot(&norm);

        let n = n0 / n1;
        let s2t = n.powi(2) * (1.0 - ci.powi(2));
        let ct = (1.0 - s2t).sqrt();

        self.dir =
            Unit::new_unchecked((n * inc.into_inner()) + ((n * ci) - ct) * norm.into_inner());
    }
}

impl Emit for Ray {
    #[inline]
    #[must_use]
    fn cast(&self, _rng: &mut ThreadRng) -> Ray {
        self.clone()
    }
}
