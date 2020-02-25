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

    data_dens!(emis_dens, emis);
    data_dens!(scat_dens, scats);
    data_dens!(abs_dens, abs);
    data_dens!(shift_dens, shifts);
    data_dens!(dist_trav_dens, dist_trav);
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
        self.emis_dens().save(&path.join("lm_emis_dens.nc"));
        self.scat_dens().save(&path.join("lm_scat_dens.nc"));
        self.abs_dens().save(&path.join("lm_abs_dens.nc"));
        self.shift_dens().save(&path.join("lm_shift_dens.nc"));
        self.dist_trav_dens()
            .save(&path.join("lm_dist_trav_dens.nc"));
    }
}
