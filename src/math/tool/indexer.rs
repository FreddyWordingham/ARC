//! Indexer implementation.

use crate::{clone, math::Range};

/// One-dimensional indexing structure.
#[derive(Debug, Clone)]
pub struct Indexer {
    /// Range.
    range: Range,
    /// Total number of bins.
    bins: u64,
}

impl Indexer {
    clone!(range, Range);
    clone!(bins, u64);

    /// Construct a new Range.
    #[inline]
    #[must_use]
    pub fn new(range: Range, bins: u64) -> Self {
        assert!(bins > 0);

        Self { range, bins }
    }

    /// Calculate the bin width.
    #[inline]
    #[must_use]
    pub fn bin_width(&self) -> f64 {
        self.range.width() / self.bins as f64
    }

    /// Determine the corresponding index.
    #[inline]
    #[must_use]
    pub fn index(&self, x: f64) -> usize {
        assert!(self.range.contains(x));

        ((x - self.range.min() / self.range.width()) * self.bins as f64).floor() as usize
    }

    /// Determine the corresponding index if the value is within the range.
    #[inline]
    #[must_use]
    pub fn try_index(&self, x: f64) -> Option<usize> {
        if self.range.contains(x) {
            Some(self.index(x))
        } else {
            None
        }
    }
}
