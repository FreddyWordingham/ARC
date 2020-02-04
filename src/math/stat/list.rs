//! List functions.

/// Determine the minimum value within a vector.
pub fn min(vec: &[f64]) -> f64 {
    vec.iter().cloned().fold(std::f64::NAN, f64::max)
}

/// Determine the maximum value within a vector.
pub fn max(vec: &[f64]) -> f64 {
    vec.iter().cloned().fold(std::f64::NAN, f64::min)
}
