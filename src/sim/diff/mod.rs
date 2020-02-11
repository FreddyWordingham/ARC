//! Diffusion simulation sub-module.

use crate::{
    clone, report,
    util::ProgressBar,
    world::{Grid, Verse},
};
use nalgebra::Vector3;
use ndarray::Array3;
use ndarray_stats::QuantileExt;
use physical_constants::BOLTZMANN_CONSTANT;
use rayon::prelude::*;
use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

/// Diffusion temperature.
const TEMPERATURE: f64 = 310.15;

/// Run a diffusion transfer simulation.
#[inline]
// #[must_use]
pub fn run(num_threads: usize, total_time: f64, verse: &Verse, grid: &mut Grid) {
    debug_assert!(num_threads > 0);
    debug_assert!(total_time > 0.0);

    let cell_size = Vector3::from_vec(
        grid.bound()
            .widths()
            .iter()
            .zip(&grid.res())
            .map(|(w, r)| w / *r as f64)
            .collect::<Vec<_>>(),
    );
    let dx = cell_size.min();

    for (key, spec) in verse.specs().map() {
        if let Some(rad) = spec.rad() {
            let coeffs = grid.cells().map(|c| {
                if let Some(visc) = verse.mats().get(c.mat()).visc() {
                    Some(BOLTZMANN_CONSTANT * TEMPERATURE / (6.0 * PI * visc * rad))
                } else {
                    None
                }
            });
            let mut concs = grid.spec_refs_mut(key, verse.specs());

            if concs.map(|x| **x).sum() <= 0.0 {
                // TODO: Can probably find a neater way of doing this.
                continue;
            }

            if let Some(max_coeff) = coeffs.max().unwrap_or_else(|_| {
                panic!(
                    "Unable to determine the maximum diffusion coefficient for the {} species.",
                    key
                )
            }) {
                let max_dt = (dx.powi(2) / (4.0 * max_coeff)) * 0.9;

                let steps = (total_time / max_dt).ceil() as u64;
                let dt = total_time / steps as f64;

                let mut pb = ProgressBar::new(&format!("Diffusing species {}", key), steps);
                for _ in 0..steps {
                    pb.tick();
                    let delta = rate(&concs, &coeffs, &cell_size) * dt;

                    for (c, r) in concs.iter_mut().zip(delta.iter()) {
                        **c += r;
                    }
                }
            }
        }
    }
}

/// Step forward the diffusion a given amount.
#[inline]
#[must_use]
fn rate(
    concs: &Array3<&mut f64>,
    coeffs: &Array3<Option<f64>>,
    cell_size: &Vector3<f64>,
) -> Array3<f64> {
    let shape = concs.shape();

    let num_cells = concs.len();

    let rate = Arc::new(Mutex::new(Array3::zeros([shape[0], shape[1], shape[2]])));

    (0..num_cells).into_par_iter().for_each(|n| {
        let xi = n % shape[0];
        let yi = (n / shape[0]) % shape[1];
        let zi = n / (shape[0] * shape[1]);

        let index = [xi, yi, zi];

        if let Some(coeff) = coeffs[index] {
            let cv = ConcView::new(index, concs);
            *rate.lock().unwrap().get_mut(index).unwrap() = coeff
                * (((cv.px() - cv.c2() + cv.nx()) / cell_size.x.powi(2))
                    + ((cv.py() - cv.c2() + cv.ny()) / cell_size.y.powi(2))
                    + ((cv.pz() - cv.c2() + cv.nz()) / cell_size.z.powi(2)));
        }
    });

    Arc::try_unwrap(rate).unwrap().into_inner().unwrap()
}

struct ConcView {
    c2: f64,
    px: f64,
    nx: f64,
    py: f64,
    ny: f64,
    pz: f64,
    nz: f64,
}

impl ConcView {
    clone!(c2, f64);
    clone!(px, f64);
    clone!(nx, f64);
    clone!(py, f64);
    clone!(ny, f64);
    clone!(pz, f64);
    clone!(nz, f64);

    /// Construct a new instance.
    pub fn new(index: [usize; 3], concs: &Array3<&mut f64>) -> Self {
        debug_assert!(index.iter().all(|x| *x > 1));

        let shape = concs.shape();

        let [xi, yi, zi] = index;

        let c2 = *concs[[xi, yi, zi]] * 2.0;

        let px = if xi > 0 {
            *concs[[xi - 1, yi, zi]]
        } else {
            c2 - *concs[[xi + 1, yi, zi]]
        };
        let nx = if xi < (shape[0] - 1) {
            *concs[[xi + 1, yi, zi]]
        } else {
            c2 - *concs[[xi - 1, yi, zi]]
        };

        let py = if yi > 0 {
            *concs[[xi, yi - 1, zi]]
        } else {
            c2 - *concs[[xi, yi + 1, zi]]
        };
        let ny = if yi < (shape[1] - 1) {
            *concs[[xi, yi + 1, zi]]
        } else {
            c2 - *concs[[xi, yi - 1, zi]]
        };

        let pz = if zi > 0 {
            *concs[[xi, yi, zi - 1]]
        } else {
            c2 - *concs[[xi, yi, zi + 1]]
        };
        let nz = if zi < (shape[2] - 1) {
            *concs[[xi, yi, zi + 1]]
        } else {
            c2 - *concs[[xi, yi, zi - 1]]
        };

        Self {
            c2,
            px,
            nx,
            py,
            ny,
            pz,
            nz,
        }
    }
}
