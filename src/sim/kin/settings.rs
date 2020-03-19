//! Settings implementation.

use crate::clone;
use attr::json;

/// Runtime settings structure.
#[json]
pub struct Settings {
    /// Total integration time.
    time: f64,
    /// Maximum concentration fraction delta that can occur within one step.
    max_conc_frac_delta: f64,
    /// Minimum timestep.
    min_timestep: f64,
}

impl Settings {
    clone!(time, f64);
    clone!(max_conc_frac_delta, f64);
    clone!(min_timestep, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(time: f64, max_conc_frac_delta: f64, min_timestep: f64) -> Self {
        debug_assert!(time > 0.0);
        debug_assert!(max_conc_frac_delta > 0.0);
        debug_assert!(max_conc_frac_delta <= 1.0);
        debug_assert!(min_timestep > 0.0);

        Self {
            time,
            max_conc_frac_delta,
            min_timestep,
        }
    }
}
