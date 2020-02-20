//! Diffusion simulation sub-module.

pub mod grid;
pub mod settings;
pub mod stencil;

pub use self::{grid::*, settings::*, stencil::*};

use crate::{
    list::Cartesian::{X, Y},
    ord::SpecSet,
};
use nalgebra::Vector3;
use ndarray::{Array1, Array3};
use ndarray_stats::QuantileExt;
use physical_constants::BOLTZMANN_CONSTANT;
use rayon::prelude::*;
use std::{f64::consts::PI, sync::Mutex};

/// Diffusion temperature [K].
const TEMP: f64 = 310.0;

/// Minimum timestep.
const MIN_TIMESTEP: f64 = 1.0e-3;

/// Diffuse the concentrations forward the given amount of time.
#[inline]
pub fn run(
    time: f64,
    grid: &Grid,
    specs: &SpecSet,
    concs: &mut Array3<Array1<f64>>,
    viscs: &Array3<Option<f64>>,
) {
    let mut cell_size = grid.bound().widths();
    for (w, n) in cell_size.iter_mut().zip(&grid.res()) {
        *w /= *n as f64;
    }
    let dx = cell_size.min();

    for (i, spec) in specs.map().values().enumerate() {
        if let Some(rad) = spec.rad() {
            let coeffs = viscs.map(|visc| {
                if let Some(visc) = visc {
                    Some((TEMP * BOLTZMANN_CONSTANT) / (6.0 * PI * rad * visc))
                } else {
                    None
                }
            });
            if let Some(max_coeff) = coeffs
                .max()
                .expect("Could not determine maximum diffusion coefficient.")
            {
                let mut cs = concs.map_mut(|c| c.get_mut(i).expect("Invalid species index."));
                let max_dt = dx.powi(2) / (4.0 * max_coeff);
                let dt = max_dt * 0.1;
                let dt = dt.max(MIN_TIMESTEP);

                let steps = (time / dt).ceil() as u64;
                let dt = time / steps as f64;

                for _ in 0..steps {
                    let rates = diff_rate(&cs, &coeffs, &cell_size);
                    for (c, r) in cs.iter_mut().zip(rates.iter()) {
                        **c += r;
                    }
                }
            }
        }
    }
}

/// Calculate the diffusion rates for each cell.
#[inline]
#[must_use]
pub fn diff_rate(
    concs: &Array3<&mut f64>,
    coeffs: &Array3<Option<f64>>,
    cell_size: &Vector3<f64>,
) -> Array3<f64> {
    debug_assert!(concs.shape() == coeffs.shape());

    let num_cells = concs.len();

    let rate = Array3::zeros(concs.raw_dim());
    let res = concs.shape();
    let rate = Mutex::new(rate);

    (0..num_cells).into_par_iter().for_each(|n| {
        let xi = n % res.get(X as usize).expect("Missing index.");
        let yi = (n / res.get(X as usize).expect("Missing index."))
            % res.get(Y as usize).expect("Missing index.");
        let zi = n
            / (res.get(X as usize).expect("Missing index.")
                * res.get(Y as usize).expect("Missing index."));

        let index = [xi, yi, zi];

        if let Some(coeff) = coeffs.get(index).expect("Invalid index.") {
            let stencil = Stencil::new_gradient(index, concs);
            let r = stencil.rate(*coeff, cell_size);
            *rate
                .lock()
                .expect("Unable to lock rate array.")
                .get_mut(index)
                .expect("Invalid index.") = r;
        }
    });

    rate.into_inner()
        .expect("Unable to retrieve rates from within mutex.")
}
