//! Material implementation.

use crate::{access, ord::InterSet};

/// Material physical properties.
pub struct Verse {
    // /// Surfaces.
    // surfs: SurfSet,
    /// Interfaces.
    inters: InterSet,
}

impl Verse {
    access!(inters, InterSet);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(inters: InterSet) -> Self {
        Self { inters }
    }
}
