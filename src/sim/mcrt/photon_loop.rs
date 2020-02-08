//! Core MCRT photon loop function.

use crate::{
    geom::Trace,
    math::distribution,
    sim::{CellRec, Hit, LightMap},
    util::ParProgressBar,
    world::{Grid, Light, Verse},
};
use log::warn;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

/// Maximum number of loops a photon will make before being culled prematurely.
const MAX_LOOPS: u64 = 1_000;

/// Weight below which to perform roulette each photon loop.
const ROULETTE: f64 = 0.1;

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
            let mut phot = light.emit(&mut rng, total_phot, verse.surfs());

            assert!(grid.bound().contains(phot.ray().pos()));

            let mut shifted = false;

            let mut cr = CellRec::new(phot.ray().pos(), grid, &mut lm);
            *cr.rec_mut().emissions_mut() += phot.weight();

            let mut env = verse
                .mats()
                .get(cr.cell().mat())
                .optics()
                .env(phot.wavelength());

            let mut num_loops = 0;
            loop {
                assert!(phot.weight() > 0.0);

                num_loops += 1;
                if num_loops >= MAX_LOOPS {
                    warn!(
                        "Photon prematurely killed as number of loops exceeded {}",
                        MAX_LOOPS
                    );
                }

                if phot.weight() < ROULETTE {
                    if rng.gen_range(0.0_f64, 1.0) <= ROULETTE {
                        *phot.weight_mut() /= ROULETTE;
                    } else {
                        break;
                    }
                }

                let scat_dist = -(rng.gen_range(0.0_f64, 1.0)).ln() / env.inter_coeff();
                let cell_dist = cr
                    .cell()
                    .bound()
                    .dist(&phot.ray())
                    .expect("Unable to determine boundary distance.");
                let inter_dist = cr.cell().inter_dist(phot.ray());

                match Hit::new(scat_dist, cell_dist, inter_dist, bump_dist) {
                    Hit::Scattering(dist) => {
                        *cr.rec_mut().dist_travelled_mut() += dist;
                        phot.ray_mut().travel(dist);

                        *cr.rec_mut().scatters_mut() += phot.weight();
                        phot.ray_mut().rotate(
                            distribution::henyey_greenstein(&mut rng, env.asym()),
                            rng.gen_range(0.0, 2.0 * PI),
                        );

                        *cr.rec_mut().absorptions_mut() += env.albedo() * phot.weight();
                        *phot.weight_mut() *= env.albedo();

                        if !shifted && rng.gen_range(0.0, 1.0) <= env.shift_prob() {
                            *cr.rec_mut().shifts_mut() += phot.weight();
                            shifted = true;
                        }
                    }
                    Hit::Cell(dist) => {
                        let dist = dist + bump_dist;
                        *cr.rec_mut().dist_travelled_mut() += dist;
                        phot.ray_mut().travel(dist);

                        if !grid.bound().contains(phot.ray().pos()) {
                            break;
                        }

                        cr = CellRec::new(phot.ray().pos(), grid, &mut lm);
                    }
                    Hit::Interface(dist) => {
                        phot.ray_mut().travel(dist + bump_dist);
                    }
                    Hit::InterfaceCell(dist) => {
                        phot.ray_mut().travel(dist + bump_dist);
                    }
                }
            }
        }
    }

    lm
}
