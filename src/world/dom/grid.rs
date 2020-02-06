//! Grid implementation.

use crate::{access, geom::Aabb, world::Cell};
use ndarray::Array3;

// /// Material detection rays must be aimed at a triangle with at least this deviation from the triangle's plane.
// const HIT_ANGLE_THRESHOLD: f64 = 1.0e-3;

/// Grid partition scheme.
pub struct Grid {
    /// Boundary.
    bound: Aabb,
    /// Cells.
    cells: Array3<Cell>,
}

impl Grid {
    access!(bound, Aabb);
    access!(cells, Array3<Cell>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(bound: Aabb, res: [usize; 3]) -> Self {
        let total_cells = res.get(0).expect("Missing resolution index.")
            * res.get(1).expect("Missing resolution index.")
            * res.get(2).expect("Missing resolution index.");

        assert!(total_cells > 0);

        let cells = Vec::with_capacity(total_cells);

        for _n in 0..total_cells {}

        Self {
            bound,
            cells: Array3::from_shape_vec(res, cells)
                .expect("Failed to convert cell vector to an array3."),
        }
    }
}
