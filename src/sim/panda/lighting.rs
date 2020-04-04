//! Lighting functions.

use crate::{
    geom::Ray,
    math::sample::golden,
    sim::panda::{Cell, ShaderSettings},
};
use nalgebra::{Point3, Unit, Vector3};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Calculate the ambient lighting coefficient.
#[inline]
#[must_use]
pub fn ambient(sett: &ShaderSettings) -> f64 {
    sett.ambient()
}

/// Calculate the diffuse lighting coefficient.
#[inline]
#[must_use]
pub fn diffuse(sett: &ShaderSettings, ray: &Ray, norm: &Unit<Vector3<f64>>) -> f64 {
    let light_dir = Unit::new_normalize(sett.sun_pos() - ray.pos());

    sett.diffuse() * norm.dot(&light_dir).abs()
}

/// Calculate the specular lighting coefficient.
#[inline]
#[must_use]
pub fn specular(
    sett: &ShaderSettings,
    ray: &Ray,
    norm: &Unit<Vector3<f64>>,
    cam_pos: &Point3<f64>,
) -> f64 {
    let light_dir = Unit::new_normalize(sett.sun_pos() - ray.pos());
    let view_dir = Unit::new_normalize(cam_pos - ray.pos());

    let ref_dir = reflect(&-light_dir, norm);

    sett.specular() * view_dir.dot(&ref_dir).max(0.0).powi(sett.specular_pow())
}

/// Calculate the sunlight factor.
#[inline]
#[must_use]
pub fn sunlight(
    sett: &ShaderSettings,
    ray: &Ray,
    norm: &Unit<Vector3<f64>>,
    root: &Cell,
    bump_dist: f64,
) -> f64 {
    debug_assert!(bump_dist > 0.0);

    let mut light_ray = Ray::new(*ray.pos(), *norm);
    light_ray.travel(bump_dist);

    *light_ray.dir_mut() = Unit::new_normalize(sett.sun_pos() - light_ray.pos());

    let mut light = 1.0;
    while let Some(hit) = root.observe(light_ray.clone(), bump_dist) {
        match hit.group() {
            -1 => {
                light *= 1.0 - sett.transparency();
            }
            _ => {
                return sett.shadow();
            }
        }

        light_ray.travel(hit.dist() + bump_dist);
    }

    light
}

/// Calculate the sunlight factor from a number of samples.
#[inline]
#[must_use]
pub fn sunlight_samples(
    sett: &ShaderSettings,
    ray: &Ray,
    norm: &Unit<Vector3<f64>>,
    root: &Cell,
    bump_dist: f64,
    rng: &mut ThreadRng,
) -> f64 {
    debug_assert!(bump_dist > 0.0);
    debug_assert!(sett.sunlight_samples() > 0);
    debug_assert!(sett.sunlight_radius() > 0.0);

    let shadow = |mut light_ray: Ray| {
        let mut light = 1.0;
        while let Some(hit) = root.observe(light_ray.clone(), bump_dist) {
            match hit.group() {
                -1 => {
                    light *= 1.0 - sett.transparency();
                }
                _ => {
                    return sett.shadow();
                }
            }

            light_ray.travel(hit.dist() + bump_dist);
        }

        light
    };

    let mut light_ray = Ray::new(*ray.pos(), *norm);
    light_ray.travel(bump_dist + (sett.sunlight_radius() * 0.1));

    let mut total_shadow = 0.0;
    let offset = rng.gen::<f64>() * 2.0 * PI;
    for n in 0..sett.sunlight_samples() {
        let mut lr = light_ray.clone();
        let (theta, phi) = golden::hemisphere(n as i32, sett.sunlight_samples());
        lr.rotate(phi, theta + offset);
        lr.travel(sett.sunlight_radius());

        total_shadow += shadow(Ray::new(
            *lr.pos(),
            Unit::new_normalize(sett.sun_pos() - lr.pos()),
        ));
    }

    total_shadow / sett.sunlight_samples() as f64
}

/// Calculate ambient light.
#[inline]
#[must_use]
pub fn casting_samples(
    sett: &ShaderSettings,
    ray: &Ray,
    norm: &Unit<Vector3<f64>>,
    root: &Cell,
    bump_dist: f64,
    rng: &mut ThreadRng,
) -> f64 {
    debug_assert!(bump_dist > 0.0);
    debug_assert!(sett.sunlight_samples() > 0);
    debug_assert!(sett.sunlight_radius() > 0.0);

    let norm = if norm.dot(ray.dir()) < 0.0 {
        *norm
    } else {
        Unit::new_normalize(norm.as_ref() * -1.0)
    };

    let mut obs_ray = Ray::new(*ray.pos(), norm);
    obs_ray.travel(bump_dist + sett.sunlight_radius());

    let mut total_shadow = 0.0;
    let offset = rng.gen::<f64>() * 2.0 * PI;
    for n in 0..sett.sunlight_samples() {
        let mut lr = obs_ray.clone();
        let (theta, phi) = golden::sphere(n as i32, sett.sunlight_samples());
        lr.rotate(phi, theta + offset);

        total_shadow += sunlight(sett, &lr, &norm, root, bump_dist);
    }

    total_shadow / sett.sunlight_samples() as f64
}

/// Calculate the reflection vector for a given input unit vector and surface normal.
#[inline]
#[must_use]
fn reflect(inc: &Unit<Vector3<f64>>, norm: &Unit<Vector3<f64>>) -> Unit<Vector3<f64>> {
    Unit::new_normalize(inc.as_ref() + (2.0 * (-inc.dot(norm)) * norm.as_ref()))
} // TODO: Check this for inverted normals.
