//! Settings implementation.

use crate::clone;
use attr::json;

/// Runtime settings structure.
#[json]
pub struct Settings {
    /// Number of threads to use.
    num_threads: usize,
}

impl Settings {
    clone!(num_threads, usize);
}
