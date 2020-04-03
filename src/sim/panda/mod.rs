//! Rendering simulation sub-module.

pub mod camera;
pub mod cell;
pub mod grid_settings;
pub mod group;
pub mod hit;
pub mod scan;
pub mod shader_settings;

pub use self::{
    camera::*, cell::*, grid_settings::*, group::*, hit::*, scan::*, shader_settings::*,
};

use crate::util::ParProgressBar;
use ndarray::Array2;
use num_cpus;
use palette::{LinSrgba, Srgba};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a panda rendering simulation.
#[inline]
#[must_use]
pub fn run(cam: &Camera, _grid: &Cell) -> Array2<LinSrgba> {
    let pb = ParProgressBar::new("Rendering", cam.num_pix() as u64);
    let pb = Arc::new(Mutex::new(pb));
    let thread_ids: Vec<usize> = (0..num_cpus::get()).collect();
    // let thread_ids: Vec<usize> = vec![0];

    let stacks: Vec<_> = thread_ids.par_iter().map(|id| *id).collect();
    pb.lock()
        .expect("Could not lock progress bar.")
        .finish_with_message("Render complete.");

    Array2::from_elem(cam.res(), Srgba::new(1.0, 0.1, 0.6, 1.0).into_linear())
}
