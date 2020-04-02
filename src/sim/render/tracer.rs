//! Tracer implementation.

use crate::{access, geom::Ray};

/// Tracing ray.
#[derive(Debug, Clone)]
pub struct Tracer {
    ray: Ray,
    dist_travelled: f64,
}

impl Tracer {
    access!(ray, ray_mut, Ray);
    access!(dist_travelled, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(ray: Ray) -> Self {
        Self {
            ray,
            dist_travelled: 0.0,
        }
    }

    /// Move along the direction of travel a given distance recording the distance.
    #[inline]
    pub fn travel(&mut self, dist: f64) {
        debug_assert!(dist > 0.0);

        self.dist_travelled += dist;
        self.ray.travel(dist);
    }
}
