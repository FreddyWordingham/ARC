//! Shadowing sub-module.

use crate::{geom::Ray, img::Shader, sim::render::Grid};
use nalgebra::{Unit, Vector3};

/// Calculate the direct shadowing factor.
#[allow(clippy::single_match_else)]
#[inline]
#[must_use]
pub fn direct(grid: &Grid, shader: &Shader, mut ray: Ray, norm: &Unit<Vector3<f64>>) -> f64 {
    *ray.dir_mut() = *norm;
    ray.travel(shader.bump_dist());

    *ray.dir_mut() = Unit::new_normalize(shader.sun_pos() - ray.pos());

    let mut light = 1.0;
    while let Some(hit) = grid.observe(ray.clone(), shader.bump_dist()) {
        ray.travel(hit.dist() + shader.bump_dist());

        match hit.group() {
            1 => {
                return 0.0;
            }
            _ => {
                light *= shader.shadow_weights().direct();
            }
        }
    }

    light
}