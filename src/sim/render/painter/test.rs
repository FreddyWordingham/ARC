//! Test painter.

use crate::{
    geom::Ray,
    img::Shader,
    sim::render::{Grid, Scheme},
};
use nalgebra::Point3;
// use palette::{Gradient, LinSrgba};
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Paint the ray if it hits something.
#[inline]
#[must_use]
pub fn paint(
    _cam_pos: &Point3<f64>,
    _grid: &Grid,
    _shader: &Shader,
    _scheme: &Scheme,
    _ray: Ray,
    _rng: &mut ThreadRng,
) -> LinSrgba {
    let mut col = LinSrgba::default();

    // while let Some(hit) = root.observe() {

    // }

    col
}
