//! Tracer implementation.

use crate::{access, geom::Ray};

/// Ray tracer.
pub struct Tracer {
    /// Ray of travel.
    ray: Ray,
    /// Total distance travelled.
    dist_travelled: f64,
}

impl Tracer {
    access!(ray, ray_mut, Ray);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(ray: Ray) -> Self {
        Self {
            ray,
            dist_travelled: 0.0,
        }
    }

    /// Move along the direction of travel a given distance.
    #[inline]
    pub fn travel(&mut self, dist: f64) {
        debug_assert!(dist > 0.0);

        self.ray.travel(dist);
        self.dist_travelled += dist;
    }

    /// Get the distance travelled and reset the counter.
    #[inline]
    #[must_use]
    pub fn dist_travelled(&mut self) -> f64 {
        let d = self.dist_travelled;
        self.dist_travelled = 0.0;
        d
    }
}
