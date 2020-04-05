//! Light-Map structure.

use crate::{
    access, clone,
    file::Save,
    sim::raman::{Grid, Record},
};
use ndarray::Array3;
use std::{ops::AddAssign, path::Path};

macro_rules! data_dens {
    ($dens_func: ident, $prop: ident) => {
        /// Create a density data-cube of the lightmap's records.
        #[inline]
        #[must_use]
        pub fn $dens_func(&self) -> Array3<f64> {
            self.recs.map(|rec| rec.$prop() / self.cell_vol)
        }
    };
}

/// Light-Map structure implementation.
/// Stores output data from an MCRT simulation.
#[derive(Debug)]
pub struct LightMap {
    /// Record array.
    recs: Array3<Record>,
    /// Cell volume [m^3].
    cell_vol: f64,
}

impl LightMap {
    access!(recs, recs_mut, Array3<Record>);
    clone!(cell_vol, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(grid: &Grid) -> Self {
        let cell_vol = grid.bound().vol() / grid.cells().len() as f64;

        Self {
            recs: Array3::default(grid.res()),
            cell_vol,
        }
    }

    data_dens!(emissions_dens, emissions);
    data_dens!(scatters_dens, scatters);
    data_dens!(absorptions_dens, absorptions);
    data_dens!(shift_dens, shifts);
    data_dens!(dist_travelled_dens, dist_travelled);
}

impl AddAssign<&Self> for LightMap {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        debug_assert!((self.cell_vol - rhs.cell_vol).abs() < 1.0e-9);

        self.recs += &rhs.recs;
    }
}

impl Save for LightMap {
    fn save(&self, path: &Path) {
        self.emissions_dens()
            .save(&path.join("lm_emissions_dens.nc"));
        self.scatters_dens().save(&path.join("lm_scatters_dens.nc"));
        self.absorptions_dens()
            .save(&path.join("lm_absorptions_dens.nc"));
        self.shift_dens().save(&path.join("lm_shift_dens.nc"));
        self.dist_travelled_dens()
            .save(&path.join("lm_dist_travelled_dens.nc"));
    }
}
