//! Rendering simulation sub-module.

pub mod camera;
pub mod frame;
pub mod grid;
pub mod group;
pub mod hit;
pub mod lighting;
pub mod pipe;
pub mod scan;
pub mod scene;
pub mod scheme;

pub use self::{camera::*, frame::*, grid::*, group::*, hit::*, scan::*, scene::*, scheme::*};

use crate::util::{ParProgressBar, ProgressBar};
use ndarray::Array2;
use palette::LinSrgba;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

/// Image splitting factor in each dimension.
const SPLITTING_FACTOR: usize = 8;
/// Distance to travel away from colliding surfaces.
const BUMP_DIST: f64 = 1.0e-6;

/// Run a rendering simulation.
#[inline]
#[must_use]
pub fn image(grid: &Grid, frame: &Frame) -> Array2<LinSrgba> {
    debug_assert!(frame.camera().res().0 % SPLITTING_FACTOR == 0);
    debug_assert!(frame.camera().res().1 % SPLITTING_FACTOR == 0);

    let num_sections = SPLITTING_FACTOR.pow(2);
    let pb = ParProgressBar::new("Rendering", num_sections as u64);
    let pb = Arc::new(Mutex::new(pb));

    let sections: Vec<usize> = (0..num_sections).collect();
    let sections: Vec<(usize, Array2<LinSrgba>)> = sections
        .par_iter()
        .map(|index| {
            pb.lock().expect("Could not lock progress bar.").tick();
            let section = render_section(*index, grid, frame, BUMP_DIST);
            (*index, section)
        })
        .collect();
    pb.lock()
        .expect("Could not lock progress bar.")
        .finish_with_message("Render complete.");

    stitch(frame.camera(), sections)
}

/// Render a section of the image.
#[inline]
#[must_use]
fn render_section(index: usize, grid: &Grid, frame: &Frame, bump_dist: f64) -> Array2<LinSrgba> {
    debug_assert!(bump_dist > 0.0);

    let section_res = frame.camera().frame_res(SPLITTING_FACTOR as usize);

    let fx = index % SPLITTING_FACTOR;
    let fy = index / SPLITTING_FACTOR;

    let start = (section_res.0 * fx, section_res.1 * fy);

    let mut rng = thread_rng();

    let super_samples = frame.quality().super_samples().pow(2);
    let dof_samples = frame.quality().dof_samples();
    let weight = 1.0 / (super_samples * dof_samples) as f32;

    let mut section = Array2::default(section_res);
    for xi in 0..section_res.0 {
        let rx = start.0 + xi;
        for yi in 0..section_res.1 {
            let ry = start.1 + yi;

            for n in 0..super_samples {
                let offset = rng.gen_range(0.0, 2.0 * PI);
                for m in 0..dof_samples {
                    let ray = frame.gen_ray(rx, ry, n, m, offset);

                    *section
                        .get_mut((xi, yi))
                        .expect("Could not access frame pixel.") += pipe::colour(
                        &ray.pos().clone(),
                        grid,
                        frame.shader(),
                        ray,
                        bump_dist,
                        &mut rng,
                    ) * weight;
                }
            }
        }
    }

    section
}

/// Stitch together the stack of sections.
#[inline]
#[must_use]
fn stitch(cam: &Camera, sections: Vec<(usize, Array2<LinSrgba>)>) -> Array2<LinSrgba> {
    let mut img = unsafe { Array2::uninitialized(cam.res()) };

    let img_res = cam.res();
    let section_res = (img_res.0 / SPLITTING_FACTOR, img_res.1 / SPLITTING_FACTOR);

    let mut pb = ProgressBar::new("Stitching", sections.len() as u64);
    for (index, section) in sections {
        pb.tick();

        let nx = index % SPLITTING_FACTOR;
        let ny = index / SPLITTING_FACTOR;

        let start_x = nx * section_res.0;
        let start_y = ny * section_res.1;

        for (px, fx) in (start_x..(start_x + section_res.0)).zip(0..section_res.0) {
            for (py, fy) in (start_y..(start_y + section_res.1)).zip(0..section_res.1) {
                *img.get_mut((px, py)).expect("Could not write to image.") =
                    *section.get((fx, fy)).expect("Could not read from frame.");
            }
        }
    }
    pb.finish_with_message("Stitching complete.");

    img
}
