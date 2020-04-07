//! Lighting sub-module.

use crate::{geom::Ray, img::Shader};
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
pub fn diffuse(shader: &Shader, ray: &Ray, norm: &Unit<Vector3<f64>>) -> f64 {
    let light_dir = Unit::new_normalize(shader.sun_pos() - ray.pos());

    shader.light_weights().diffuse() * norm.dot(&light_dir).abs()
}
