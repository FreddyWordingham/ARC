//! Golden ratio sampling functions.

use lazy_static::lazy_static;
use std::f64::consts::{FRAC_PI_2, PI};

lazy_static! {
    /// Golden ratio constant.
    static ref GOLDEN_RATIO: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;
}

/// Sample points within a circle using the golden ratio.
#[inline]
#[must_use]
pub fn circle(n: i64, max: i64) -> (f64, f64) {
    debug_assert!(n >= 0);
    debug_assert!(n < max);

    let r = n as f64 / (max - 1) as f64;
    let theta = n as f64 * *GOLDEN_RATIO;

    (r, theta)
}

/// Sample points on a sphere's surface using the golden ratio.
#[inline]
#[must_use]
pub fn sphere(n: i64, max: i64) -> (f64, f64) {
    debug_assert!(n >= 0);
    debug_assert!(n < max);

    let d = n as f64 + (((1 - max) as f64) * 0.5);
    let phi = ((2.0 * d) / max as f64).asin() + FRAC_PI_2;
    let theta = ((2.0 * PI) / *GOLDEN_RATIO) * (d % *GOLDEN_RATIO);

    (phi, theta)
}
