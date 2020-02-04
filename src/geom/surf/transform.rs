//! Transform trait.

use nalgebra::Similarity3;

/// Types implementing this trait may be transformed.
pub trait Transform {
    /// Apply the given transformation.
    fn transform(&mut self, trans: &Similarity3<f64>);
}
