//! Rendering simulation sub-module.

pub mod camera;
pub mod cell;
pub mod group;
pub mod scan;

pub use self::{camera::*, cell::*, group::*};

use crate::util::ParProgressBar;
use log::info;
use ndarray::Array2;
use num_cpus;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Perform a rendering simulation.
#[inline]
#[must_use]
pub fn run(cam: &Camera, grid: &Cell) -> Vec<Array2<f64>> {
    let pb = ParProgressBar::new("Rendering", cam.num_pix() as u64);
    let pb = Arc::new(Mutex::new(pb));
    let thread_ids: Vec<usize> = (0..num_cpus::get()).collect();
    // let thread_ids: Vec<usize> = vec![0];

    let num_pix = cam.num_pix();

    let mut stacks: Vec<_> = thread_ids
        .par_iter()
        .map(|id| {
            scan::run_thread(
                *id,
                cam,
                grid,
                &Arc::clone(&pb),
                ((num_pix / num_cpus::get()) / 100).max(10),
            )
        })
        .collect();
    pb.lock()
        .expect("Could not lock progress bar.")
        .finish_with_message("Render complete.");

    info!("Stacking images from threads");
    let mut prime_stack = stacks.pop().expect("Did not receive any images.");
    for stack in stacks {
        for (prime_img, stack_img) in prime_stack.iter_mut().zip(stack.iter()) {
            *prime_img += stack_img;
        }
    }

    prime_stack
}
