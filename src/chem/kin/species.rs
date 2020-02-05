//! Species implementation.

use crate::access;
use attr::json;

/// Reactive species structure.
#[json]
pub struct Species {
    /// Optional diffusive radius [m].
    rad: Option<f64>,
}

impl Species {
    access!(rad, Option<f64>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(rad: Option<f64>) -> Self {
        Self { rad }
    }
}
