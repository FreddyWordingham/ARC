//! Trace trait.

use crate::geom::Ray;
use nalgebra::{Point3, Unit, Vector3};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Emit trait implementation.
/// Types implementing this trait can cast rays.
pub trait Emit {
    /// Cast a new ray from a random position on the surface with a corresponding normal direction.
    fn cast(&self, rng: &mut ThreadRng) -> Ray;
}

impl Emit for Point3<f64> {
    #[inline]
    #[must_use]
    fn cast(&self, rng: &mut ThreadRng) -> Ray {
        let theta = rng.gen_range(0.0, 2.0 * PI);
        let z = rng.gen_range(-1.0, 1.0);

        Ray::new(
            *self,
            Unit::new_normalize(Vector3::new(
                (1.0_f64 - (z * z)).sqrt() * theta.cos(),
                (1.0_f64 - (z * z)).sqrt() * theta.sin(),
                z,
            )),
        )
    }
}
