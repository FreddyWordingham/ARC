//! Arctic rendering painter pipeline.

use crate::{
    geom::Ray,
    img::Shader,
    phys::{laws::reflect_dir, Crossing},
    sim::render::{Grid, Scheme},
};
use nalgebra::{Point3, Unit, Vector3};
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Minimum fragment weight to simulate.
const MIN_WEIGHT: f64 = 0.01;

/// Mirror colouring fraction.
const MIRROR_COLOURING: f32 = 0.15;

/// Refraction colouring fraction.
const REFRACTION_COLOURING: f32 = 0.20;

/// Puddle reflection shimmer factor.
const PUDDLE_SHIMMER: f64 = 12.0;

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
    weight: f64,
) -> LinSrgba {
    debug_assert!(shader.bump_dist() > 0.0);
    debug_assert!(weight > 0.0);

    let mut col = LinSrgba::default();

    if weight <= MIN_WEIGHT {
        return col;
    }

    while let Some(hit) = grid.observe(ray.clone(), shader.bump_dist()) {
        ray.travel(hit.dist());

        let light = light(cam_pos, shader, &ray, hit.side().norm());
        let shadow = shadow(grid, shader, &ray, hit.side().norm());
        let illumination = light * shadow;

        match hit.group() {
            1 => {
                // Water
                col += scheme.get(hit.group()).get(illumination as f32) * MIRROR_COLOURING;

                *ray.dir_mut() = reflect_dir(ray.dir(), hit.side().norm());
                let theta = ((ray.pos().x * PUDDLE_SHIMMER).sin().powi(2)
                    * (ray.pos().y * PUDDLE_SHIMMER).sin().powi(2))
                    * 0.5e-1;
                let rot = nalgebra::Rotation3::from_axis_angle(&nalgebra::Vector3::y_axis(), theta);
                *ray.dir_mut() = nalgebra::Unit::new_normalize(rot * ray.dir().as_ref());
                ray.travel(shader.bump_dist());
            }
            11 => {
                // Rocks 1
                col += scheme.get(hit.group()).get(illumination as f32) * REFRACTION_COLOURING;

                let crossing = Crossing::new(ray.dir(), hit.side().norm(), 1.0, 1.1);
                if let Some(trans_dir) = crossing.trans_dir() {
                    let ref_prob = crossing.ref_prob();
                    let trans_prob = 1.0 - ref_prob;

                    let mut ref_ray =
                        Ray::new(ray.pos().clone(), reflect_dir(ray.dir(), hit.side().norm()));
                    ref_ray.travel(shader.bump_dist());
                    let ref_col = paint(
                        cam_pos,
                        grid,
                        shader,
                        scheme,
                        ref_ray,
                        rng,
                        weight * ref_prob,
                    );

                    let mut trans_ray = ray;
                    *trans_ray.dir_mut() = *trans_dir;
                    trans_ray.travel(shader.bump_dist());
                    let trans_col = paint(
                        cam_pos,
                        grid,
                        shader,
                        scheme,
                        trans_ray,
                        rng,
                        weight * trans_prob,
                    );

                    return col + (ref_col * ref_prob as f32) + (trans_col * trans_prob as f32);
                } else {
                    *ray.dir_mut() = reflect_dir(ray.dir(), hit.side().norm());
                    ray.travel(shader.bump_dist());
                }
            }
            _ => {
                col += scheme.get(hit.group()).get(illumination as f32);
                break;
            }
        }
    }

    col
}

/// Calculate the lighting. 0.0 = Complete darkness. 1.0 = Full brightness.
#[inline]
#[must_use]
fn light(cam_pos: &Point3<f64>, shader: &Shader, ray: &Ray, norm: &Unit<Vector3<f64>>) -> f64 {
    let light_dir = Unit::new_normalize(shader.sun_pos() - ray.pos());
    let view_dir = Unit::new_normalize(cam_pos - ray.pos());
    let ref_dir = reflect_dir(&Unit::new_normalize(-light_dir.as_ref()), norm);

    let mut ambient = 1.0;
    let mut diffuse = norm.dot(&light_dir).max(0.0);
    let mut specular = view_dir
        .dot(&ref_dir)
        .max(0.0)
        .powi(shader.light_weights().specular_power());

    ambient *= shader.light_weights().ambient();
    diffuse *= shader.light_weights().diffuse();
    specular *= shader.light_weights().specular();

    ambient + diffuse + specular
}

/// Calculate the shadowing multiplier. 0.0 = Full shadow. 1.0 = No shadow.
#[inline]
#[must_use]
fn shadow(grid: &Grid, shader: &Shader, ray: &Ray, norm: &Unit<Vector3<f64>>) -> f64 {
    let mut light_ray = Ray::new(*ray.pos(), *norm);
    light_ray.travel(shader.bump_dist());
    let light_dir = Unit::new_normalize(shader.sun_pos() - light_ray.pos());
    *light_ray.dir_mut() = light_dir;

    let mut direct = visibility(grid, shader, light_ray);

    direct *= shader.shadow_weights().direct();

    direct
}

/// Calculate the visibility of a given ray.
#[allow(clippy::never_loop)]
#[allow(clippy::single_match_else)]
#[inline]
#[must_use]
fn visibility(grid: &Grid, shader: &Shader, mut ray: Ray) -> f64 {
    let mut vis = 1.0;
    while let Some(hit) = grid.observe(ray.clone(), shader.bump_dist()) {
        if vis <= 0.0 {
            break;
        }

        let mut dist = hit.dist();

        match hit.group() {
            _ => {
                // Opaque
                vis = 0.1;
                break;
            }
        }

        ray.travel(dist);
    }

    vis
}
