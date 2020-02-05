//! Rectangle implementation.

use crate::{
    access,
    geom::{Emit, Ray},
};
use std::f64::consts::FRAC_PI_2;

/// Aperture geometry.
#[derive(Debug, Clone)]
pub struct Aperture {
    /// Ray.
    ray: Ray,
    /// Numerical aperture.
    na: f64,
}

impl Aperture {
    access!(ray, Ray);
    access!(na, f64);

    /// Construct a new instance.
    pub fn new(ray: Ray, na: f64) -> Self {
        assert!(na > 0.0);
        assert!(na < FRAC_PI_2);

        Self { ray, na }
    }
}

impl Emit for Aperture {
    fn cast(&self, rng: &mut ThreadRng) -> Ray {
        let pitch = rng.gen_range(self.na.cos(), 1.0).acos();
        let roll = rng.gen_range(0.0, 2.0 * PI);

        let mut ray = self.ray.clone();
        ray.rotate(pitch, roll);

        ray
    }
}
