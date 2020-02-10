//! Binner implementation.

use crate::{clone, math::Range};

/// One-dimensional binning structure.
#[derive(Debug, Clone)]
pub struct Binner {
    /// Range.
    range: Range,
    /// Total number of bins.
    bins: u64,
}

impl Binner {
    clone!(range, Range);
    clone!(bins, u64);

    /// Construct a new Range.
    #[inline]
    #[must_use]
    pub fn new(range: Range, bins: u64) -> Self {
        debug_assert!(bins > 0);

        Self { range, bins }
    }

    /// Calculate the bin width.
    #[inline]
    #[must_use]
    pub fn bin_width(&self) -> f64 {
        self.range.width() / self.bins as f64
    }

    /// Determine the corresponding bin.
    #[inline]
    #[must_use]
    pub fn bin(&self, x: f64) -> usize {
        debug_assert!(self.range.contains(x));

        ((x - self.range.min() / self.range.width()) * self.bins as f64).floor() as usize
    }

    /// Determine the corresponding bin if the value is within the range.
    #[inline]
    #[must_use]
    pub fn try_bin(&self, x: f64) -> Option<usize> {
        if self.range.contains(x) {
            Some(self.bin(x))
        } else {
            None
        }
    }
}
