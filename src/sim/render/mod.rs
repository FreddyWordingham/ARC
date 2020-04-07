//! Rendering simulation sub-module.

pub mod camera;
pub mod frame;
pub mod grid;
pub mod group;
pub mod hit;
pub mod lighting;
pub mod scan;
pub mod scene;

pub use self::{camera::*, frame::*, grid::*, group::*, hit::*, scan::*, scene::*};

use ndarray::Array2;
use palette::LinSrgba;

/// Run a rendering simulation.
#[inline]
#[must_use]
pub fn image(grid: &Grid, frame: &Frame) -> Array2<LinSrgba> {
    let cam = frame.camera();

    Array2::default(cam.res())

    // let pb = ParProgressBar::new("Rendering", total_frames as u64);
    // let pb = Arc::new(Mutex::new(pb));

    // let frames: Vec<usize> = (0..total_frames).collect();

    // let frames: Vec<(usize, Array2<LinSrgba>)> = frames
    //     .par_iter()
    //     .map(|index| {
    //         pb.lock().expect("Could not lock progress bar.").tick();
    //         let frame = render_frame(*index, sett, cam, root, BUMP_DIST);
    //         if cam.frame_saving() {
    //             save::png(out_dir, &format!("{}_{}", name, index), frame.clone());
    //         }
    //         (*index, frame)
    //     })
    //     .collect();
    // pb.lock()
    //     .expect("Could not lock progress bar.")
    //     .finish_with_message("Render complete.");

    // stitch(cam, frames)
}
