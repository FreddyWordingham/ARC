//! Core MCRT photon loop function.

use crate::{
    geom::Trace,
    math::distribution,
    phys::{Crossing, Environment, Photon},
    sim::mcrt::{Grid, LightMap},
    util::ParProgressBar,
    world::Light,
};
use log::warn;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::{
    f64::{consts::PI, MIN_POSITIVE},
    sync::{Arc, Mutex},
};

/// Maximum number of loops a photon will make before being culled prematurely.
const MAX_LOOPS: u64 = 1_000_000;

/// Weight below which to perform roulette each photon loop.
const ROULETTE: f64 = 0.1;

/// Run a single threaded instance of the photon loop.
#[inline]
#[must_use]
pub fn run_thread(
    pb: &Arc<Mutex<ParProgressBar>>,
    block_size: u64,
    light: &Light,
    grid: &Grid,
) -> LightMap {
    let mut lm = LightMap::new(grid);

    

    lm
}
