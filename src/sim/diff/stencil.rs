//! Stencil-type enumeration.

use nalgebra::Vector3;
use ndarray::Array3;

/// Stencil enumeration implementation.
#[derive(Debug)]
pub enum Stencil {
    /// Gradient.
    Gradient {
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
    },
}

impl Stencil {
    /// Construct a new gradient instance.
    #[inline]
    #[must_use]
    pub fn new_gradient(index: [usize; 3], concs: &Array3<f64>) -> Self {
        let maxs: Vec<_> = concs.shape().iter().map(|r| r - 1).collect();
        let max_x = *maxs.get(0).expect("Missing index.");
        let max_y = *maxs.get(1).expect("Missing index.");
        let max_z = *maxs.get(2).expect("Missing index.");

        let [xi, yi, zi] = index;

        let c2 = *concs.get([xi, yi, zi]).expect("Invalid index.") * 2.0;

        let prev_x = if xi == 0 {
            c2 - *concs.get([xi + 1, yi, zi]).expect("Invalid index.")
        } else {
            *concs.get([xi - 1, yi, zi]).expect("Invalid index.")
        };
        let next_x = if xi == max_x {
            c2 - *concs.get([xi - 1, yi, zi]).expect("Invalid index.")
        } else {
            *concs.get([xi + 1, yi, zi]).expect("Invalid index.")
        };

        let prev_y = if yi == 0 {
            c2 - *concs.get([xi, yi + 1, zi]).expect("Invalid index.")
        } else {
            *concs.get([xi, yi - 1, zi]).expect("Invalid index.")
        };
        let next_y = if yi == max_y {
            c2 - *concs.get([xi, yi - 1, zi]).expect("Invalid index.")
        } else {
            *concs.get([xi, yi + 1, zi]).expect("Invalid index.")
        };

        let prev_z = if zi == 0 {
            c2 - *concs.get([xi, yi, zi + 1]).expect("Invalid index.")
        } else {
            *concs.get([xi, yi, zi - 1]).expect("Invalid index.")
        };
        let next_z = if zi == max_z {
            c2 - *concs.get([xi, yi, zi - 1]).expect("Invalid index.")
        } else {
            *concs.get([xi, yi, zi + 1]).expect("Invalid index.")
        };

        Self::Gradient {
            c2,
            prev_x,
            next_x,
            prev_y,
            next_y,
            prev_z,
            next_z,
        }
    }

    /// Calculate the rate of diffusion.
    #[inline]
    #[must_use]
    pub fn rate(&self, coeff: f64, cell_size: &Vector3<f64>) -> f64 {
        match self {
            Self::Gradient {
                c2,
                prev_x,
                next_x,
                prev_y,
                next_y,
                prev_z,
                next_z,
            } => {
                coeff
                    * (((next_x - c2 + prev_x) / cell_size.x.powi(2))
                        + ((next_y - c2 + prev_y) / cell_size.y.powi(2))
                        + ((next_z - c2 + prev_z) / cell_size.z.powi(2)))
            }
        }
    }
}
