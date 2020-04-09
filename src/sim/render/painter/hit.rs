//! Hit painter.

use crate::{
    geom::Ray,
    img::Shader,
    sim::render::{Grid, Scheme},
};
use nalgebra::Point3;
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Paint the ray if it hits something.
#[inline]
#[must_use]
pub fn paint(
    _cam_pos: &Point3<f64>,
    grid: &Grid,
    shader: &Shader,
    scheme: &Scheme,
    ray: Ray,
    _rng: &mut ThreadRng,
) -> LinSrgba {
    debug_assert!(shader.bump_dist() > 0.0);

    let mut col = LinSrgba::default();
    if grid.observe(ray, shader.bump_dist()).is_some() {
        col += scheme.get(0).get(1.0);
    }

    col
}
