//! Image implementation.

pub mod camera;
pub mod palette;

pub use self::{camera::*, palette::*};

use crate::{
    access,
    rend::{
        settings::{Quality, Shader},
        AspectRatio, Grid, Scene,
    },
};
use ::palette::LinSrgba;
use ndarray::Array2;

/// Image structure.
pub struct Image {
    /// Target aspect ratio.
    aspect_ratio: AspectRatio,
    /// Quality settings.
    quality: Quality,
    /// Shader settings.
    shader: Shader,
    /// Palette colours.
    palette: Palette,
    /// Camera.
    camera: Camera,
}

impl Image {
    access!(aspect_ratio, AspectRatio);
    access!(quality, Quality);
    access!(shader, Shader);
    access!(palette, Palette);
    access!(camera, Camera);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        aspect_ratio: AspectRatio,
        quality: Quality,
        shader: Shader,
        palette: Palette,
        camera: Camera,
    ) -> Self {
        Self {
            aspect_ratio,
            quality,
            shader,
            palette,
            camera,
        }
    }

    /// Create a rendering of a given scene.
    #[inline]
    #[must_use]
    pub fn render(&self, _scene: &Scene, _grid: &Grid) -> Array2<LinSrgba> {
        Array2::default(self.camera.res())

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
}
