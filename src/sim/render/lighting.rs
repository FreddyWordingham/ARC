//! Lighting sub-module.

use crate::{
    geom::{optics, Ray},
    img::Shader,
};
use nalgebra::{Unit, Vector3};

/// Calculate the ambient lighting coefficient.
#[inline]
#[must_use]
pub fn ambient(shader: &Shader) -> f64 {
    shader.light_weights().ambient()
}

/// Calculate the diffuse lighting coefficient.
#[inline]
#[must_use]
pub fn diffuse(
    shader: &Shader,
    ray: &Ray,
    norm: &Unit<Vector3<f64>>,
    light_dir: &Unit<Vector3<f64>>,
) -> f64 {
    shader.light_weights().diffuse() * norm.dot(&light_dir).max(0.0)
}

/// Calculate the specular lighting coefficient.
#[inline]
#[must_use]
pub fn specular(
    shader: &Shader,
    norm: &Unit<Vector3<f64>>,
    light_dir: &Unit<Vector3<f64>>,
    view_dir: &Unit<Vector3<f64>>,
) -> f64 {
    let ref_dir = optics::reflect(&Unit::new_normalize(-light_dir.as_ref()), norm);
    (view_dir.dot(&ref_dir))
        .max(0.0)
        .powi(shader.light_weights().specular_power())
}
