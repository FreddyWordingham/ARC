//! Settings implementation.

use crate::{access, clone, ord::LightKey};
use attr::json;

/// Runtime settings structure.
#[json]
pub struct Settings {
    /// Number of photons.
    num_phot: u64,
    /// Light to emit from.
    light: LightKey,
}

impl Settings {
    clone!(num_phot, u64);
    access!(light, LightKey);
}
