//! Hit painter.

use crate::{
    geom::Ray,
    img::Shader,
    sim::render::{Grid, Scheme},
};
use nalgebra::Point3;
use palette::{Gradient, LinSrgba};
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

    let backup = Gradient::new(vec![
        LinSrgba::new(1.0, 1.0, 0.0, 1.0),
        LinSrgba::new(0.0, 1.0, 1.0, 1.0),
    ]);
    let grad_0 = if scheme.grads().contains_key(&0) {
        &scheme.grads().get(&0).expect("Invalid gradient group.")
    } else {
        &backup
    };

    let mut col = LinSrgba::default();
    if grid.observe(ray.clone(), shader.bump_dist()).is_some() {
        col += grad_0.get(1.0);
        break;
    }

    col
}
