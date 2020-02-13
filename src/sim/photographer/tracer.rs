//! Tracer implementation.

use crate::{access, clone, geom::Ray};

/// Ray tracer.
pub struct Tracer {
    /// Ray of travel.
    ray: Ray,
    /// Total distance travelled.
    dist_travelled: f64,
}

impl Tracer {
    access!(ray, Ray);
    clone!(dist_travelled, f64);

    /// Construct a new instance.
    pub fn new(ray: Ray) -> Self {
        Self {
            ray,
            dist_travelled: 0.0,
        }
    }
}
