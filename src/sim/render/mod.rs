//! Rendering simulation sub-module.

pub mod camera;

pub use self::camera::*;

use crate::{ord::MeshKey, util::ParProgressBar};
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

    let mut prime_stack = vec![
        (MeshKey::new("road"), Array2::zeros(cam.res())),
        (MeshKey::new("sides"), Array2::zeros(cam.res())),
        (MeshKey::new("trees"), Array2::zeros(cam.res())),
        (MeshKey::new("beans"), Array2::zeros(cam.res())),
    ];

    let stacks: Vec<_> = thread_ids
        .par_iter()
        .map(|id| {
            run_thread(
                *id,
                cam,
                &Arc::clone(&pb),
                (num_pix / num_cpus::get()) / 100,
                prime_stack.clone(),
            )
        })
        .collect();
    pb.lock()
        .expect("Could not lock progress bar.")
        .finish_with_message("Render complete.");

    for stack in stacks {
        for ((prime_key, prime_img), (stack_key, stack_img)) in
            prime_stack.iter_mut().zip(stack.iter())
        {
            debug_assert!(prime_key == stack_key);
            *prime_img += stack_img;
        }
    }

    prime_stack
}

/// Render using a single thread.
#[inline]
#[must_use]
fn run_thread(
    id: usize,
    cam: &Camera,
    pb: &Arc<Mutex<ParProgressBar>>,
    block_size: usize,
    mut stack: Vec<(MeshKey, Array2<f64>)>,
) -> Vec<(MeshKey, Array2<f64>)> {
    let road = &mut stack.get_mut(0).expect("Invalid image index.").1;

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

            *road.get_mut((xi, yi)).expect("Invalid pixel index.") += id as f64;
        }
    }

    stack
}
