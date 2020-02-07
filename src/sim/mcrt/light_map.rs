//! Light-Map structure.

use crate::{sim::Record, world::Grid};
use ndarray::Array3;
use std::ops::AddAssign;

macro_rules! data_dens {
    ($dens_func: ident, $prop: ident) => {
        /// Create a density data-cube of the lightmap's records.
        #[inline]
        #[must_use]
        pub fn $dens_func(&self) -> Array3<f64> {
            self.recs.map(|rec| rec.$prop / self.cell_vol)
        }
    };
}

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
    pub fn new(grid: &Grid) -> Self {
        let res = grid.cells().shape();
        let cell_vol = grid.bound().vol() / grid.cells().len() as f64;

        Self {
            recs: Array3::default([res[0], res[1], res[2]]),
            cell_vol,
        }
    }

    /// Create a list of density mappings.
    #[inline]
    #[must_use]
    pub fn dens_maps(&self) -> Vec<(&str, Array3<f64>)> {
        vec![
            ("emission dens", self.emission_dens()),
            ("scatter dens", self.scatter_dens()),
            ("absorption dens", self.absorption_dens()),
            ("shift dens", self.shift_dens()),
            ("dist travelled dens", self.dist_travelled_dens()),
        ]
    }

    data_dens!(emission_dens, emissions);
    data_dens!(scatter_dens, scatters);
    data_dens!(absorption_dens, absorptions);
    data_dens!(shift_dens, shifts);
    data_dens!(dist_travelled_dens, dist_travelled);
}

impl AddAssign<&Self> for LightMap {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        assert!((self.cell_vol - rhs.cell_vol).abs() < 1.0e-9);

        self.recs += &rhs.recs;
    }
}
