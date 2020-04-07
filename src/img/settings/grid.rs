//! Grid input settings implementation.

use crate::clone;
use attr::json;

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
