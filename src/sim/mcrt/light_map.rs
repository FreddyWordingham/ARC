//! Light-Map structure.

use crate::{file::Save, sim::Record, world::Grid};
use ndarray::Array3;
use std::{ops::AddAssign, path::Path};

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
        let res = grid.res();
        let cell_vol = grid.bound().vol() / grid.cells().len() as f64;

        Self {
            recs: Array3::default(res),
            cell_vol,
        }
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

impl Save for LightMap {
    fn save(&self, path: &Path) {
        self.emission_dens()
            .save(&path.join("lightmap_emission_dens.nc"));
        self.scatter_dens()
            .save(&path.join("lightmap_scatter_dens.nc"));
        self.absorption_dens()
            .save(&path.join("lightmap_absorption_dens.nc"));
        self.shift_dens().save(&path.join("lightmap_shift_dens.nc"));
        self.dist_travelled_dens()
            .save(&path.join("lightmap_dist_travelled_dens.nc"));
    }
}
