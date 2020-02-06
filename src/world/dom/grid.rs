//! Grid implementation.

use crate::{
    access,
    geom::Aabb,
    math::indexer,
    ord::sort,
    util::ParProgressBar,
    world::{Cell, Verse},
};
use nalgebra::{Point3, Vector3};
use ndarray::Array3;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

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
    pub fn new(bound: Aabb, res: [usize; 3], _verse: &Verse) -> Self {
        let total_cells = res.get(0).expect("Missing resolution index.")
            * res.get(1).expect("Missing resolution index.")
            * res.get(2).expect("Missing resolution index.");

        assert!(total_cells > 0);

        let mut cell_size = bound.widths();
        for (w, n) in cell_size.iter_mut().zip(&res) {
            *w /= *n as f64;
        }

        let num_threads = 4;

        let pb = ParProgressBar::new("Building", total_cells as u64);
        let pb = Arc::new(Mutex::new(pb));
        let thread_ids: Vec<usize> = (0..num_threads).collect();

        let cell_blocks: Vec<_> = thread_ids
            .par_iter()
            .map(|id| {
                Self::init_cell_blocks(
                    *id,
                    res,
                    &cell_size,
                    Arc::clone(&pb),
                    ((total_cells / num_threads) / 100).min(10) as u64,
                )
            })
            .collect();
        pb.lock()
            .expect("Could not lock progress bar.")
            .finish_with_message("Complete.");

        let cells = sort::stitch(cell_blocks);

        Self {
            bound,
            cells: Array3::from_shape_vec(res, cells)
                .expect("Failed to convert cell vector to an array3."),
        }
    }

    /// Initialise the cells populating the grid.
    #[inline]
    #[must_use]
    fn init_cell_blocks(
        _id: usize,
        res: [usize; 3],
        cell_size: &Vector3<f64>,
        pb: Arc<Mutex<ParProgressBar>>,
        block_size: u64,
    ) -> Vec<(usize, Vec<Cell>)> {
        let mut cell_blocks = Vec::new();

        while let Some((start, end)) = {
            let mut pb = pb.lock().expect("Could not lock progress bar.");
            let b = pb.block(block_size);
            std::mem::drop(pb);
            b
        } {
            let mut cells: Vec<_> = Vec::with_capacity((end - start) as usize);
            for n in start..end {
                let index = indexer::three_dim(n as usize, res);

                let x = cell_size.x * index[0] as f64;
                let y = cell_size.y * index[1] as f64;
                let z = cell_size.z * index[2] as f64;

                let mins = Point3::new(x, y, z);
                let maxs = mins + cell_size;

                cells.push(Cell::new(
                    Aabb::new(mins, maxs),
                    crate::ord::MatKey::new("mat_key"),
                    crate::ord::StateKey::new("state_key"),
                ));
            }
            cell_blocks.push((start as usize, cells));
        }

        cell_blocks
    }
}
