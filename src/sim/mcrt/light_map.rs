//! Light-Map structure.

use crate::{
    access, clone,
    sim::mcrt::{Grid, Record},
};
use ndarray::Array3;
use std::ops::AddAssign;

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
}

impl AddAssign<&Self> for LightMap {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        debug_assert!((self.cell_vol - rhs.cell_vol).abs() < 1.0e-9);

        self.recs += &rhs.recs;
    }
}

// impl Save for LightMap {
//     fn save(&self, path: &Path) {
//         self.emission_dens()
//             .save(&path.join("lightmap_emission_dens.nc"));
//         self.scatter_dens()
//             .save(&path.join("lightmap_scatter_dens.nc"));
//         self.absorption_dens()
//             .save(&path.join("lightmap_absorption_dens.nc"));
//         self.shift_dens().save(&path.join("lightmap_shift_dens.nc"));
//         self.dist_travelled_dens()
//             .save(&path.join("lightmap_dist_travelled_dens.nc"));
//     }
// }
