//! Core scanning function.

use crate::{
    sim::render::{Camera, Cell},
    util::ParProgressBar,
};
use log::warn;
use ndarray::Array2;
use std::sync::{Arc, Mutex};

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
    let mut layer_0 = Array2::zeros(cam.res());
    let mut layer_1 = Array2::zeros(cam.res());
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

            while let Some((new_ray, dist, norm, group)) = grid.observe(ray) {
                ray = new_ray;

                match group {
                    0 => {
                        *layer_0.get_mut((xi, yi)).expect("Invalid pixel index.") += 1.0;
                        *layer_1.get_mut((xi, yi)).expect("Invalid pixel index.") += dist;
                        *layer_2.get_mut((xi, yi)).expect("Invalid pixel index.") +=
                            ray.dir().dot(&norm).acos();
                        break;
                    }
                    1 => {
                        ray.reflect(&norm);
                        ray.travel(1.0e-6);
                    }
                    _ => {
                        warn!("Do not know how to handle group {}.", group);
                        break;
                    }
                }
            }
        }
    }

    vec![layer_0, layer_1, layer_2]
}
