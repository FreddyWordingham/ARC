//! Hit implementation.

use crate::{access, clone, rend::Group};
use nalgebra::{Unit, Vector3};

/// Hit collision information.
#[derive(Clone)]
pub struct Hit {
    /// Group hit.
    group: Group,
    /// Distance to the hit.
    dist: f64,
    /// Normal of the surface.
    norm: Unit<Vector3<f64>>,
}

impl Hit {
    clone!(group, Group);
    clone!(dist, f64);
    access!(norm, Unit<Vector3<f64>>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(group: Group, dist: f64, norm: Unit<Vector3<f64>>) -> Self {
        debug_assert!(dist > 0.0);

        Self { group, dist, norm }
    }
}
