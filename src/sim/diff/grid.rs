//! Grid implementation.

use crate::{
    access, clone,
    geom::{Aabb, Ray},
    list::Cartesian::X,
    math::indexer,
    ord::{sort, InterSet, MatKey, RegionSet, StateKey, SurfSet},
    util::ParProgressBar,
};
use nalgebra::{Point3, Unit, Vector3};
use ndarray::Array3;
use num_cpus;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Material detection rays must be aimed at a triangle with at least this deviation from the triangle's plane.
const HIT_ANGLE_THRESHOLD: f64 = 1.0e-3;

/// Grid partition scheme.
pub struct Grid<'a> {
    /// Boundary.
    bound: Aabb,
    /// Minimum cell width.
    dx: f64,
    /// Array of local material keys.
    mats: Array3<&'a MatKey>,
    /// Array of local state keys.
    states: Array3<&'a StateKey>,
}

impl<'a> Grid<'a> {
    access!(bound, Aabb);
    clone!(dx, f64);
    access!(mats, Array3<&'a MatKey>);
    access!(states, Array3<&'a StateKey>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        res: [usize; 3],
        bound: Aabb,
        inters: &'a InterSet,
        regions: &'a RegionSet,
        surfs: &'a SurfSet,
    ) -> Self {
        let total_cells = res.get(X as usize).expect("Missing resolution index.")
            * res.get(1).expect("Missing resolution index.")
            * res.get(2).expect("Missing resolution index.");

        let mut cell_size = bound.widths();
        for (w, n) in cell_size.iter_mut().zip(&res) {
            *w /= *n as f64;
        }
        let dx = cell_size.min();

        let pb = ParProgressBar::new("Building material map", total_cells as u64);
        let pb = Arc::new(Mutex::new(pb));
        let thread_ids: Vec<usize> = (0..num_cpus::get()).collect();

        let mat_blocks: Vec<_> = thread_ids
            .par_iter()
            .map(|_| {
                Self::init_mat_blocks(
                    &Arc::clone(&pb),
                    ((total_cells / num_cpus::get()) / 100).max(1) as u64,
                    &res,
                    &bound,
                    inters,
                    surfs,
                    &cell_size,
                )
            })
            .collect();
        pb.lock()
            .expect("Could not lock progress bar.")
            .finish_with_message("Material map building complete.");

        let mats = sort::stitch(mat_blocks);

        let pb = ParProgressBar::new("Building state map", total_cells as u64);
        let pb = Arc::new(Mutex::new(pb));
        let thread_ids: Vec<usize> = (0..num_cpus::get()).collect();

        let state_blocks: Vec<_> = thread_ids
            .par_iter()
            .map(|_| {
                Self::init_state_blocks(
                    &Arc::clone(&pb),
                    ((total_cells / num_cpus::get()) / 100).max(1) as u64,
                    &res,
                    &bound,
                    regions,
                    surfs,
                    &cell_size,
                )
            })
            .collect();
        pb.lock()
            .expect("Could not lock progress bar.")
            .finish_with_message("State map building complete.");

        let states = sort::stitch(state_blocks);

        Self {
            bound,
            dx,
            mats: Array3::from_shape_vec(res, mats).expect("Unable to construct material array."),
            states: Array3::from_shape_vec(res, states).expect("Unable to construct state array."),
        }
    }

    /// Initialise the material keys populating the grid.
    #[inline]
    #[must_use]
    fn init_mat_blocks(
        pb: &Arc<Mutex<ParProgressBar>>,
        block_size: u64,
        res: &[usize; 3],
        bound: &Aabb,
        inters: &'a InterSet,
        surfs: &'a SurfSet,
        cell_size: &Vector3<f64>,
    ) -> Vec<(usize, Vec<&'a MatKey>)> {
        let mut mat_blocks = Vec::new();

        let gen_mat_ray = |p: &Point3<f64>| -> Ray {
            for inter in inters.map().values() {
                let surf = surfs.get(inter.surf());
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

        while let Some((start, end)) = {
            let mut pb = pb.lock().expect("Could not lock progress bar.");
            let b = pb.block(block_size);
            std::mem::drop(pb);
            b
        } {
            let mut mats: Vec<_> = Vec::with_capacity((end - start) as usize);
            for n in start..end {
                let index = indexer::three_dim(n as usize, *res);

                let x = cell_size.x * index[0] as f64;
                let y = cell_size.y * index[1] as f64;
                let z = cell_size.z * index[2] as f64;

                let mins = bound.mins() + Vector3::new(x, y, z);
                let maxs = mins + cell_size;
                let cell_bound = Aabb::new(mins, maxs);

                let p = cell_bound.centre();

                let mat = inters
                    .observe_mat(surfs, bound, &gen_mat_ray(&p))
                    .expect("Unable to observe material.");

                mats.push(mat);
            }
            mat_blocks.push((start as usize, mats));
        }

        mat_blocks
    }

    /// Initialise the state keys populating the grid.
    #[inline]
    #[must_use]
    fn init_state_blocks(
        pb: &Arc<Mutex<ParProgressBar>>,
        block_size: u64,
        res: &[usize; 3],
        bound: &Aabb,
        regions: &'a RegionSet,
        surfs: &'a SurfSet,
        cell_size: &Vector3<f64>,
    ) -> Vec<(usize, Vec<&'a StateKey>)> {
        let mut state_blocks = Vec::new();

        let gen_state_ray = |p: &Point3<f64>| -> Ray {
            for region in regions.map().values() {
                let surf = surfs.get(region.surf());
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

        while let Some((start, end)) = {
            let mut pb = pb.lock().expect("Could not lock progress bar.");
            let b = pb.block(block_size);
            std::mem::drop(pb);
            b
        } {
            let mut states: Vec<_> = Vec::with_capacity((end - start) as usize);
            for n in start..end {
                let index = indexer::three_dim(n as usize, *res);

                let x = cell_size.x * index[0] as f64;
                let y = cell_size.y * index[1] as f64;
                let z = cell_size.z * index[2] as f64;

                let mins = bound.mins() + Vector3::new(x, y, z);
                let maxs = mins + cell_size;
                let cell_bound = Aabb::new(mins, maxs);

                let p = cell_bound.centre();

                let state = regions
                    .observe_state(surfs, bound, &gen_state_ray(&p))
                    .expect("Unable to observe state.");

                states.push(state);
            }
            state_blocks.push((start as usize, states));
        }

        state_blocks
    }
}
