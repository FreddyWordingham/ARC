//! Diffusion simulation sub-module.

use crate::{
    math::list,
    world::{Grid, Verse},
};
use ndarray_stats::QuantileExt;
use physical_constants::BOLTZMANN_CONSTANT;
use std::f64::consts::PI;

/// Diffusion temperature.
const TEMPERATURE: f64 = 310.15;

/// Run a diffusion transfer simulation.
#[inline]
// #[must_use]
pub fn run(num_threads: usize, total_time: f64, verse: &Verse, grid: &mut Grid) {
    debug_assert!(num_threads > 0);
    debug_assert!(total_time > 0.0);

    let dx = list::min(
        &grid
            .bound()
            .widths()
            .iter()
            .zip(&grid.res())
            .map(|(w, r)| w / *r as f64)
            .collect::<Vec<_>>(),
    );

    for (key, spec) in verse.specs().map() {
        if let Some(rad) = spec.rad() {
            let _concs = grid.spec_refs_mut(key, verse.specs());
            let coeffs = grid.cells().map(|c| {
                if let Some(visc) = verse.mats().get(c.mat()).visc() {
                    Some(BOLTZMANN_CONSTANT * TEMPERATURE / (6.0 * PI * visc * rad))
                } else {
                    None
                }
            });
            if let Some(max_coeff) = coeffs.max().unwrap() {
                let max_dt = (dx.powi(2) / (4.0 * max_coeff.powi(2))) * 0.1;
                println!("Max dt for {} is {}", key, max_dt);
            }
        }
    }
}
