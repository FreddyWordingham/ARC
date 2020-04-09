//! Lumin rendering painter pipeline.

use crate::{
    geom::Ray,
    img::Shader,
    phys::laws::reflect_dir,
    sim::render::{Grid, Scheme},
};
use nalgebra::{Point3, Unit, Vector3};
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

        let light = light(cam_pos, shader, &ray, hit.side().norm());
        let shadow = 0.1;
        let illumination = light * (1.0 - shadow);

        match hit.group() {
            _ => {
                col += scheme.get(hit.group()).get(illumination as f32);
                break;
            }
        }
    }

    col
}

/// Calculate the lighting.
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
