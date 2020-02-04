//! Histogram implementation.

use crate::{access, clone, file::Save, math::Range};
use ndarray::Array1;
use std::{fs::File, io::Write, path::Path};

/// Static range, constant bin width, Histogram.
pub struct Histogram {
    /// Domain of values.
    range: Range,
    /// Width of each bin.
    bin_width: f64,
    /// Bin data.
    bins: Array1<f64>,
}

impl Histogram {
    clone!(range, Range);
    clone!(bin_width, f64);
    access!(bins, Array1<f64>);

    /// Construct a new instance
    #[inline]
    #[must_use]
    pub fn new(range: Range, num_bins: usize) -> Self {
        assert!(num_bins > 0);

        let bin_width = range.width() / num_bins as f64;

        Self {
            range,
            bin_width,
            bins: Array1::zeros(num_bins),
        }
    }

    /// Find the corresponding index of the bin data for a given value.
    #[inline]
    #[must_use]
    fn find_index(&self, x: f64) -> usize {
        assert!(self.range.contains(x));

        (((x - self.range.min()) / self.range.width()) * self.bins.len() as f64).floor() as usize
    }

    /// Increment the bin corresponding to x by unity.
    #[inline]
    pub fn collect(&mut self, x: f64) {
        assert!(self.range.contains(x));

        let index = self.find_index(x);
        *self.bins.get_mut(index).expect("Invalid index.") += 1.0;
    }

    /// Increment the bin corresponding to x by the given weight.
    #[inline]
    pub fn collect_weight(&mut self, x: f64, weight: f64) {
        assert!(self.range.contains(x));

        let index = self.find_index(x);
        *self.bins.get_mut(index).expect("Invalid index.") += weight;
    }

    /// Increment the bin corresponding to x by unity if the value of x is within the value range.
    #[inline]
    pub fn try_collect(&mut self, x: f64) {
        if !self.range.contains(x) {
            return;
        }

        self.collect(x);
    }

    /// Increment the bin corresponding to x by the given weight if the value of x is within the value range.
    #[inline]
    pub fn try_collect_weight(&mut self, x: f64, weight: f64) {
        if !self.range.contains(x) {
            return;
        }

        self.collect_weight(x, weight);
    }
}

impl Save for Histogram {
    #[inline]
    #[allow(clippy::cast_precision_loss)]
    fn save(&self, path: &Path) {
        let mut file = File::create(path).expect("Unable to create histogram file.");

        for (iter, value) in self.bins.iter().enumerate() {
            let x = (iter as f64 + 0.5).mul_add(self.bin_width, self.range.min());
            writeln!(file, "{:>31}, {:>31}", x, value).expect("Failed to write to file.");
        }
    }
}
