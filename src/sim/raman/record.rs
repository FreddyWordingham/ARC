//! Light-Map record structure.

use crate::clone;
use std::ops::AddAssign;

/// Record structure implementation.
/// Stores data about a single cell during an MCRT simulation.
#[derive(Debug, Clone)]
pub struct Record {
    /// Total weight of photon emissions.
    emissions: f64,
    /// Total weight of scattering events.
    scatters: f64,
    /// Total weight of absorption events.
    absorptions: f64,
    /// Total weight of shift events.
    shifts: f64,
    /// Total distance travelled by photons.
    dist_travelled: f64,
    ///Total weight of Raman photons detected.
    det_raman: f64,
    ///Number of Raman photons made.
    ram_laser: f64,
}

impl Record {
    clone!(emissions, emissions_mut, f64);
    clone!(scatters, scatters_mut, f64);
    clone!(absorptions, absorptions_mut, f64);
    clone!(shifts, shifts_mut, f64);
    clone!(dist_travelled, dist_travelled_mut, f64);
    clone!(det_raman, det_raman_mut, f64);
    clone!(ram_laser, ram_laser_mut, f64);
}

impl Default for Record {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self {
            emissions: 0.0,
            scatters: 0.0,
            absorptions: 0.0,
            shifts: 0.0,
            dist_travelled: 0.0,
            det_raman: 0.0,
            ram_laser: 0.0,
        }
    }
}

impl AddAssign<Self> for Record {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.emissions += rhs.emissions;
        self.scatters += rhs.scatters;
        self.absorptions += rhs.absorptions;
        self.shifts += rhs.shifts;
        self.dist_travelled += rhs.dist_travelled;
        self.det_raman += rhs.det_raman;
        self.ram_laser += rhs.ram_laser;
    }
}
