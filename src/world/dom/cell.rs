//! Grid implementation.

use crate::{
    access,
    geom::{Aabb, Collide, Ray, SmoothTriangle, Trace},
    ord::{InterKey, MatKey, StateKey},
    world::{Interface, Verse},
};

// /// Material detection rays must be aimed at a triangle with at least this deviation from the triangle's plane.
// const HIT_ANGLE_THRESHOLD: f64 = 1.0e-3;

/// Cell holding local information.
pub struct Cell<'a> {
    /// Boundary.
    bound: Aabb,
    /// Central material.
    mat: MatKey,
    /// Initial state.
    state: StateKey,
    /// Intersecting interface triangles.
    inter_tris: Vec<((&'a InterKey, &'a Interface), Vec<&'a SmoothTriangle>)>,
}

impl<'a> Cell<'a> {
    access!(bound, Aabb);
    access!(mat, MatKey);
    access!(state, StateKey);
    access!(
        inter_tris,
        Vec<((&'a InterKey, &'a Interface), Vec<&'a SmoothTriangle>)>
    );

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(bound: Aabb, mat: MatKey, state: StateKey, verse: &'a Verse) -> Self {
        let mut inter_tris = Vec::new();

        for (key, inter) in verse.inters().map() {
            let surf = verse.surfs().get(inter.surf());
            if bound.overlap(surf.aabb()) {
                let mut intersections = Vec::new();

                for tri in surf.tris().iter().filter(|tri| tri.overlap(&bound)) {
                    intersections.push(tri);
                }

                if !intersections.is_empty() {
                    inter_tris.push(((key, inter), intersections));
                }
            }
        }

        Self {
            bound,
            mat,
            state,
            inter_tris,
        }
    }

    /// Determine the distance to the next interface along a ray's line of sight.
    #[inline]
    #[must_use]
    pub fn inter_dist(&self, ray: &Ray) -> Option<f64> {
        assert!(self.bound().contains(ray.pos()));

        let mut nearest = None;
        for ((_name, _inter), tris) in &self.inter_tris {
            for tri in tris {
                if let Some(dist) = tri.dist(ray) {
                    if nearest.is_none() || dist < nearest.expect("Something went wrong...") {
                        nearest = Some(dist);
                    }
                }
            }
        }

        nearest
    }
}
