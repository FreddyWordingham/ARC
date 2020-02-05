//! Grid implementation.

use crate::{access, geom::Aabb};

// /// Material detection rays must be aimed at a triangle with at least this deviation from the triangle's plane.
// const HIT_ANGLE_THRESHOLD: f64 = 1.0e-3;

/// Grid partition scheme.
pub struct Grid {
    /// Boundary.
    bound: Aabb,
}

impl Grid {
    access!(bound, Aabb);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(bound: Aabb) -> Self {
        Self { bound }
    }
}
