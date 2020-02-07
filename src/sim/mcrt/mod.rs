//! Monte-Carlo radiative transfer simulation sub-module.

pub mod hit;
pub mod light_map;
pub mod photon_loop;
pub mod record;

pub use self::{hit::*, light_map::*, record::*};

use crate::{
    ord::LightKey,
    util::ParProgressBar,
    world::{Grid, Verse},
};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a Monte-Carlo radiative transfer simulation.
#[inline]
#[must_use]
pub fn run(
    num_threads: usize,
    num_phot: u64,
    light: &LightKey,
    verse: &Verse,
    grid: &Grid,
) -> LightMap {
    let pb = ParProgressBar::new("Photon Loop", num_phot as u64);
    let pb = Arc::new(Mutex::new(pb));
    let thread_ids: Vec<usize> = (0..num_threads).collect();

    let mut light_maps: Vec<_> = thread_ids
        .par_iter()
        .map(|id| {
            photon_loop::run_thread(
                *id,
                verse,
                grid,
                verse.lights().get(light),
                num_phot,
                &Arc::clone(&pb),
                ((num_phot / num_threads as u64) / 100).max(10) as u64,
            )
        })
        .collect();
    pb.lock()
        .expect("Could not lock progress bar.")
        .finish_with_message("Complete.");

    let mut light_map = light_maps.pop().expect("Did not receive any light maps.");
    for lm in light_maps {
        light_map += &lm;
    }

    light_map
}
