//! Grid implementation.

use crate::{
    access,
    geom::{Aabb, SmoothTriangle},
    ord::{InterKey, MatKey},
    world::Interface,
};

/// Cell holding local information.
pub struct Cell<'a> {
    /// Boundary.
    bound: Aabb,
    /// Central material.
    mat: MatKey,
    /// Intersecting interface triangles.
    inter_tris: Vec<((&'a InterKey, &'a Interface), Vec<&'a SmoothTriangle>)>,
}

impl<'a> Cell<'a> {
    access!(bound, Aabb);
    access!(mat, MatKey);
    access!(
        inter_tris,
        Vec<((&'a InterKey, &'a Interface), Vec<&'a SmoothTriangle>)>
    );

    pub fn new(bound: Aabb, mat: MatKey) -> Self {
        Self {
            bound,
            mat,
            inter_tris: Vec::new(),
        }
    }
}
