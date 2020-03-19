//! Core MCRT photon loop function.

use crate::{
    geom::Trace,
    list::Cartesian::{X, Y, Z},
    math::distribution,
    ord::{MatKey, MatSet, SurfSet},
    phys::{Crossing, Environment, Photon},
    sim::raman::{Cell, CellRec, Grid, Hit, LightMap},
    util::ParProgressBar,
    world::Light,
};
use log::warn;
use nalgebra::{Point3, Unit};
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
#[allow(clippy::too_many_lines)]
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
    let mut extra_phot: Option<Photon> = None;
    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        //for _ in start..end {
        let mut total = end - start;
        //println!("Start and end: {}, {}", start, end);
        //println!("total: {}", total);
        while total > 0 {
<<<<<<< Updated upstream
=======
            let mut _shifted = false;
>>>>>>> Stashed changes
            let mut mat = None;
            let mut phot = if let Some(phot) = extra_phot {
                total += 1;
                extra_phot = None;
                //println!("cont phot!");
                mat = Some(MatKey::new("ptfe"));
                phot
            } else {
                total -= 1;
                //println!("From light source");
                light.emit(&mut rng, num_phot, surfs)
            };

            debug_assert!(grid.bound().contains(phot.ray().pos()));

            let mut cr = CellRec::new(phot.ray().pos(), grid, &mut lm);
            *cr.rec_mut().emis_mut() += phot.weight();

            #[allow(unused_assignments)]
            // This is here to suppress a warning. I think it's a bug though so check it. - Freddy
            let mut env = if let Some(mat) = mat {
                mats.get(&mat).optics().env(phot.wavelength())
            } else {
                mats.get(cr.cell().mat()).optics().env(phot.wavelength())
            };

            //println!("abs coeff: {}", env.abs_coeff());

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
<<<<<<< Updated upstream
                        // if shifted { println!("Abs coeff: {}", env.abs_coeff()); };
=======
                        //if shifted == true {
                            //println!("Abs coeff: {}", env.abs_coeff());
                        //};
>>>>>>> Stashed changes

                        *cr.rec_mut().abs_mut() +=
                            phot.weight() * phot.power() * env.abs_coeff() * dist;

                        *cr.rec_mut().scats_mut() += phot.weight();
                        phot.ray_mut().rotate(
                            distribution::henyey_greenstein(&mut rng, env.asym()),
                            rng.gen_range(0.0, 2.0 * PI),
                        );

                        *cr.rec_mut().abs_mut() +=
                            phot.weight() * phot.power() * env.abs_coeff() * dist;
                        *phot.weight_mut() *= env.albedo();
                        let enhanced_prob = 1000.0 * env.shift_prob();

                        if !_shifted && rng.gen_range(0.0, 1.0) <= enhanced_prob {
                            let mut reweight = phot.clone();
                            *phot.weight_mut() *= env.shift_prob() / enhanced_prob;
                            *reweight.weight_mut() *= 1.0 - env.shift_prob();
                            extra_phot = Some(reweight);
                            *cr.rec_mut().shifts_mut() += phot.weight();
                            *cr.rec_mut().ram_laser_mut() += 1.0;
                            *phot.wavelength_mut() = 884.0e-9;
                            _shifted = true;
                            env = mats.get(&MatKey::new("ptfe")).optics().env(phot.wavelength());
                            //println!("Ramanised!: {}", phot.ray().pos());
                        }
                        if _shifted {
                            *cr.rec_mut().det_raman_mut() += peel_off(
                                phot.clone(),
                                env.clone(),
                                grid,
                                mats,
                                &Point3::new(0.0129, 0.0, 0.0),
                                bump_dist,
                            )
                            .unwrap_or(0.0);
                        }
                    }
                    Hit::Cell(dist) => {
                        let dist = dist + bump_dist;
                        *cr.rec_mut().dist_trav_mut() += dist;
                        phot.ray_mut().travel(dist);
                        *cr.rec_mut().abs_mut() +=
                            phot.weight() * phot.power() * env.abs_coeff() * dist;

                        if !grid.bound().contains(phot.ray().pos()) {
                            break;
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
                                break;
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
                            break;
                        }
                        cr = CellRec::new(phot.ray().pos(), grid, &mut lm);
                    }
                }
            }
        }
    }

    lm
}

<<<<<<< Updated upstream
/// Create a periodic-xy boundary condition for the photons.
fn _periodic_xy(phot: &mut Photon, mins: &Point3<f64>, maxs: &Point3<f64>) -> bool {
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
=======
>>>>>>> Stashed changes

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

/// Perform a peel off event.
#[inline]
#[must_use]
pub fn peel_off(
    mut phot: Photon,
    mut env: Environment,
    grid: &Grid,
    mats: &MatSet,
    pos: &Point3<f64>,
    bump_dist: f64,
) -> Option<f64> {
    let g = env.asym();
    let g2 = g.powi(2);

    let dir = Unit::new_normalize(pos - phot.ray().pos());

    let cos_ang = phot.ray().dir().dot(&dir);
    let mut prob = phot.weight() * 0.5 * ((1.0 - g2) / (1.0 + g2 - (2.0 * g * cos_ang)).powf(1.5));
    if prob < 0.00001 {
        return None;
    }

    *phot.ray_mut().dir_mut() = dir;
    let mut cell = get_cell(phot.ray().pos(), &grid);

    loop {
        //if prob < 0.00001 {
        //    return None;
        //}

        let cell_dist = cell
            .bound()
            .dist(phot.ray())
            .expect("Unable to determine cell distance.");
        let inter_dist = cell.inter_dist_inside_norm_inter(phot.ray());

        if let Some((dist, inside, _norm, inter)) = inter_dist {
            if dist < cell_dist {
                prob *= (-(dist + bump_dist) * env.inter_coeff()).exp();
                phot.ray_mut().travel(dist + bump_dist);

                if inside {
                    env = mats
                        .map()
                        .get(inter.in_mat())
                        .unwrap()
                        .optics()
                        .env(phot.wavelength());
                } else {
                    env = mats
                        .map()
                        .get(inter.out_mat())
                        .unwrap()
                        .optics()
                        .env(phot.wavelength());
                }
            } else {
                prob *= (-(cell_dist + bump_dist) * env.inter_coeff()).exp();
                phot.ray_mut().travel(cell_dist + bump_dist);
            }
        } else {
            prob *= (-(cell_dist + bump_dist) * env.inter_coeff()).exp();
            phot.ray_mut().travel(cell_dist + bump_dist);
        }

        if !grid.bound().contains(phot.ray().pos()) {
            break;
        }

        cell = get_cell(phot.ray().pos(), &grid);
    }
    //report!(prob);
    Some(prob)
}

///Get a cell reference
#[inline]
#[must_use]
pub fn get_cell<'a>(pos: &Point3<f64>, grid: &'a &Grid) -> &'a Cell<'a> {
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

    let cell = grid.cells().get(index).expect("Invalid grid index.");

    debug_assert!(cell.bound().contains(pos));

    cell
}
