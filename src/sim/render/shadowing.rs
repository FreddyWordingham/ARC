//! Shadowing sub-module.

use crate::{geom::Ray, img::Shader, phys::laws, sim::render::Grid};
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
        ray.travel(hit.dist());

        match hit.group() {
            13..=15 => {
                ray.travel(shader.bump_dist());
                light *= shader.shadow_weights().transparency();
            }
            17..=18 => {
                ray.travel(shader.bump_dist());
                light *= shader.shadow_weights().transparency();
            }
            23..=25 => {
                *ray.dir_mut() = laws::reflect_dir(ray.dir(), hit.side().norm());
                ray.travel(shader.bump_dist());
            }
            _ => {
                light *= 0.1;
                break;
            }
        }
    }

    light * shader.shadow_weights().direct()
}
