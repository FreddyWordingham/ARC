//! Rendering simulation sub-module.

pub mod camera;
pub mod cell;
pub mod grid;
pub mod group;

pub use self::camera::*;
pub use self::cell::*;
pub use self::grid::*;
pub use self::group::*;

use crate::{
    geom::{Ray, Trace},
    ord::{MeshKey, MeshSet},
    util::ParProgressBar,
};
use log::warn;
use nalgebra::{Unit, Vector3};
use ndarray::Array2;
use num_cpus;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Distance to travel away from surfaces.
const BUMP_DIST: f64 = 0.001;

/// Perform a rendering simulation.
#[inline]
#[must_use]
pub fn run(cam: &Camera, ents: &MeshSet) -> Vec<(MeshKey, Array2<f64>)> {
    let pb = ParProgressBar::new("Imaging Loop", cam.num_pix() as u64);
    let pb = Arc::new(Mutex::new(pb));
    let thread_ids: Vec<usize> = (0..num_cpus::get()).collect();

    let num_pix = cam.num_pix();

    let mut stacks: Vec<_> = thread_ids
        .par_iter()
        .map(|id| {
            run_thread(
                *id,
                cam,
                ents,
                &Arc::clone(&pb),
                ((num_pix / num_cpus::get()) / 100).max(10),
            )
        })
        .collect();
    pb.lock()
        .expect("Could not lock progress bar.")
        .finish_with_message("Render complete.");

    let mut prime_stack = stacks.pop().expect("Did not receive any images.");
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
) -> Vec<(MeshKey, Array2<f64>)> {
    let mut people = Array2::zeros(cam.res());
    let mut floor = Array2::zeros(cam.res());
    let path = Array2::zeros(cam.res());
    let mut trees = Array2::zeros(cam.res());
    let mut leaves = Array2::zeros(cam.res());
    let mut hills = Array2::zeros(cam.res());

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

            while let Some((key, dist, norm)) = scan(ents, &ray) {
                match key.str() {
                    "fjmw" | "klm" | "dog" => {
                        *people.get_mut((xi, yi)).expect("Invalid pixel index.") += 1.0;
                        break;
                    }
                    "floor" => {
                        *floor.get_mut((xi, yi)).expect("Invalid pixel index.") += 1.0;
                        break;
                    }
                    "hills" | "sides" => {
                        *hills.get_mut((xi, yi)).expect("Invalid pixel index.") += 1.0;
                        break;
                    }
                    "leaves" | "bushes" => {
                        *leaves.get_mut((xi, yi)).expect("Invalid pixel index.") += 1.0;
                        break;
                    }
                    "trees" | "lamps" => {
                        *trees.get_mut((xi, yi)).expect("Invalid pixel index.") += 1.0;
                        break;
                    }
                    "windows" | "path" => {
                        ray.travel(dist);
                        let inc = ray.dir().clone();
                        *ray.dir_mut() = Unit::new_normalize(
                            inc.into_inner() - (norm.into_inner() * (2.0 * (inc.dot(&norm)))),
                        );
                        ray.travel(BUMP_DIST);
                    }
                    _ => {
                        warn!("Do not know how to handle {}.", key);
                        break;
                    } // _ => panic!("Do not know how to handle {}.", key),
                }
            }
        }
    }

    vec![
        (MeshKey::new("people"), people),
        (MeshKey::new("floor"), floor),
        (MeshKey::new("path"), path),
        (MeshKey::new("trees"), trees),
        (MeshKey::new("leaves"), leaves),
        (MeshKey::new("hills"), hills),
    ]
}

/// Scan for what the ray will hit.
fn scan<'a>(ents: &'a MeshSet, ray: &Ray) -> Option<(&'a MeshKey, f64, Unit<Vector3<f64>>)> {
    let mut hit = None;

    for (key, mesh) in ents.map() {
        if let Some((dist, norm)) = mesh.dist_norm(ray) {
            if let Some((_, d, _)) = hit {
                if dist < d {
                    hit = Some((key, dist, norm));
                }
            } else {
                hit = Some((key, dist, norm));
            }
        }
    }

    hit
}
