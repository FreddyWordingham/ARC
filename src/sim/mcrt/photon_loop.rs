//! Core MCRT photon loop function.

use crate::{
    geom::Trace,
    math::distribution,
    ord::{MatSet, SurfSet},
    phys::{Crossing, Environment, Photon},
    sim::mcrt::{CellRec, Grid, LightMap},
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
    num_phot: u64,
    light: &Light,
    grid: &Grid,
    surfs: &SurfSet,
    mats: &MatSet,
) -> LightMap {
    let bump_dist = grid.bump_dist();

    let mut lm = LightMap::new(grid);
    let mut rng = thread_rng();

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        for _ in start..end {
            let mut phot = light.emit(&mut rng, num_phot, surfs);

            debug_assert!(grid.bound().contains(phot.ray().pos()));

            let mut shifted = false;

            let mut cr = CellRec::new(phot.ray().pos(), grid, &mut lm);
            *cr.rec_mut().emis_mut() += phot.weight();

            let mut env = mats.get(cr.cell().mat()).optics().env(phot.wavelength());

            let mut num_loops = 0;
            loop {
                debug_assert!(phot.weight() > 0.0);

                num_loops += 1;
                if num_loops >= MAX_LOOPS {
                    warn!(
                        "Photon prematurely killed as number of loops exceeded {}",
                        MAX_LOOPS
                    );
                }
            }
        }
    }

    lm
}
