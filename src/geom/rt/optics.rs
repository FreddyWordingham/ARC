//! Optical functions.

use nalgebra::{Unit, Vector3};

/// Calculate the reflection unit vector.
pub fn reflect(in_dir: &Unit<Vector3<f64>>, norm: &Unit<Vector3<f64>>) -> Unit<Vector3<f64>> {
    Unit::new_normalize(in_dir.into_inner() - (norm.as_ref() * (2.0 * in_dir.dot(&norm))))
}
