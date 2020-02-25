//! Light-Map record structure.

use crate::clone;
use std::ops::AddAssign;

/// Record structure implementation.
/// Stores data about a single cell during an MCRT simulation.
#[derive(Debug, Clone)]
pub struct Record {
    /// Total weight of photon emissions.
    emis: f64,
    /// Total weight of scattering events.
    scats: f64,
    /// Total weight of absorption events.
    abs: f64,
    /// Total weight of shift events.
    shifts: f64,
    /// Total distance travelled by photons.
    dist_trav: f64,
}

impl Record {
    clone!(emis, emis_mut, f64);
    clone!(scats, scats_mut, f64);
    clone!(abs, abs_mut, f64);
    clone!(shifts, shifts_mut, f64);
    clone!(dist_trav, dist_trav_mut, f64);
}

impl Default for Record {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self {
            emis: 0.0,
            scats: 0.0,
            abs: 0.0,
            shifts: 0.0,
            dist_trav: 0.0,
        }
    }
}

impl AddAssign<Self> for Record {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.emis += rhs.emis;
        self.scats += rhs.scats;
        self.abs += rhs.abs;
        self.shifts += rhs.shifts;
        self.dist_trav += rhs.dist_trav;
    }
}
