//! Imaging simulation sub-module.

pub mod camera;

pub use self::camera::*;

use crate::world::{Grid, Verse};
use ndarray::Array2;

/// Run a diffusion transfer simulation.
#[inline]
// #[must_use]
pub fn run(num_threads: usize, cam: &Camera, _verse: &Verse, _grid: &Grid) -> Array2<f64> {
    debug_assert!(num_threads > 0);

    // let cam = Camera::new(res);

    // cam.to_image()

    Array2::zeros((cam.res().0 as usize, cam.res().1 as usize))
}
