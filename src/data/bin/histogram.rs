//! Histogram implementation.

use crate::{
    access,
    file::Save,
    math::{Indexer, Range},
};
use ndarray::Array1;
use std::{fs::File, io::Write, path::Path};

/// Static range, constant bin width, Histogram.
pub struct Histogram {
    /// Indexer.
    indexer: Indexer,
    /// Count data.
    counts: Array1<f64>,
}

impl Histogram {
    access!(indexer, Indexer);
    access!(counts, Array1<f64>);

    /// Construct a new instance
    #[inline]
    #[must_use]
    pub fn new(min: f64, max: f64, bins: u64) -> Self {
        assert!(min < max);
        assert!(bins > 0);

        Self {
            indexer: Indexer::new(Range::new(min, max), bins),
            counts: Array1::zeros(bins as usize),
        }
    }

    /// Increment the bin corresponding to x by unity.
    #[inline]
    pub fn collect(&mut self, x: f64) {
        assert!(self.indexer.range().contains(x));

        let index = self.indexer.index(x);
        *self.counts.get_mut(index).expect("Invalid index.") += 1.0;
    }

    /// Increment the bin corresponding to x by a given weight.
    #[inline]
    pub fn collect_weight(&mut self, x: f64, weight: f64) {
        assert!(self.indexer.range().contains(x));
        assert!(weight > 0.0);

        let index = self.indexer.index(x);
        *self.counts.get_mut(index).expect("Invalid index.") += weight;
    }

    /// Increment the bin corresponding to x by unity if x is contained within the range.
    #[inline]
    pub fn try_collect(&mut self, x: f64) {
        if let Some(index) = self.indexer.try_index(x) {
            *self.counts.get_mut(index).expect("Invalid index.") += 1.0;
        }
    }

    /// Increment the bin corresponding to x by unity if x is contained within the range.
    #[inline]
    pub fn try_collect_weight(&mut self, x: f64, weight: f64) {
        if let Some(index) = self.indexer.try_index(x) {
            *self.counts.get_mut(index).expect("Invalid index.") += weight;
        }
    }
}

impl Save for Histogram {
    #[inline]
    #[allow(clippy::cast_precision_loss)]
    fn save(&self, path: &Path) {
        let mut file = File::create(path).expect("Unable to create histogram file.");

        let bin_width = self.indexer.bin_width();
        let min = self.indexer.range().min();

        for (iter, value) in self.counts.iter().enumerate() {
            let x = (iter as f64 + 0.5).mul_add(bin_width, min);
            writeln!(file, "{:>31}, {:>31}", x, value).expect("Failed to write to file.");
        }
    }
}
