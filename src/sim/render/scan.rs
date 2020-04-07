//! Hit scan implementation.

use crate::sim::render::Hit;

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
    /// Surface and cell boundary collision within the bump distance.
    Both {
        /// Surface hit information.
        hit: Hit,
        /// Boundary distance.
        dist: f64,
    },
}
