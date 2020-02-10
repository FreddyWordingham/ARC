//! Grid implementation.

use crate::{
    access,
    geom::{Aabb, Ray},
    math::{indexer, list},
    ord::{sort, MatKey, MatSet, Set, SpecKey, SpecSet, StateKey, StateSet},
    util::ParProgressBar,
    world::{Cell, Verse},
};
use nalgebra::{Point3, Unit, Vector3};
use ndarray::Array3;
use rayon::prelude::*;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

/// Material detection rays must be aimed at a triangle with at least this deviation from the triangle's plane.
const HIT_ANGLE_THRESHOLD: f64 = 1.0e-3;

/// Grid partition scheme.
pub struct Grid<'a> {
    /// Boundary.
    bound: Aabb,
    /// Cells.
    cells: Array3<Cell<'a>>,
}

impl<'a> Grid<'a> {
    access!(bound, Aabb);
    access!(cells, Array3<Cell<'a>>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(num_threads: usize, bound: Aabb, res: [usize; 3], verse: &'a Verse) -> Self {
        debug_assert!(num_threads > 0);

        let total_cells = res.get(0).expect("Missing resolution index.")
            * res.get(1).expect("Missing resolution index.")
            * res.get(2).expect("Missing resolution index.");

        debug_assert!(total_cells > 0);

        let mut cell_size = bound.widths();
        for (w, n) in cell_size.iter_mut().zip(&res) {
            *w /= *n as f64;
        }

        let pb = ParProgressBar::new("Building", total_cells as u64);
        let pb = Arc::new(Mutex::new(pb));
        let thread_ids: Vec<usize> = (0..num_threads).collect();

        let cell_blocks: Vec<_> = thread_ids
            .par_iter()
            .map(|id| {
                Self::init_cell_blocks(
                    *id,
                    res,
                    verse,
                    &bound,
                    &cell_size,
                    &Arc::clone(&pb),
                    ((total_cells / num_threads) / 100).max(10) as u64,
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
        verse: &'a Verse,
        bound: &Aabb,
        cell_size: &Vector3<f64>,
        pb: &Arc<Mutex<ParProgressBar>>,
        block_size: u64,
    ) -> Vec<(usize, Vec<Cell<'a>>)> {
        let mut cell_blocks = Vec::new();

        let gen_mat_ray = |p: &Point3<f64>| -> Ray {
            for inter in verse.inters().map().values() {
                let surf = verse.surfs().get(inter.surf());
                for tri in surf.tris() {
                    let tc = tri.tri().centre();

                    if bound.contains(&tc) {
                        let dir = Unit::new_normalize(tc - p);
                        if dir.dot(tri.tri().plane_norm()).abs() >= HIT_ANGLE_THRESHOLD {
                            return Ray::new(*p, dir);
                        }
                    }
                }
            }

            panic!("Unable to determine suitable material tracing ray.");
        };

        let gen_state_ray = |p: &Point3<f64>| -> Ray {
            for region in verse.regions().map().values() {
                let surf = verse.surfs().get(region.surf());
                for tri in surf.tris() {
                    let tc = tri.tri().centre();

                    if bound.contains(&tc) {
                        let dir = Unit::new_normalize(tc - p);
                        if dir.dot(tri.tri().plane_norm()).abs() >= HIT_ANGLE_THRESHOLD {
                            return Ray::new(*p, dir);
                        }
                    }
                }
            }

            panic!("Unable to determine suitable state tracing ray.");
        };

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

                let mins = bound.mins() + Vector3::new(x, y, z);
                let maxs = mins + cell_size;
                let cell_bound = Aabb::new(mins, maxs);

                let p = cell_bound.centre();

                let mat = verse
                    .inters()
                    .observe_mat(verse.surfs(), bound, &gen_mat_ray(&p))
                    .expect("Unable to observe material.");
                let state = verse
                    .regions()
                    .observe_state(verse.surfs(), bound, &gen_state_ray(&p))
                    .expect("Unable to observe state.");

                cells.push(Cell::new(Aabb::new(mins, maxs), mat, state, verse));
            }
            cell_blocks.push((start as usize, cells));
        }

        cell_blocks
    }

    /// Get the resolution of the grid.
    #[inline]
    #[must_use]
    pub fn res(&self) -> [usize; 3] {
        let res = self.cells.shape();

        [
            *res.get(0).expect("Missing resolution index."),
            *res.get(1).expect("Missing resolution index."),
            *res.get(2).expect("Missing resolution index."),
        ]
    }

    /// Determine a suitable bump distance for the grid.
    #[inline]
    #[must_use]
    pub fn bump_dist(&self) -> f64 {
        let mins: Vec<f64> = self
            .bound
            .widths()
            .iter()
            .zip(&self.res())
            .map(|(dx, r)| *dx / *r as f64 * 1.0e-3)
            .collect();

        list::min(&mins)
    }

    /// Create a map of the material keys.
    #[inline]
    #[must_use]
    pub fn mat_keys(&self) -> Array3<&MatKey> {
        self.cells.map(Cell::mat)
    }

    /// Create a set of material maps.
    #[inline]
    #[must_use]
    pub fn mat_maps(&self, mats: &MatSet) -> Set<MatKey, Array3<f64>> {
        let mut set = BTreeMap::new();

        let keys = self.mat_keys();
        for key in mats.map().keys() {
            set.insert(key.clone(), keys.map(|k| if k == &key { 1.0 } else { 0.0 }));
        }

        Set::new(set)
    }

    /// Create a map of the state keys.
    #[inline]
    #[must_use]
    pub fn state_keys(&self) -> Array3<&StateKey> {
        self.cells.map(Cell::state)
    }

    /// Create a set of state maps.
    #[inline]
    #[must_use]
    pub fn state_maps(&self, states: &StateSet) -> Set<StateKey, Array3<f64>> {
        let mut set = BTreeMap::new();

        let keys = self.state_keys();
        for key in states.map().keys() {
            set.insert(key.clone(), keys.map(|k| if k == &key { 1.0 } else { 0.0 }));
        }

        Set::new(set)
    }

    /// Create a map of the cells containing boundaries.
    #[inline]
    #[must_use]
    pub fn inter_boundaries(&self) -> Array3<f64> {
        self.cells
            .map(|c| if c.inter_tris().is_empty() { 0.0 } else { 1.0 })
    }

    /// Create a viewing map of a single species.
    #[inline]
    #[must_use]
    pub fn spec_refs(&self, spec: &SpecKey, specs: &SpecSet) -> Array3<&f64> {
        let index = specs.index_of_name(spec);
        self.cells.map(|c| c.concs().get(index).unwrap())
    }

    /// Create a mutable viewing map of a single species.
    #[inline]
    #[must_use]
    pub fn spec_refs_mut(&mut self, spec: &SpecKey, specs: &SpecSet) -> Array3<&mut f64> {
        let index = specs.index_of_name(spec);
        self.cells
            .map_mut(|c| c.concs_mut().get_mut(index).unwrap())
    }

    /// Create a set of species concentration reference maps.
    #[inline]
    #[must_use]
    pub fn spec_set_refs(&self, specs: &SpecSet) -> Set<SpecKey, Array3<&f64>> {
        let mut set = BTreeMap::new();

        for (index, key) in specs.map().keys().enumerate() {
            set.insert(
                key.clone(),
                self.cells.map(|c| c.concs().get(index).unwrap()),
            );
        }

        Set::new(set)
    }
}
