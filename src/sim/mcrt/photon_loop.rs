//! Core MCRT photon loop function.

use crate::{
    geom::Trace,
    math::distribution,
    ord::{MatSet, SurfSet},
    phys::{Crossing, Environment, Photon},
    sim::mcrt::{CellRec, Grid, Hit, LightMap},
    util::ParProgressBar,
    world::Light,
};
use log::warn;
use nalgebra::Point3;
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
                    .dist(phot.ray())
                    .expect("Could not determine cell distance.");
                let inter_dist = cr.cell().inter_dist(phot.ray());

                match Hit::new(scat_dist, cell_dist, inter_dist, bump_dist) {
                    Hit::Scattering(dist) => {
                        *cr.rec_mut().dist_trav_mut() += dist;
                        phot.ray_mut().travel(dist);
                        *cr.rec_mut().abs_mut() +=
                            phot.weight() * phot.power() * env.abs_coeff() * dist;

                        *cr.rec_mut().scats_mut() += phot.weight();
                        phot.ray_mut().rotate(
                            distribution::henyey_greenstein(&mut rng, env.asym()),
                            rng.gen_range(0.0, 2.0 * PI),
                        );

                        // *cr.rec_mut().abs_mut() += env.albedo() * phot.weight();
                        *cr.rec_mut().abs_mut() +=
                            phot.weight() * phot.power() * env.abs_coeff() * dist;
                        *phot.weight_mut() *= env.albedo();

                        if !shifted && rng.gen_range(0.0, 1.0) <= env.shift_prob() {
                            *cr.rec_mut().shifts_mut() += phot.weight();
                            shifted = true;
                        }
                    }
                    Hit::Cell(dist) => {
                        let dist = dist + bump_dist;
                        *cr.rec_mut().dist_trav_mut() += dist;
                        phot.ray_mut().travel(dist);
                        *cr.rec_mut().abs_mut() +=
                            phot.weight() * phot.power() * env.abs_coeff() * dist;

                        if !grid.bound().contains(phot.ray().pos()) {
                            if !periodic_xy(&mut phot, grid.bound().mins(), grid.bound().maxs()) {
                                break;
                            }
                        }

                        cr = CellRec::new(phot.ray().pos(), grid, &mut lm);
                    }
                    Hit::Interface(dist) => {
                        hit_interface(
                            mats, bump_dist, &mut rng, &mut phot, &mut cr, &mut env, dist,
                        );

                        if !cr.cell().bound().contains(phot.ray().pos()) {
                            // TODO: This should be able to be removed.
                            if !grid.bound().contains(phot.ray().pos()) {
                                if !periodic_xy(&mut phot, grid.bound().mins(), grid.bound().maxs())
                                {
                                    break;
                                }
                            }

                            // warn!("Interface crossing caused cell crossing!");
                            cr = CellRec::new(phot.ray().pos(), grid, &mut lm);
                        }
                    }
                    Hit::InterfaceCell(dist) => {
                        hit_interface(
                            mats, bump_dist, &mut rng, &mut phot, &mut cr, &mut env, dist,
                        );

                        if !grid.bound().contains(phot.ray().pos()) {
                            if !periodic_xy(&mut phot, grid.bound().mins(), grid.bound().maxs()) {
                                break;
                            }
                        }

                        cr = CellRec::new(phot.ray().pos(), grid, &mut lm);
                    }
                }
            }
        }
    }

    lm
}

/// Create a periodic-xy boundary condition for the photons.
fn periodic_xy(phot: &mut Photon, mins: &Point3<f64>, maxs: &Point3<f64>) -> bool {
    let p = phot.ray_mut().pos_mut();
    let w = maxs - mins;

    if p.z < mins.z || p.z > maxs.z {
        return false;
    }

    while p.x < mins.x {
        p.x += w.x;
    }
    while p.x > maxs.x {
        p.x -= w.x;
    }

    while p.y < mins.y {
        p.y += w.y;
    }
    while p.y > maxs.y {
        p.y -= w.y;
    }

    true
}

/// Perform an interface hit event.
#[inline]
fn hit_interface(
    mats: &MatSet,
    bump_dist: f64,
    rng: &mut ThreadRng,
    phot: &mut Photon,
    cr: &mut CellRec,
    env: &mut Environment,
    dist: f64,
) {
    let (_, inside, norm, inter) = cr
        .cell()
        .inter_dist_inside_norm_inter(phot.ray())
        .expect("Failed to observe interface within cell.");

    let next_mat = if inside {
        inter.out_mat()
    } else {
        inter.in_mat()
    };
    let next_env = mats.get(next_mat).optics().env(phot.wavelength());

    let n_curr = env.ref_index();
    let n_next = next_env.ref_index();

    let crossing = Crossing::new(phot.ray().dir(), &norm, n_curr, n_next);

    if rng.gen_range(0.0, 1.0) <= crossing.ref_prob() {
        let effective_dist = (dist - bump_dist).max(MIN_POSITIVE);
        *cr.rec_mut().dist_trav_mut() += effective_dist;
        phot.ray_mut().travel(effective_dist);
        *cr.rec_mut().abs_mut() += phot.weight() * phot.power() * env.abs_coeff() * dist;

        *phot.ray_mut().dir_mut() = *crossing.ref_dir();
    } else {
        let effective_dist = dist + bump_dist;
        *cr.rec_mut().dist_trav_mut() += effective_dist;
        phot.ray_mut().travel(effective_dist);
        *cr.rec_mut().abs_mut() += phot.weight() * phot.power() * env.abs_coeff() * dist;

        *phot.ray_mut().dir_mut() = crossing
            .trans_dir()
            .expect("Failed to determine transmission direction.");

        *env = next_env;
    }
}
