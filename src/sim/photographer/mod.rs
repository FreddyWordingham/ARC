//! Monte-Carlo radiative transfer simulation sub-module.

pub mod camera;

pub use self::camera::*;

use crate::world::{Grid, Verse};
use ndarray::Array2;

/// Run an photography simulation.
#[inline]
#[must_use]
pub fn run(
    _num_threads: usize,
    _num_phot: u64,
    res: (usize, usize),
    _cam: &Camera,
    _verse: &Verse,
    _grid: &Grid,
) -> Array2<f64> {
    Array2::zeros(res)
}
