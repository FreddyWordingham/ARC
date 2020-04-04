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
use rayon::prelude::*;
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

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
            let frame = render_frame(*index, sett, cam, root);
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
) -> Array2<LinSrgba> {
    let frame_res = cam.frame_res();

    let fx = index % cam.splits().0;
    let fy = index / cam.splits().0;

    let start_x = frame_res.0 * fx;
    let start_y = frame_res.1 * fy;

    let mut frame = Array2::default(frame_res);

    let super_samples = cam.ss_power().pow(2);

    for xi in 0..frame_res.0 {
        let rx = start_x + xi;
        for yi in 0..frame_res.1 {
            let ry = start_y + yi;

            for n in 0..super_samples {
                let ray = cam.gen_ss_ray(rx, ry, n);
                *frame
                    .get_mut((xi, yi))
                    .expect("Could not access frame pixel.") += pipe::colour(sett, cam, root, ray);
            }
        }
    }

    frame
}
