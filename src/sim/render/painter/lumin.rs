//! Lumin rendering painter pipeline.

use crate::{
    geom::Ray,
    img::Shader,
    // phys::{laws::reflect_dir, Crossing},
    sim::render::{Grid, Scheme},
};
use nalgebra::Point3;
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Paint the ray if it hits something.
#[allow(clippy::never_loop)]
#[allow(clippy::single_match_else)]
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
    let col = LinSrgba::default();
    col
}
