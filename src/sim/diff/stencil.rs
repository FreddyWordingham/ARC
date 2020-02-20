//! Stencil-type enumeration.

use ndarray::Array3;

/// Stencil enumeration implementation.
#[derive(Debug)]
pub enum Stencil {
    /// Gradient.
    Gradient {},
}

impl Stencil {
    pub fn new_gradient(index: [usize; 3], concs: &Array3<f64>) -> Self {
        Self::Gradient {}
    }
}
