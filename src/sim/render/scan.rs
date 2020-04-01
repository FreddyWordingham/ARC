//! Core scanning function.

use crate::{
    sim::render::{Camera, Cell, Settings, Tracer},
    util::ParProgressBar,
};
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
    _sett: &Settings,
) -> Vec<Array2<f64>> {
    let mut layer_0 = Array2::zeros(cam.res());
    let mut layer_1 = Array2::zeros(cam.res());

    let super_samples = cam.ss_power().pow(2);

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size as u64);
        std::mem::drop(pb);
        b
    } {
        for n in start..end {
            let xi = n as usize % cam.res().0;
            let yi = n as usize / cam.res().0;

            for n in 0..super_samples {
                let mut tracer = Tracer::new(cam.gen_ss_ray(xi, yi, n));

                while let Some((new_tracer, _dist, _norm, group)) = grid.observe(tracer) {
                    tracer = new_tracer;

                    match group {
                        0 => {
                            *layer_0.get_mut((xi, yi)).expect("Invalid pixel index.") += 1.0;
                            *layer_1.get_mut((xi, yi)).expect("Invalid pixel index.") +=
                                tracer.dist_travelled();
                            break;
                        }
                        // 1 => {}
                        // 2 => {}
                        _ => {
                            // warn!("Do not know how to handle group {}.", group);
                            break;
                        }
                    }
                }
            }
        }
    }

    vec![layer_0, layer_1]
}
