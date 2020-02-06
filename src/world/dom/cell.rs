//! Grid implementation.

use crate::{
    access,
    geom::Aabb,
    ord::{MatKey, StateKey},
};

// /// Material detection rays must be aimed at a triangle with at least this deviation from the triangle's plane.
// const HIT_ANGLE_THRESHOLD: f64 = 1.0e-3;

/// Cell holding local information.
pub struct Cell {
    /// Boundary.
    bound: Aabb,
    /// Central material.
    mat: MatKey,
    /// Initial state.
    state: StateKey,
}

impl Cell {
    access!(bound, Aabb);
    access!(mat, MatKey);
    access!(state, StateKey);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(bound: Aabb, mat: MatKey, state: StateKey) -> Self {
        Self { bound, mat, state }
    }
}
