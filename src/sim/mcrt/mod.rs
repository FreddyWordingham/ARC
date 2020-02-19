//! Monte-Carlo radiative transfer simulation sub-module.

pub mod cell;
pub mod grid;
pub mod hit;
pub mod light_map;
pub mod photon_loop;
pub mod record;
pub mod settings;

pub use self::{cell::*, grid::*, hit::*, light_map::*, record::*, settings::*};

use crate::{ord::LightSet, util::ParProgressBar, world::Light};
use num_cpus;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a MCRT simulation.
#[inline]
#[must_use]
pub fn run(num_phot: u64, light: &Light, grid: &Grid) -> LightMap {
    debug_assert!(num_phot > 0);

    let pb = ParProgressBar::new("Photon Loop", num_phot);
    let pb = Arc::new(Mutex::new(pb));
    let thread_ids: Vec<usize> = (0..num_cpus::get()).collect();

    let mut light_maps: Vec<_> = thread_ids
        .par_iter()
        .map(|id| {
            photon_loop::run_thread(
                &Arc::clone(&pb),
                ((num_phot / num_cpus::get() as u64) / 100).max(1) as u64,
                light,
                grid,
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
