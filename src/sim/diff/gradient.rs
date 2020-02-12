//! Gradient concentration view.

use crate::clone;
use ndarray::Array3;

/// Construct a viewing gradient of the concentrations.
pub struct Gradient {
    /// Twice the central concentration.
    c2: f64,
    /// Previous-x concentration.
    px: f64,
    /// Next-x concentration.
    nx: f64,
    /// Previous-y concentration.
    py: f64,
    /// Next-y concentration.
    ny: f64,
    /// Previous-z concentration.
    pz: f64,
    /// Next-z concentration.
    nz: f64,
}

impl Gradient {
    clone!(c2, f64);
    clone!(px, f64);
    clone!(nx, f64);
    clone!(py, f64);
    clone!(ny, f64);
    clone!(pz, f64);
    clone!(nz, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(index: [usize; 3], concs: &Array3<&mut f64>) -> Self {
        debug_assert!(index.iter().all(|x| *x > 1));

        let shape = concs.shape();

        let [xi, yi, zi] = index;

        let c2 = **concs.get([xi, yi, zi]).expect("Missing index.") * 2.0;

        let px = if xi > 0 {
            **concs.get([xi - 1, yi, zi]).expect("Missing index.")
        } else {
            c2 - **concs.get([xi + 1, yi, zi]).expect("Missing index.")
        };
        let nx = if xi < (shape[0] - 1) {
            **concs.get([xi + 1, yi, zi]).expect("Missing index.")
        } else {
            c2 - **concs.get([xi - 1, yi, zi]).expect("Missing index.")
        };

        let py = if yi > 0 {
            **concs.get([xi, yi - 1, zi]).expect("Missing index.")
        } else {
            c2 - **concs.get([xi, yi + 1, zi]).expect("Missing index.")
        };
        let ny = if yi < (shape[1] - 1) {
            **concs.get([xi, yi + 1, zi]).expect("Missing index.")
        } else {
            c2 - **concs.get([xi, yi - 1, zi]).expect("Missing index.")
        };

        let pz = if zi > 0 {
            **concs.get([xi, yi, zi - 1]).expect("Missing index.")
        } else {
            c2 - **concs.get([xi, yi, zi + 1]).expect("Missing index.")
        };
        let nz = if zi < (shape[2] - 1) {
            **concs.get([xi, yi, zi + 1]).expect("Missing index.")
        } else {
            c2 - **concs.get([xi, yi, zi - 1]).expect("Missing index.")
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
