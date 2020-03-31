//! Rendering simulation sub-module.

pub mod camera;
pub mod cell;
pub mod grid_settings;
pub mod group;

pub use self::{camera::*, cell::*, grid_settings::*, group::*};

use crate::{geom::Trace, util::ParProgressBar};
use log::{info, warn};
use nalgebra::Unit;
use ndarray::Array2;
use num_cpus;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Distance to travel away from surfaces.
const BUMP_DIST: f64 = 0.001;

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
            run_thread(
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

/// Render using a single thread.
#[inline]
#[must_use]
fn run_thread(
    _id: usize,
    cam: &Camera,
    grid: &Cell,
    pb: &Arc<Mutex<ParProgressBar>>,
    block_size: usize,
) -> Vec<Array2<f64>> {
    let mut grid_misses = Array2::zeros(cam.res());
    let mut lost_cell = Array2::zeros(cam.res());
    let mut layer_2 = Array2::zeros(cam.res());

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size as u64);
        std::mem::drop(pb);
        b
    } {
        for n in start..end {
            let xi = n as usize % cam.res().0;
            let yi = n as usize / cam.res().0;

            let mut ray = cam.gen_ray(xi, yi);

            if !grid.boundary().contains(ray.pos()) {
                if let Some(dist) = grid.boundary().dist(&ray) {
                    ray.travel(dist + BUMP_DIST);
                } else {
                    *grid_misses.get_mut((xi, yi)).expect("Invalid pixel index.") += 1.0;
                    // warn!("Observation ray missed grid.");
                    continue;
                }
            }

            if grid.find_terminal_cell(ray.pos()).is_none() {
                *lost_cell.get_mut((xi, yi)).expect("Invalid pixel index.") += 1.0;
                continue;
            }

            'outer: while let Some(cell) = grid.find_terminal_cell(ray.pos()) {
                debug_assert!(cell.boundary().contains(ray.pos()));

                loop {
                    if let Some((dist, norm, group)) = cell.observe(&ray) {
                        match group {
                            0 => {
                                *layer_2.get_mut((xi, yi)).expect("Invalid pixel index.") += 1.0;
                                break 'outer;
                            }
                            1 => {
                                ray.travel(dist);
                                let inc = ray.dir().clone();
                                *ray.dir_mut() = Unit::new_normalize(
                                    inc.into_inner()
                                        - (norm.into_inner() * (2.0 * (inc.dot(&norm)))),
                                );
                                // ray.travel(BUMP_DIST);
                                continue 'outer;
                            }
                            _ => {
                                warn!("Do not know how to handle group {}.", group);
                                break 'outer;
                            }
                        }
                    } else if let Some(dist) = cell.boundary().dist(&ray)
                    // .expect("Could not determine cell boundary distance.")
                    {
                        ray.travel(dist + BUMP_DIST);
                        // ray.travel(dist);
                        continue 'outer;
                    } else {
                        warn!("Ray escaped cell.");
                        break 'outer;
                    }
                }
            }
        }
    }

    vec![grid_misses, lost_cell, layer_2]
}
