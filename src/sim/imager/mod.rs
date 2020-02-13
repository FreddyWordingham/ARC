//! Imaging simulation sub-module.

pub mod camera;
pub mod imaging_loop;

pub use self::camera::*;

use crate::{
    ord::LightKey,
    util::ParProgressBar,
    world::{Grid, Verse},
};
use ndarray::Array2;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a diffusion transfer simulation.
#[inline]
// #[must_use]
pub fn run(
    num_threads: usize,
    num_phot: u64,
    light: &LightKey,
    cam: &Camera,
    res: (usize, usize),
    verse: &Verse,
    grid: &Grid,
) -> Array2<f64> {
    debug_assert!(num_threads > 0);

    let pb = ParProgressBar::new("Imaging Loop", num_phot as u64);
    let pb = Arc::new(Mutex::new(pb));
    let thread_ids: Vec<usize> = (0..num_threads).collect();

    let mut images: Vec<_> = thread_ids
        .par_iter()
        .map(|id| {
            imaging_loop::run_thread(
                *id,
                verse,
                grid,
                verse.lights().get(light),
                num_phot,
                &Arc::clone(&pb),
                ((num_phot / num_threads as u64) / 1000).max(10) as u64,
                &cam,
                res,
            )
        })
        .collect();
    pb.lock()
        .expect("Unable to lock progress bar.")
        .finish_with_message("Complete.");

    let mut image = images.pop().expect("Did not receive any images.");
    for img in images {
        image += &img;
    }

    image
}
