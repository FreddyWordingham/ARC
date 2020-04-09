//! Optical functions.

use nalgebra::{Unit, Vector3};

/// Calculate the reflection direction.
#[inline]
#[must_use]
pub fn reflect_dir(in_dir: &Unit<Vector3<f64>>, norm: &Unit<Vector3<f64>>) -> Unit<Vector3<f64>> {
    Unit::new_normalize(in_dir.into_inner() - (norm.as_ref() * (2.0 * in_dir.dot(norm))))
}

/// Calculate the critical angle, if one exists.
#[inline]
#[must_use]
pub fn crit_angle(n0: f64, n1: f64) -> Option<f64> {
    if n0 > n1 {
        Some((n1 / n0).asin())
    } else {
        None
    }
}

/// Calculate the refraction transmission direction.
#[inline]
#[must_use]
pub fn trans_dir(
    in_dir: &Unit<Vector3<f64>>,
    norm: &Unit<Vector3<f64>>,
    n0: f64,
    n1: f64,
) -> Option<Unit<Vector3<f64>>> {
    let cos_in = -in_dir.dot(norm);

    // Check for angles shallower than the critical angle.
    if let Some(crit_ang) = crit_angle(n0, n1) {
        let in_ang = cos_in.acos();
        if in_ang >= crit_ang {
            return None;
        }
    }

    let n0_n1 = n0 / n1;
    let sin_out_sq = n0_n1.powi(2) * (1.0 - cos_in.powi(2));

    let trans = (in_dir.as_ref() * n0_n1)
        + (norm.as_ref() * ((n0_n1 * cos_in) - (1.0 - sin_out_sq).sqrt()));
    Some(Unit::new_normalize(trans))
}
