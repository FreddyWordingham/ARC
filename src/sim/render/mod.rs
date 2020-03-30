//! Rendering simulation sub-module.

pub mod camera;

pub use self::camera::*;

use crate::{
    geom::{Ray, Trace},
    ord::{MeshKey, MeshSet},
    util::ParProgressBar,
};
use log::warn;
use ndarray::Array2;
use num_cpus;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Perform a rendering simulation.
#[inline]
#[must_use]
pub fn run(cam: &Camera, ents: &MeshSet) -> Vec<(MeshKey, Array2<f64>)> {
    let pb = ParProgressBar::new("Imaging Loop", cam.num_pix() as u64);
    let pb = Arc::new(Mutex::new(pb));
    // let thread_ids: Vec<usize> = (0..num_cpus::get()).collect();
    let thread_ids: Vec<usize> = vec![0];

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
                ents,
                &Arc::clone(&pb),
                ((num_pix / num_cpus::get()) / 100).max(10),
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
    _id: usize,
    cam: &Camera,
    ents: &MeshSet,
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

            let mut ray = cam.gen_ray(xi, yi);

            while let Some((key, dist)) = scan(ents, &ray) {
                ray.travel(dist);

                match key.str() {
                    "road" | "path" | "sides" => {
                        *road.get_mut((xi, yi)).expect("Invalid pixel index.") += dist;
                        break;
                    }
                    "cube" => {
                        *road.get_mut((xi, yi)).expect("Invalid pixel index.") += dist;
                        break;
                    }
                    _ => {
                        warn!("Do not know how to handle {}.", key);
                        break;
                    } // _ => panic!("Do not know how to handle {}.", key),
                }
            }
        }
    }

    stack
}

/// Scan for what the ray will hit.
fn scan<'a>(ents: &'a MeshSet, ray: &Ray) -> Option<(&'a MeshKey, f64)> {
    let mut hit = None;

    for (key, mesh) in ents.map() {
        if let Some(dist) = mesh.dist(ray) {
            if let Some((_, d)) = hit {
                if dist < d {
                    hit = Some((key, dist));
                }
            } else {
                hit = Some((key, dist));
            }
        }
    }

    hit
}
