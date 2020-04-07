//! Image quality settings implementation.

use crate::clone;
use attr::json;

/// Quality settings.
#[json]
pub struct Quality {
    /// Number of section splits in each axis.
    section_splits: (usize, usize),
    /// Target number of image pixels.
    target_pixels: usize,
    /// Super sampling power.
    super_samples: Option<usize>,
    /// Depth of field samples.
    dof_samples: Option<usize>,
    /// Shadow samples.
    shadow_samples: usize,
}

impl Quality {
    clone!(section_splits, (usize, usize));
    clone!(target_pixels, usize);
    clone!(super_samples, Option<usize>);
    clone!(dof_samples, Option<usize>);
    clone!(shadow_samples, usize);

    /// Calculate the number of samples expected per pixel.
    #[inline]
    #[must_use]
    pub fn samples_per_pixel(&self) -> usize {
        self.super_samples.unwrap_or(1).pow(2) * self.dof_samples.unwrap_or(1) * self.shadow_samples
    }
}
