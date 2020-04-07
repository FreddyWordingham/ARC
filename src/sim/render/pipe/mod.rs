//! Rendering pipe sub-module.

use crate::{geom::Ray, img::Shader, sim::render::Grid};
use nalgebra::Point3;
use palette::Gradient;
use palette::{LinSrgba, Srgba};
use rand::rngs::ThreadRng;

/// Determine the colour of a given ray.
#[inline]
#[must_use]
pub fn colour(
    _cam_pos: &Point3<f64>,
    grid: &Grid,
    _shader: &Shader,
    mut ray: Ray,
    bump_dist: f64,
    _rng: &mut ThreadRng,
) -> LinSrgba {
    debug_assert!(bump_dist > 0.0);

    let grad_0 = Gradient::new(vec![
        LinSrgba::new(0.0, 0.0, 0.0, 1.0),
        LinSrgba::new(1.0, 1.0, 1.0, 1.0),
    ]);

    println!("> 0");

    while let Some(hit) = grid.observe(ray.clone(), bump_dist) {
        println!("> 1");
        ray.travel(hit.dist() + bump_dist);

        match hit.group() {
            _ => {
                println!("> 2");
                return LinSrgba::from(grad_0.get(1.0));
            }
        }
    }

    println!("> 3");
    Srgba::new(0.2, 0.2, 0.2, 0.2).into_linear()
}
