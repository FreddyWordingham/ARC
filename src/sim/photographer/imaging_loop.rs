//! Core MCRT photon loop function.

use crate::{
    sim::Camera,
    util::ParProgressBar,
    world::{Grid, Verse},
};
use ndarray::Array2;
use std::sync::{Arc, Mutex};

/// Run a single threaded instance of the imagine loop.
#[inline]
#[must_use]
pub fn run_thread(
    _id: usize,
    cam: &Camera,
    verse: &Verse,
    grid: &Grid,
    pb: &Arc<Mutex<ParProgressBar>>,
    block_size: u64,
) -> Array2<f64> {
    Array2::zeros(cam.res())
}
