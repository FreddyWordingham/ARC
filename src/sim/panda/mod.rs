//! Rendering simulation sub-module.

pub mod camera;
pub mod cell;
pub mod grid_settings;
pub mod group;
pub mod hit;
pub mod lighting;
pub mod pipe;
pub mod save;
pub mod scan;
pub mod shader_settings;

pub use self::{
    camera::*, cell::*, grid_settings::*, group::*, hit::*, scan::*, shader_settings::*,
};

use crate::util::{ParProgressBar, ProgressBar};
use log::info;
use ndarray::Array2;
use palette::LinSrgba;
use rand::{rngs::ThreadRng, thread_rng};
use rayon::prelude::*;
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

/// Distance to travel away from surfaces.
const BUMP_DIST: f64 = 1.0e-6;

/// Run a panda rendering simulation.
#[inline]
pub fn run(out_dir: &Path, name: &str, sett: &ShaderSettings, cam: &Camera, root: &Cell) {
    let total_frames = cam.splits().0 * cam.splits().1;
    let pb = ParProgressBar::new("Rendering", total_frames as u64);
    let pb = Arc::new(Mutex::new(pb));

    let frames: Vec<usize> = (0..total_frames).collect();

    let frames: Vec<(usize, Array2<LinSrgba>)> = frames
        .par_iter()
        .map(|index| {
            pb.lock().expect("Could not lock progress bar.").tick();
            let frame = render_frame(*index, sett, cam, root, BUMP_DIST);
            if cam.frame_saving() {
                save::png(out_dir, &format!("{}_{}", name, index), frame.clone());
            }
            (*index, frame)
        })
        .collect();
    pb.lock()
        .expect("Could not lock progress bar.")
        .finish_with_message("Render complete.");

    info!("Stitching image");
    let img = stitch(cam, frames);

    info!("Saving image");
    save::png(out_dir, name, img);
}

/// Stitch together the stack of frames.
#[inline]
#[must_use]
fn stitch(cam: &Camera, frames: Vec<(usize, Array2<LinSrgba>)>) -> Array2<LinSrgba> {
    let mut img = unsafe { Array2::uninitialized(cam.res()) };

    let frame_res = cam.frame_res();

    let mut pb = ProgressBar::new("Stitching", frames.len() as u64);
    for (index, frame) in frames {
        pb.tick();

        let nx = index % cam.splits().0;
        let ny = index / cam.splits().0;

        let start_x = nx * frame_res.0;
        let start_y = ny * frame_res.1;

        for (px, fx) in (start_x..(start_x + frame_res.0)).zip(0..frame_res.0) {
            for (py, fy) in (start_y..(start_y + frame_res.1)).zip(0..frame_res.1) {
                *img.get_mut((px, py)).expect("Could not write to image.") =
                    *frame.get((fx, fy)).expect("Could not read from frame.");
            }
        }
    }
    pb.finish_with_message("Stitching complete.");

    img
}

/// Render a frame of the image.
#[inline]
#[must_use]
fn render_frame(
    index: usize,
    sett: &ShaderSettings,
    cam: &Camera,
    root: &Cell,
    bump_dist: f64,
) -> Array2<LinSrgba> {
    debug_assert!(bump_dist > 0.0);

    let frame_res = cam.frame_res();

    let fx = index % cam.splits().0;
    let fy = index / cam.splits().0;

    let start = (frame_res.0 * fx, frame_res.1 * fy);

    let super_samples = cam.ss_power().pow(2);
    let dof_samples = cam.dof_samples();

    let mut rng = thread_rng();

    if dof_samples > 1 {
        if super_samples > 1 {
            frame_ss_dof(
                sett,
                cam,
                root,
                bump_dist,
                frame_res,
                start,
                super_samples,
                dof_samples,
                &mut rng,
            )
        } else {
            frame_dof(
                sett,
                cam,
                root,
                bump_dist,
                frame_res,
                start,
                dof_samples,
                &mut rng,
            )
        }
    } else {
        if super_samples > 1 {
            frame_ss(
                sett,
                cam,
                root,
                bump_dist,
                frame_res,
                start,
                super_samples,
                &mut rng,
            )
        } else {
            frame(sett, cam, root, bump_dist, frame_res, start, &mut rng)
        }
    }
}

/// Render a basic frame.
#[inline]
#[must_use]
fn frame(
    sett: &ShaderSettings,
    cam: &Camera,
    root: &Cell,
    bump_dist: f64,
    frame_res: (usize, usize),
    start: (usize, usize),
    rng: &mut ThreadRng,
) -> Array2<LinSrgba> {
    let mut frame = Array2::default(frame_res);

    for xi in 0..frame_res.0 {
        let rx = start.0 + xi;
        for yi in 0..frame_res.1 {
            let ry = start.1 + yi;

            let ray = cam.gen_ray(rx, ry);

            *frame
                .get_mut((xi, yi))
                .expect("Could not access frame pixel.") +=
                pipe::colour(sett, &ray.pos().clone(), root, ray, bump_dist, rng);
        }
    }

    frame
}

/// Render a frame with super-sampling.
#[inline]
#[must_use]
fn frame_ss(
    sett: &ShaderSettings,
    cam: &Camera,
    root: &Cell,
    bump_dist: f64,
    frame_res: (usize, usize),
    start: (usize, usize),
    super_samples: i32,
    rng: &mut ThreadRng,
) -> Array2<LinSrgba> {
    let mut frame = Array2::default(frame_res);

    for xi in 0..frame_res.0 {
        let rx = start.0 + xi;
        for yi in 0..frame_res.1 {
            let ry = start.1 + yi;

            for n in 0..super_samples {
                let ray = cam.gen_ss_ray(rx, ry, n as i32);

                *frame
                    .get_mut((xi, yi))
                    .expect("Could not access frame pixel.") +=
                    pipe::colour(sett, &ray.pos().clone(), root, ray, bump_dist, rng)
                        / super_samples as f32;
            }
        }
    }

    frame
}

/// Render a frame with depth-of-field.
#[inline]
#[must_use]
fn frame_dof(
    sett: &ShaderSettings,
    cam: &Camera,
    root: &Cell,
    bump_dist: f64,
    frame_res: (usize, usize),
    start: (usize, usize),
    dof_samples: i32,
    rng: &mut ThreadRng,
) -> Array2<LinSrgba> {
    let mut frame = Array2::default(frame_res);

    for xi in 0..frame_res.0 {
        let rx = start.0 + xi;
        for yi in 0..frame_res.1 {
            let ry = start.1 + yi;

            for m in 0..dof_samples {
                let ray = cam.gen_ss_dof_ray(rx, ry, 0, m as i32);

                *frame
                    .get_mut((xi, yi))
                    .expect("Could not access frame pixel.") +=
                    pipe::colour(sett, &ray.pos().clone(), root, ray, bump_dist, rng)
                        / dof_samples as f32;
            }
        }
    }

    frame
}

/// Render a frame with super-sampling and depth-of-field.
#[allow(clippy::too_many_arguments)]
#[inline]
#[must_use]
fn frame_ss_dof(
    sett: &ShaderSettings,
    cam: &Camera,
    root: &Cell,
    bump_dist: f64,
    frame_res: (usize, usize),
    start: (usize, usize),
    super_samples: i32,
    dof_samples: i32,
    rng: &mut ThreadRng,
) -> Array2<LinSrgba> {
    let mut frame = Array2::default(frame_res);

    for xi in 0..frame_res.0 {
        let rx = start.0 + xi;
        for yi in 0..frame_res.1 {
            let ry = start.1 + yi;

            for n in 0..super_samples {
                for m in 0..dof_samples {
                    let ray = cam.gen_ss_dof_ray(rx, ry, n as i32, m as i32);

                    *frame
                        .get_mut((xi, yi))
                        .expect("Could not access frame pixel.") +=
                        pipe::colour(sett, &ray.pos().clone(), root, ray, bump_dist, rng)
                            / (super_samples * dof_samples) as f32;
                }
            }
        }
    }

    frame
}
