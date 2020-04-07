//! Rendering pipe sub-module.

use crate::{
    geom::Ray,
    // geom::{Ray, Trace},
    img::Shader,
    sim::render::{lighting, Grid, Scheme},
};
use nalgebra::Point3;
use palette::{Gradient, LinSrgba, Srgba};
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
    bump_dist: f64,
    _rng: &mut ThreadRng,
) -> LinSrgba {
    debug_assert!(bump_dist > 0.0);

    let backup = Gradient::new(vec![
        LinSrgba::new(1.0, 1.0, 0.0, 1.0),
        LinSrgba::new(0.0, 1.0, 1.0, 1.0),
    ]);
    let grad_0 = if scheme.grads().contains_key(&0) {
        &scheme.grads().get(&0).expect("Invalid gradient group.")
    } else {
        &backup
    };

    if let Some(hit) = grid.observe(ray.clone(), bump_dist) {
        ray.travel(hit.dist() + bump_dist);
        let x = lighting::ambient(shader) + lighting::diffuse(shader, &ray, hit.norm());

        return LinSrgba::from(grad_0.get(x as f32));
    }

    Srgba::new(0.2, 0.2, 0.2, 0.2).into_linear()
}
