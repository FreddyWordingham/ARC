//! Test painter.

use crate::{
    geom::Ray,
    img::Shader,
    phys::{laws::reflect_dir, Crossing},
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
    rng: &mut ThreadRng,
    mut weighting: f64,
) -> LinSrgba {
    debug_assert!(weighting > 0.0);

    let mut col = LinSrgba::default();
    while let Some(hit) = grid.observe(ray.clone(), shader.bump_dist()) {
        if weighting < 0.01 {
            break;
        }

        ray.travel(hit.dist());

        let light_dir = Unit::new_normalize(shader.sun_pos() - ray.pos());
        let view_dir = Unit::new_normalize(cam_pos - ray.pos());

        let light = lighting::ambient(shader)
            + lighting::diffuse(shader, hit.side().norm(), &light_dir)
            + lighting::specular(shader, hit.side().norm(), &light_dir, &view_dir);
        let shadow = shadowing::direct(grid, shader, ray.clone(), hit.side().norm());

        let x = light * shadow;

        match hit.group() {
            13..=15 => {
                col += scheme.get(hit.group()).get(x as f32)
                    * (weighting as f32)
                    * shader.shadow_weights().transparency() as f32;

                ray.travel(shader.bump_dist());
            }
            17..=18 => {
                col += scheme.get(hit.group()).get(x as f32) * (weighting as f32) * 0.1;

                let crossing = Crossing::new(ray.dir(), hit.side().norm(), 1.0, 1.1);
                if let Some(trans_dir) = crossing.trans_dir() {
                    let ref_prob = crossing.ref_prob();

                    let mut ref_ray = ray.clone();
                    *ref_ray.dir_mut() = reflect_dir(ray.dir(), hit.side().norm());
                    ref_ray.travel(shader.bump_dist());
                    col += paint(
                        cam_pos,
                        grid,
                        shader,
                        scheme,
                        ref_ray,
                        rng,
                        ref_prob * weighting,
                    );

                    weighting *= 1.0 - ref_prob;

                    *ray.dir_mut() = *trans_dir;
                    ray.travel(shader.bump_dist());
                } else {
                    *ray.dir_mut() = reflect_dir(ray.dir(), hit.side().norm());
                    ray.travel(shader.bump_dist());
                }
            }
            23..=25 => {
                col += scheme.get(hit.group()).get(x as f32) * (weighting as f32) * 0.1;

                *ray.dir_mut() = reflect_dir(ray.dir(), hit.side().norm());
                ray.travel(shader.bump_dist());
            }
            _ => {
                col += scheme.get(hit.group()).get(x as f32) * (weighting as f32);

                break;
            }
        }
    }

    col
}
