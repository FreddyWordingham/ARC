//! Core MCRT photon loop function.

use crate::{
    list::Cartesian::{X, Y, Z},
    phys::Photon,
    sim::Camera,
    util::ParProgressBar,
    world::{Cell, Grid, Verse},
};
use log::warn;
use nalgebra::Point3;
use ndarray::Array2;
use std::sync::{Arc, Mutex};

/// Maximum number of loops a photon will make before being culled prematurely.
const MAX_LOOPS: u64 = 1_000;

/// Run a single threaded instance of the imagine loop.
#[inline]
#[must_use]
pub fn run_thread(
    _id: usize,
    cam: &Camera,
    verse: &Verse,
    grid: &Grid,
    pb: &Arc<Mutex<ParProgressBar>>,
    block_size: u64,
) -> Array2<f64> {
    // let bump_dist = grid.bump_dist();

    let mut img = Array2::zeros(cam.res());

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        for n in start..end {
            let xi = n as usize % cam.res().0;
            let yi = n as usize / cam.res().0;

            let ray = cam.gen_ray(xi, yi);
            let mut phot = Photon::new(630.0e-9, 1.0, ray);

            let cell = find_cell(phot.ray().pos(), grid);

            let mut env = verse.mats().get(cell.mat()).optics().env(phot.wavelength());

            let mut num_loops = 0;
            loop {
                debug_assert!(phot.weight() > 0.0);

                num_loops += 1;
                if num_loops >= MAX_LOOPS {
                    warn!(
                        "Photon prematurely killed as number of loops exceeded {}",
                        MAX_LOOPS
                    );
                    break;
                }
            }

            // let mut phot = light.emit(&mut rng, total_phot, verse.surfs());
        }
    }

    img
}

/// Determine the cell containing a given position.
fn find_cell<'a>(pos: &Point3<f64>, grid: &'a Grid) -> &'a Cell<'a> {
    debug_assert!(grid.bound().contains(pos));

    let mins = grid.bound().mins();
    let maxs = grid.bound().maxs();
    let resolution = grid.res();

    let id: Vec<usize> = pos
        .iter()
        .zip(mins.iter().zip(maxs.iter()))
        .zip(&resolution)
        .map(|((p, (min, max)), n)| (((p - min) / (max - min)) * *n as f64) as usize)
        .collect();
    let index = (
        *id.get(X as usize).expect("Missing index."),
        *id.get(Y as usize).expect("Missing index."),
        *id.get(Z as usize).expect("Missing index."),
    );

    grid.cells().get(index).expect("Invalid grid index.")
}
