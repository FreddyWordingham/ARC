//! Chemical kinetics simulation sub-module.

pub mod reactor;
pub mod settings;

pub use self::{reactor::*, settings::*};

use crate::ord::{ReactSet, SpecSet};
use ndarray::Array1;

/// Run a diffusion transfer simulation.
#[inline]
// #[must_use]
pub fn run(sett: &Settings, reacts: &ReactSet, specs: &SpecSet, concs: &mut Array1<f64>) {
    debug_assert!(specs.map().len() == concs.len());

    let reactor = Reactor::new(&reacts, &specs);
}
