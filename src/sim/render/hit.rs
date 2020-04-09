//! Hit implementation.

use crate::{access, clone, geom::Side, sim::render::Group};

/// Hit collision information.
#[derive(Clone)]
pub struct Hit {
    /// Group hit.
    group: Group,
    /// Distance to the hit.
    dist: f64,
    /// Normal of the surface.
    side: Side,
}

impl Hit {
    clone!(group, Group);
    clone!(dist, dist_mut, f64);
    access!(side, Side);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(group: Group, dist: f64, side: Side) -> Self {
        debug_assert!(dist > 0.0);

        Self { group, dist, side }
    }
}
