//! Hit side enumeration.

use attr::json;
use nalgebra::{Unit, Vector3};

/// Side of a surface hit.
#[json]
pub enum Side {
    /// Inside of mesh hit. d.dot(n) > 0.0
    Inside,
    /// Outside of mesh hit. d.dot(n) < 0.0
    Outside,
}

impl Side {
    /// Determine the side of hit from the travel direction and the surface normal.
    #[inline]
    #[must_use]
    pub fn new(dir: &Unit<Vector3<f64>>, norm: &Unit<Vector3<f64>>) -> Self {
        if dir.dot(norm) > 0.0 {
            Self::Inside {}
        } else {
            Self::Outside {}
        }
    }

    /// Check if the side is an inside.
    #[inline]
    #[must_use]
    pub fn is_inside(&self) -> bool {
        match self {
            Self::Inside {} => true,
            Self::Outside {} => false,
        }
    }

    /// Check if the side is an outside.
    #[inline]
    #[must_use]
    pub fn is_outside(&self) -> bool {
        match self {
            Self::Inside {} => false,
            Self::Outside {} => true,
        }
    }
}
