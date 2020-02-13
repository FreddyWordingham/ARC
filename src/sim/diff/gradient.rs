//! Gradient concentration view.

use crate::{
    clone,
    list::Cartesian::{X, Y, Z},
};
use ndarray::Array3;

/// Construct a viewing gradient of the concentrations.
pub struct Gradient {
    /// Twice the central concentration.
    c2: f64,
    /// Previous-x concentration.
    prev_x: f64,
    /// Next-x concentration.
    next_x: f64,
    /// Previous-y concentration.
    prev_y: f64,
    /// Next-y concentration.
    next_y: f64,
    /// Previous-z concentration.
    prev_z: f64,
    /// Next-z concentration.
    next_z: f64,
}

impl Gradient {
    clone!(c2, f64);
    clone!(prev_x, f64);
    clone!(next_x, f64);
    clone!(prev_y, f64);
    clone!(next_y, f64);
    clone!(prev_z, f64);
    clone!(next_z, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(index: [usize; 3], concs: &Array3<&mut f64>) -> Self {
        debug_assert!(index.iter().all(|x| *x > 1));

        let shape = concs.shape();
        let max = [
            shape.get(X as usize).expect("Missing index.") - 1,
            shape.get(Y as usize).expect("Missing index.") - 1,
            shape.get(Z as usize).expect("Missing index.") - 1,
        ];

        let [xi, yi, zi] = index;

        let wrap_prev = |x, max| {
            if x == 0 {
                max
            } else {
                x - 1
            }
        };
        let wrap_next = |x, max| {
            if x == max {
                0
            } else {
                x + 1
            }
        };

        let c2 = **concs.get([xi, yi, zi]).expect("Invalid index.") * 2.0;

        let prev_x = if xi == 0 {
            (c2 - **concs
                .get([
                    wrap_next(xi, *max.get(X as usize).expect("Missing index.")),
                    yi,
                    zi,
                ])
                .expect("Invalid index."))
            .max(0.0)
        } else {
            **concs
                .get([
                    wrap_prev(xi, *max.get(X as usize).expect("Missing index.")),
                    yi,
                    zi,
                ])
                .expect("Invalid index.")
        };
        let next_x = if xi == *max.get(X as usize).expect("Missing index.") {
            (c2 - **concs
                .get([
                    wrap_prev(xi, *max.get(X as usize).expect("Missing index.")),
                    yi,
                    zi,
                ])
                .expect("Invalid index."))
            .max(0.0)
        } else {
            **concs
                .get([
                    wrap_next(xi, *max.get(X as usize).expect("Missing index.")),
                    yi,
                    zi,
                ])
                .expect("Invalid index.")
        };

        let prev_y = if yi == 0 {
            (c2 - **concs
                .get([
                    xi,
                    wrap_next(yi, *max.get(Y as usize).expect("Missing index.")),
                    zi,
                ])
                .expect("Invalid index."))
            .max(0.0)
        } else {
            **concs
                .get([
                    xi,
                    wrap_prev(yi, *max.get(Y as usize).expect("Missing index.")),
                    zi,
                ])
                .expect("Invalid index.")
        };
        let next_y = if yi == *max.get(Y as usize).expect("Missing index.") {
            (c2 - **concs
                .get([
                    xi,
                    wrap_prev(yi, *max.get(Y as usize).expect("Missing index.")),
                    zi,
                ])
                .expect("Invalid index."))
            .max(0.0)
        } else {
            **concs
                .get([
                    xi,
                    wrap_next(yi, *max.get(Y as usize).expect("Missing index.")),
                    zi,
                ])
                .expect("Invalid index.")
        };

        let prev_z = if zi == 0 {
            (c2 - **concs
                .get([
                    xi,
                    yi,
                    wrap_next(zi, *max.get(Z as usize).expect("Missing index.")),
                ])
                .expect("Invalid index."))
            .max(0.0)
        } else {
            **concs
                .get([
                    xi,
                    yi,
                    wrap_prev(zi, *max.get(Z as usize).expect("Missing index.")),
                ])
                .expect("Invalid index.")
        };
        let next_z = if zi == *max.get(Z as usize).expect("Missing index.") {
            (c2 - **concs
                .get([
                    xi,
                    yi,
                    wrap_prev(zi, *max.get(Z as usize).expect("Missing index.")),
                ])
                .expect("Invalid index."))
            .max(0.0)
        } else {
            **concs
                .get([
                    xi,
                    yi,
                    wrap_next(zi, *max.get(Z as usize).expect("Missing index.")),
                ])
                .expect("Invalid index.")
        };

        Self {
            c2,
            prev_x,
            next_x,
            prev_y,
            next_y,
            prev_z,
            next_z,
        }
    }
}
