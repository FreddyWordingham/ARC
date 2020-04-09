//! Lighting only painter.

use crate::{
    geom::Ray,
    img::Shader,
    sim::render::{lighting, Grid, Scheme},
};
use nalgebra::Point3;
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Paint the ray with the luminance.
#[inline]
#[must_use]
pub fn paint(
    _cam_pos: &Point3<f64>,
    grid: &Grid,
    shader: &Shader,
    scheme: &Scheme,
    mut ray: Ray,
    _rng: &mut ThreadRng,
) -> LinSrgba {
    debug_assert!(shader.bump_dist() > 0.0);

    let mut col = LinSrgba::default();
    while let Some(hit) = grid.observe(ray.clone(), shader.bump_dist()) {
        ray.travel(hit.dist() + shader.bump_dist());

        match hit.group() {
            0 => {
                let x = lighting::ambient(shader) + lighting::diffuse(shader, &ray, hit.norm());
                return col + scheme.get(0).get(x as f32);
            }
            1 => {
                let x = lighting::ambient(shader) + lighting::diffuse(shader, &ray, hit.norm());
                col += scheme.get(1).get(x as f32);
            }
            _ => {
                panic!("Do not know how to handle group: {}", hit.group());
            }
        }
    }

    col
}
