//! Tracer implementation.

use crate::{access, geom::Ray};
use attr::json;

/// Tracing ray.
pub struct Tracer {
    ray: Ray,
}

impl Tracer {
    access!(ray, ray_mut, Ray);
}
