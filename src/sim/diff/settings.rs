//! Settings implementation.

use crate::clone;
use attr::json;

/// Runtime settings structure.
#[json]
pub struct Settings {
    /// Total integration time.
    time: f64,
    /// Minimum timestep.
    min_timestep: f64,
}

impl Settings {
    clone!(time, f64);
    clone!(min_timestep, f64);
}
