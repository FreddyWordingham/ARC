//! Rendering simulation sub-module.

pub mod camera;

pub use self::camera::*;

use crate::ord::MeshKey;
use ndarray::Array2;

/// Perform a rendering simulation.
#[inline]
#[must_use]
pub fn run() -> Vec<(MeshKey, Array2<f64>)> {
    vec![]
}
