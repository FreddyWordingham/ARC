//! Light-Map structure.

use crate::sim::Record;
use ndarray::Array3;
use std::ops::AddAssign;

/// Light-Map structure implementation.
/// Stores output data from an MCRT simulation.
#[derive(Debug)]
pub struct LightMap {
    /// Record array.
    pub recs: Array3<Record>,
    /// Cell volume [m^3].
    cell_vol: f64,
}

impl LightMap {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 3], cell_vol: f64) -> Self {
        assert!(res.iter().all(|x| *x > 0));
        assert!(cell_vol > 0.0);

        Self {
            recs: Array3::default(res),
            cell_vol,
        }
    }
}

impl AddAssign<&Self> for LightMap {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        assert!((self.cell_vol - rhs.cell_vol).abs() < 1.0e-9);

        self.recs += &rhs.recs;
    }
}
