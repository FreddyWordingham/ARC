//! Lighting functions.

use crate::{
    geom::Ray,
    sim::panda::{Camera, ShaderSettings},
};
use nalgebra::{Unit, Vector3};

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
pub fn specular(sett: &ShaderSettings, ray: &Ray, norm: &Unit<Vector3<f64>>, cam: &Camera) -> f64 {
    let light_dir = Unit::new_normalize(sett.sun_pos() - ray.pos());
    let view_dir = Unit::new_normalize(cam.forward().pos() - ray.pos());

    let ref_dir = reflect(&-light_dir, norm);

    sett.specular() * view_dir.dot(&ref_dir).max(0.0).powi(sett.specular_pow())
}

/// Calculate the reflection vector for a given input unit vector and surface normal.
#[inline]
#[must_use]
fn reflect(inc: &Unit<Vector3<f64>>, norm: &Unit<Vector3<f64>>) -> Unit<Vector3<f64>> {
    Unit::new_normalize(inc.as_ref() + (2.0 * (-inc.dot(norm)) * norm.as_ref()))
} // TODO: Check this for inverted normals.
