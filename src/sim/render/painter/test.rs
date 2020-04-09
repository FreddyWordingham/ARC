//! Test painter.

use crate::{
    geom::Ray,
    img::Shader,
    sim::render::{lighting, Grid, Scheme},
};
use nalgebra::{Point3, Unit};
// use palette::{Gradient, LinSrgba};
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Paint the ray if it hits something.
#[inline]
#[must_use]
pub fn paint(
    cam_pos: &Point3<f64>,
    grid: &Grid,
    shader: &Shader,
    scheme: &Scheme,
    mut ray: Ray,
    _rng: &mut ThreadRng,
) -> LinSrgba {
    let mut col = LinSrgba::default();
    while let Some(hit) = grid.observe(ray.clone(), shader.bump_dist()) {
        ray.travel(hit.dist() + shader.bump_dist());

        match hit.group() {
            99 => {
                break;
            }
            _ => {
                let light_dir = Unit::new_normalize(shader.sun_pos() - ray.pos());
                let view_dir = Unit::new_normalize(cam_pos - ray.pos());

                let x = lighting::ambient(&shader)
                    + lighting::diffuse(&shader, &ray, hit.side().norm(), &light_dir)
                    + lighting::specular(&shader, hit.side().norm(), &light_dir, &view_dir);

                col += scheme.get(hit.group()).get(x as f32);
                break;
            }
        }
    }

    col
}
