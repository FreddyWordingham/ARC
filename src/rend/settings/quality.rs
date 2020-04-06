//! Image quality settings implementation.

use crate::clone;
use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Quality settings.
#[json]
pub struct Quality {
    /// Super samples.
    super_samples: i32,
    /// Depth of field samples.
    dof_samples: i32,
    /// Shadow samples.
    shadow_samples: i32,
}

impl Quality {
    clone!(super_samples, i32);
    clone!(dof_samples, i32);
    clone!(shadow_samples, i32);

    /// Calculate the number of samples expected per pixel.
    #[inline]
    #[must_use]
    pub fn samples_per_pixel(&self) -> i32 {
        self.super_samples * self.dof_samples * self.shadow_samples
    }
}

impl Display for Quality {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt)?;
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
        )
    }
}
