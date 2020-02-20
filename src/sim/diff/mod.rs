//! Diffusion simulation sub-module.

pub mod grid;
pub mod settings;
pub mod stencil;

pub use self::{grid::*, settings::*, stencil::*};

use crate::list::Cartesian::{X, Y};
use nalgebra::Vector3;
use ndarray::Array3;
use rayon::prelude::*;
use std::sync::Mutex;

/// Calculate the diffusion rates for each cell.
#[inline]
#[must_use]
pub fn diff_rate(
    concs: &Array3<f64>,
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
        }
    });

    rate.into_inner()
        .expect("Unable to retrieve rates from within mutex.")
}
