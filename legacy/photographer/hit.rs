//! Hit-type enumeration.

use crate::ord::InterKey;

/// Hit enumeration implementation.
#[derive(Debug)]
pub enum Hit {
    /// Cell collision.
    Cell(f64),
    /// Target collision.
    Target(f64),
    /// Sea collision.
    Refract(f64),
    /// Scenery collision.
    Scene(f64),
}

impl Hit {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(cell_dist: f64, inter_dist_key: Option<(f64, &InterKey)>, bump_dist: f64) -> Self {
        if let Some((inter_dist, inter_key)) = inter_dist_key {
            if inter_dist < (cell_dist + bump_dist) {
                if inter_key == &InterKey::new("whale") {
                    return Self::Target(inter_dist);
                }

                if inter_key == &InterKey::new("sea") {
                    return Self::Refract(inter_dist);
                }

                return Self::Scene(inter_dist);
            }

            return Self::Cell(cell_dist);
        }

        Self::Cell(cell_dist)
    }
}
