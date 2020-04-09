//! Test painter.

use crate::{
    geom::Ray,
    img::Shader,
    phys::laws::reflect_dir,
    sim::render::{lighting, shadowing, Grid, Scheme},
};
use nalgebra::{Point3, Unit};
// use palette::{Gradient, LinSrgba};
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Paint the ray if it hits something.
#[allow(clippy::never_loop)]
#[allow(clippy::single_match_else)]
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
        ray.travel(hit.dist());

        let light_dir = Unit::new_normalize(shader.sun_pos() - ray.pos());
        let view_dir = Unit::new_normalize(cam_pos - ray.pos());

        let light = lighting::ambient(shader)
            + lighting::diffuse(shader, hit.side().norm(), &light_dir)
            + lighting::specular(shader, hit.side().norm(), &light_dir, &view_dir);
        let shadow = shadowing::direct(grid, shader, ray.clone(), hit.side().norm());

        let x = light * shadow;

        match hit.group() {
            23..=25 => {
                *ray.dir_mut() = reflect_dir(ray.dir(), hit.side().norm());
                ray.travel(shader.bump_dist());

                col += scheme.get(hit.group()).get(x as f32) * 0.1;
            }
            _ => {
                col += scheme.get(hit.group()).get(x as f32);
                break;
            }
        }
    }

    col
}
