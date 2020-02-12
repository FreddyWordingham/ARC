//! Periodic in x and y concentration view.

use crate::clone;
use ndarray::Array3;

/// Construct a periodic view of the concentrations in the x and y dimensions.
/// The z dimension is handled by a gradient boundary condition.
pub struct PeriodicXY {
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

impl PeriodicXY {
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
            shape.get(0).expect("Missing index.") - 1,
            shape.get(1).expect("Missing index.") - 1,
            shape.get(2).expect("Missing index.") - 1,
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

        let prev_x = **concs
            .get([wrap_prev(xi, *max.get(0).expect("Missing index.")), yi, zi])
            .expect("Invalid index.");
        let next_x = **concs
            .get([wrap_next(xi, *max.get(0).expect("Missing index.")), yi, zi])
            .expect("Invalid index.");

        let prev_y = **concs
            .get([xi, wrap_prev(yi, *max.get(1).expect("Missing index.")), zi])
            .expect("Invalid index.");
        let next_y = **concs
            .get([xi, wrap_next(yi, *max.get(1).expect("Missing index.")), zi])
            .expect("Invalid index.");

        let prev_z =
        // c2-
            **concs
                .get([xi, yi, wrap_prev(zi, *max.get(2).expect("Missing index."))])
                .expect("Invalid index.");
        let next_z =
        // c2-
            **concs
                .get([xi, yi, wrap_next(zi, *max.get(2).expect("Missing index."))])
                .expect("Invalid index.");

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
