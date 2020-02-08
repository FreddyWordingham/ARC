//! Core MCRT photon loop function.

use crate::{
    sim::{CellRec, LightMap},
    util::ParProgressBar,
    world::{Grid, Light, Verse},
};
use rand::thread_rng;
use std::sync::{Arc, Mutex};

/// Run a single threaded instance of the photon loop.
#[inline]
#[must_use]
pub fn run_thread(
    _id: usize,
    verse: &Verse,
    grid: &Grid,
    light: &Light,
    total_phot: u64,
    pb: &Arc<Mutex<ParProgressBar>>,
    block_size: u64,
) -> LightMap {
    let mut lm = LightMap::new(grid);
    let mut rng = thread_rng();

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        for _ in start..end {
            let phot = light.emit(&mut rng, total_phot, verse.surfs());

            assert!(grid.bound().contains(phot.ray().pos()));

            let _shifted = false;

            let mut cr = CellRec::new(phot.ray().pos(), grid, &mut lm);
            *cr.rec_mut().emissions_mut() += phot.weight();

            // let env = cell
        }
    }

    lm
}
