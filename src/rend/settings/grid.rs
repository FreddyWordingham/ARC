//! Grid input settings implementation.

use crate::clone;
use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Grid settings.
#[json]
pub struct Grid {
    /// Target maximum number of triangles per cell.
    tar_tris: usize,
    /// Maximum mesh depth.
    max_depth: i32,
    /// Collision detection padding.
    padding: f64,
}

impl Grid {
    clone!(tar_tris, usize);
    clone!(max_depth, i32);
    clone!(padding, f64);
}

impl Display for Grid {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt)?;
        writeln!(fmt, "{:>30} : {}", "target triangles", self.tar_tris)?;
        writeln!(fmt, "{:>30} : {}", "max depth", self.max_depth)?;
        writeln!(
            fmt,
            "{:>30} : {}%",
            "collision detection padding",
            self.padding * 100.0
        )
    }
}
