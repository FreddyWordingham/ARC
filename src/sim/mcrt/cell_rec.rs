//! Cell-Record referencing structure.

use crate::{
    access,
    list::Cartesian::{X, Y, Z},
    sim::{LightMap, Record},
    world::{Cell, Grid},
};
use nalgebra::Point3;

/// Store a reference to a cell and it's corresponding light-map record.
pub struct CellRec<'a> {
    /// Reference to the currently occupied cell.
    cell: &'a Cell<'a>,
    /// Reference to the respective light-map record.
    rec: &'a mut Record,
}

impl<'a> CellRec<'a> {
    access!(cell, cell_mut, &'a Cell<'a>);
    access!(rec, rec_mut, &'a mut Record);

    /// Construct a new instance.
    pub fn new(pos: &Point3<f64>, grid: &'a Grid, light_map: &'a mut LightMap) -> Self {
        let mins = grid.bound().mins();
        let maxs = grid.bound().maxs();
        let resolution = grid.res();

        let id: Vec<usize> = pos
            .iter()
            .zip(mins.iter().zip(maxs.iter()))
            .zip(&resolution)
            .map(|((p, (min, max)), n)| (((p - min) / (max - min)) * *n as f64) as usize)
            .collect();
        let index = (
            *id.get(X as usize).expect("Missing index."),
            *id.get(Y as usize).expect("Missing index."),
            *id.get(Z as usize).expect("Missing index."),
        );

        let cell = grid.cells().get(index).expect("Invalid grid index.");
        let rec = light_map
            .recs_mut()
            .get_mut(index)
            .expect("Invalid record index.");

        debug_assert!(cell.bound().contains(pos));

        Self { cell, rec }
    }
}
