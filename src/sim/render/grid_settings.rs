//! Cell implementation.

use crate::clone;
use attr::json;

/// Settings used to construct the adaptive render grid.
#[json]
pub struct GridSettings {
    /// Minimum splitting depth.
    max_depth: usize,
    /// Maximum splitting depth.
    min_depth: usize,
    /// Maximum target number of triangles.
    max_tris: usize,
}

impl GridSettings {
    clone!(max_depth, usize);
    clone!(min_depth, usize);
    clone!(max_tris, usize);
}
