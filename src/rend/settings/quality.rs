//! Image quality settings implementation.

use crate::clone;
use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Quality settings.
#[json]
pub struct Quality {
    /// Total image pixels.
    total_pixels: usize,
    /// Super samples.
    super_samples: usize,
    /// Depth of field samples.
    dof_samples: usize,
    /// Shadow samples.
    shadow_samples: usize,
}

impl Quality {
    clone!(total_pixels, usize);
    clone!(super_samples, usize);
    clone!(dof_samples, usize);
    clone!(shadow_samples, usize);

    /// Calculate the number of samples expected per pixel.
    #[inline]
    #[must_use]
    pub fn samples_per_pixel(&self) -> usize {
        self.super_samples * self.dof_samples * self.shadow_samples
    }

    /// Calculate the total number of samples expected.
    #[inline]
    #[must_use]
    pub fn total_samples(&self) -> usize {
        self.total_pixels * self.samples_per_pixel()
    }
}

impl Display for Quality {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt, "{:>30} : {}", "super samples", self.super_samples)?;
        writeln!(
            fmt,
            "{:>30} : {}",
            "depth of field samples", self.dof_samples
        )?;
        writeln!(fmt, "{:>30} : {}", "shadow samples", self.shadow_samples)?;
        writeln!(
            fmt,
            "{:>30} : {}",
            "samples per pixel",
            self.samples_per_pixel()
        )?;
        writeln!(fmt, "{:>30} : {}", "total samples", self.total_samples())
    }
}
