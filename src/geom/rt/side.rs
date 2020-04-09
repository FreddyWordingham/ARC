//! Hit side enumeration.

use attr::json;
use nalgebra::{Unit, Vector3};

/// Side of a surface hit.
#[json]
#[derive(Clone)]
pub enum Side {
    /// Inside of mesh hit. d.dot(n) > 0.0
    Inside {
        // Facing surface normal vector.
        norm: Unit<Vector3<f64>>,
    },
    /// Outside of mesh hit. d.dot(n) < 0.0
    Outside {
        // Facing surface normal vector.
        norm: Unit<Vector3<f64>>,
    },
}

impl Side {
    /// Reference the normal vector.
    #[inline]
    #[must_use]
    pub fn norm(&self) -> &Unit<Vector3<f64>> {
        match self {
            Self::Inside { norm } => norm,
            Self::Outside { norm } => norm,
        }
    }

    /// Check if the side is an inside.
    #[inline]
    #[must_use]
    pub fn is_inside(&self) -> bool {
        match self {
            Self::Inside { .. } => true,
            Self::Outside { .. } => false,
        }
    }

    /// Check if the side is an outside.
    #[inline]
    #[must_use]
    pub fn is_outside(&self) -> bool {
        match self {
            Self::Inside { .. } => false,
            Self::Outside { .. } => true,
        }
    }
}
