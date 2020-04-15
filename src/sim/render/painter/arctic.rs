//! Arctic rendering painter pipeline.

use crate::{
    geom::Ray,
    img::Shader,
    sim::render::{Grid, Scheme},
};
use nalgebra::Point3;
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Minimum fragment weight to simulate.
const MIN_WEIGHT: f64 = 0.01;

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
    mut _ray: Ray,
    _rng: &mut ThreadRng,
    weight: f64,
) -> LinSrgba {
    debug_assert!(weight > 0.0);

    let col = LinSrgba::default();

    if weight <= MIN_WEIGHT {
        return col;
    }

    col
}
