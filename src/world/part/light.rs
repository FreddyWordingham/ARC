//! Light-source structure.

use crate::{
    access,
    geom::Emit,
    ord::{SurfKey, SurfSet},
    phys::{Photon, Spectrum},
};
use attr::json;
use rand::rngs::ThreadRng;

/// Light structure implementation.
#[json]
pub struct Light {
    /// Emission surface.
    surf: SurfKey,
    /// Emission spectrum.
    spec: Spectrum,
    /// Power. [J/s]
    power: f64,
}

impl Light {
    access!(surf, SurfKey);
    access!(spec, Spectrum);
    access!(power, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(surf: SurfKey, spec: Spectrum, power: f64) -> Self {
        debug_assert!(power > 0.0);

        Self { surf, spec, power }
    }

    /// Emit a new photon.
    pub fn emit(&self, rng: &mut ThreadRng, total_phot: u64, surfs: &SurfSet) -> Photon {
        Photon::new(
            self.spec.sample(rng),
            self.power / total_phot as f64,
            surfs.get(&self.surf).cast(rng),
        )
    }
}
