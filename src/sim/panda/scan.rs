//! Hit scan implementation.

use crate::sim::panda::Hit;

/// Hit scan result enumeration.
pub enum Scan {
    /// Surface hit.
    Surface {
        /// Hit information.
        hit: Hit,
    },
    /// Cell boundary.
    Boundary {
        /// Distance.
        dist: f64,
    },
}

impl Scan {
    /// Construct a new surface scan result instance.
    pub fn new_surface_scan(hit: Hit) -> Self {
        Self::Surface { hit }
    }

    /// Construct a new boundary scan result instance.
    pub fn new_boundary_scan(dist: f64) -> Self {
        debug_assert!(dist > 0.0);

        Self::Boundary { dist }
    }
}
