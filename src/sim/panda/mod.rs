//! Rendering simulation sub-module.

pub mod camera;
pub mod cell;
pub mod grid_settings;
pub mod group;
pub mod hit;
pub mod save;
pub mod scan;
pub mod shader_settings;

pub use self::{
    camera::*, cell::*, grid_settings::*, group::*, hit::*, scan::*, shader_settings::*,
};

use crate::util::ParProgressBar;
use log::info;
use ndarray::Array2;
use palette::{LinSrgba, Srgba};
use rayon::prelude::*;
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

/// Run a panda rendering simulation.
#[inline]
pub fn run(out_dir: &Path, name: &str, cam: &Camera, grid: &Cell) {
    let pb = ParProgressBar::new("Rendering", cam.num_pix() as u64);
    let pb = Arc::new(Mutex::new(pb));

    let subs = cam.splits().0 * cam.splits().1;
    let frames: Vec<usize> = (0..subs).collect();

    let frames: Vec<(usize, Array2<LinSrgba>)> = frames
        .par_iter()
        .map(|index| {
            let frame = render_frame(*index, cam, grid);
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

    for (index, frame) in frames {
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

    img
}

/// Render a frame of the image.
#[inline]
#[must_use]
fn render_frame(_index: usize, cam: &Camera, _grid: &Cell) -> Array2<LinSrgba> {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();

    // let fx = cam.res().0 / cam.splits().0;
    // let fy = cam.res().1 / cam.splits().1;

    Array2::from_elem(
        cam.frame_res(),
        Srgba::new(rng.gen(), rng.gen(), rng.gen(), 1.0).into_linear(),
    )
}
