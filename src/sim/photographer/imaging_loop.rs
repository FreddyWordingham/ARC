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
    id: usize,
    cam: &Camera,
    verse: &Verse,
    grid: &Grid,
    pb: &Arc<Mutex<ParProgressBar>>,
    block_size: u64,
) -> Array2<f64> {
    // let bump_dist = grid.bump_dist();

    let mut img = Array2::zeros(cam.res());

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        for n in start..end {
            let xi = n as usize % cam.res().0;
            let yi = n as usize / cam.res().0;

            let ray = cam.gen_ray(xi, yi);

            *img.get_mut([xi, yi]).unwrap() += id as f64;
        }
    }

    img
}
