//! Core MCRT photon loop function.

use crate::{
    sim::{LightMap, Record},
    util::ParProgressBar,
    world::{Cell, Grid, Light, Verse},
};
use nalgebra::Point3;
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

            let cell_rec = cell_and_record(phot.ray().pos(), grid, &mut lm);
            *cell_rec.1.emissions_mut() += phot.weight();

            // let env = cell
        }
    }

    lm
}

/// Retrieve a reference to the current cell, and corresponding record, that a point belongs to.
#[inline]
#[must_use]
fn cell_and_record<'a>(
    pos: &Point3<f64>,
    grid: &'a Grid,
    light_map: &'a mut LightMap,
) -> (&'a Cell<'a>, &'a mut Record) {
    let mins = grid.bound().mins();
    let maxs = grid.bound().maxs();
    let res = grid.res();

    let id: Vec<usize> = pos
        .iter()
        .zip(mins.iter().zip(maxs.iter()))
        .zip(&res)
        .map(|((p, (min, max)), n)| index(*p, *min, *max, *n))
        .collect();
    let index = (
        *id.get(0).expect("Missing index."),
        *id.get(1).expect("Missing index."),
        *id.get(2).expect("Missing index."),
    );

    let cell = grid.cells().get(index).expect("Invalid grid index.");
    let rec = light_map
        .recs_mut()
        .get_mut(index)
        .expect("Invalid record index.");

    assert!(cell.bound().contains(pos));

    (cell, rec)
}

/// Determine the index corresponding to a given resolution.
#[inline]
#[must_use]
pub fn index(x: f64, min: f64, max: f64, res: usize) -> usize {
    (((x - min) / (max - min)) * res as f64) as usize
}
