//! Core scanning function.

use crate::{
    geom::Trace,
    sim::render::{Camera, Cell},
    util::ParProgressBar,
};
use log::warn;
use nalgebra::Unit;
use ndarray::Array2;
use std::sync::{Arc, Mutex};

/// Distance to travel away from surfaces.
const BUMP_DIST: f64 = 1.0e-6;

/// Render using a single thread.
#[inline]
#[must_use]
pub fn run_thread(
    _id: usize,
    cam: &Camera,
    grid: &Cell,
    pb: &Arc<Mutex<ParProgressBar>>,
    block_size: usize,
) -> Vec<Array2<f64>> {
    let mut grid_misses = Array2::zeros(cam.res());
    let mut lost_cell = Array2::zeros(cam.res());
    let mut layer_2 = Array2::zeros(cam.res());
    let mut layer_3 = Array2::zeros(cam.res());
    let mut layer_4 = Array2::zeros(cam.res());

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
            let mut total_dist = 0.0;

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

                // loop {
                if let Some((dist, norm, group)) = cell.observe(&ray) {
                    match group {
                        0 => {
                            *layer_2.get_mut((xi, yi)).expect("Invalid pixel index.") += 1.0;
                            *layer_3.get_mut((xi, yi)).expect("Invalid pixel index.") +=
                                norm.dot(ray.dir());
                            *layer_4.get_mut((xi, yi)).expect("Invalid pixel index.") += total_dist;

                            break 'outer;
                        }
                        1 => {
                            ray.travel(dist);
                            let inc = ray.dir().clone();
                            *ray.dir_mut() = Unit::new_normalize(
                                inc.into_inner() - (norm.into_inner() * (2.0 * (inc.dot(&norm)))),
                            );
                            ray.travel(BUMP_DIST);
                            total_dist += dist + BUMP_DIST;
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
                    total_dist += dist + BUMP_DIST;
                    // ray.travel(dist);
                    continue 'outer;
                } else {
                    warn!("Ray escaped cell.");
                    break 'outer;
                }
                // }
            }
        }
    }

    vec![grid_misses, lost_cell, layer_2, layer_3, layer_4]
}
