//! Grid settings implementation.

use crate::clone;
use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Grid setup settings.
#[json]
pub struct GridSettings {
    /// Collision detection padding.
    padding: f64,
    /// Maxium mesh depth.
    max_depth: u32,
    /// Target maximum number of triangles per cell.
    tar_tris: usize,
}

impl GridSettings {
    clone!(padding, f64);
    clone!(max_depth, u32);
    clone!(tar_tris, usize);
}

impl Display for GridSettings {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt)?;
        writeln!(
            fmt,
            "{:>30} : {}%",
            "collision detection padding",
            self.padding * 100.0
        )?;
        writeln!(fmt, "{:>30} : {}", "max depth", self.max_depth)?;
        writeln!(fmt, "{:>30} : {}", "target triangles", self.tar_tris)
    }
}
