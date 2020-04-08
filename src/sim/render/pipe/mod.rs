//! Rendering pipe sub-module.

use crate::{
    geom::Ray,
    // geom::{Ray, Trace},
    img::Shader,
    sim::render::{lighting, Grid, Scheme},
};
use nalgebra::Point3;
use palette::{Gradient, LinSrgba};
use rand::rngs::ThreadRng;

/// Determine the colour of a given ray.
#[inline]
#[must_use]
pub fn colour(
    _cam_pos: &Point3<f64>,
    grid: &Grid,
    shader: &Shader,
    scheme: &Scheme,
    mut ray: Ray,
    _rng: &mut ThreadRng,
) -> LinSrgba {
    debug_assert!(shader.bump_dist() > 0.0);

    let backup = Gradient::new(vec![
        LinSrgba::new(1.0, 1.0, 0.0, 1.0),
        LinSrgba::new(0.0, 1.0, 1.0, 1.0),
    ]);
    let grad_0 = if scheme.grads().contains_key(&0) {
        &scheme.grads().get(&0).expect("Invalid gradient group.")
    } else {
        &backup
    };
    let grad_1 = if scheme.grads().contains_key(&1) {
        &scheme.grads().get(&1).expect("Invalid gradient group.")
    } else {
        &backup
    };

    let mut col = LinSrgba::default();
    while let Some(hit) = grid.observe(ray.clone(), shader.bump_dist()) {
        ray.travel(hit.dist() + shader.bump_dist());

        match hit.group() {
            0 => {
                let x = lighting::ambient(shader) + lighting::diffuse(shader, &ray, hit.norm());
                return col + grad_0.get(x as f32);
            }
            1 => {
                let x = lighting::ambient(shader) + lighting::diffuse(shader, &ray, hit.norm());
                col += grad_1.get(x as f32);
            }
            _ => {
                panic!("Do not know how to handle group: {}", hit.group());
            }
        }
    }

    col
}
