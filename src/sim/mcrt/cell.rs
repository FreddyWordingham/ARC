//! Grid implementation.

use crate::{
    access,
    geom::{Aabb, Collide, SmoothTriangle},
    ord::{InterKey, InterSet, MatKey, SurfSet},
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

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(bound: Aabb, mat: MatKey, inters: &InterSet, surfs: &SurfSet) -> Self {
        let mut inter_tris = Vec::new();

        for (key, inter) in inters.map() {
            let surf = surfs.get(inter.surf());
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
            inter_tris: Vec::new(),
        }
    }
}
