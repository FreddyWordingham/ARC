//! Rendering simulation sub-module.

pub mod camera;

pub use self::camera::*;

use crate::ord::MeshKey;
use crate::report;
use crate::util::ParProgressBar;
use ndarray::Array2;
use num_cpus;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Perform a rendering simulation.
#[inline]
#[must_use]
pub fn run(cam: &Camera) -> Vec<(MeshKey, Array2<f64>)> {
    let pb = ParProgressBar::new("Imaging Loop", cam.num_pix() as u64);
    let pb = Arc::new(Mutex::new(pb));
    let thread_ids: Vec<usize> = (0..num_cpus::get()).collect();

    let num_pix = cam.num_pix();

    let imgs: Vec<_> = thread_ids
        .par_iter()
        .map(|id| {
            run_thread(
                *id,
                cam,
                &Arc::clone(&pb),
                (num_pix / num_cpus::get()) / 100,
            )
        })
        .collect();
    // pb.lock()
    //     .expect("Could not lock progress bar.")
    //     .finish_with_message("Complete.");

    vec![]
}

fn run_thread(
    id: usize,
    cam: &Camera,
    pb: &Arc<Mutex<ParProgressBar>>,
    block_size: usize,
) -> Vec<(MeshKey, Array2<f64>)> {
    report!(id);

    let stack = vec![];

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size as u64);
        std::mem::drop(pb);
        b
    } {
        for n in start..end {
            let xi = n as usize % cam.res().0;
            let yi = n as usize / cam.res().0;

            let _ray = cam.gen_ray(xi, yi);
        }
    }

    stack
}
