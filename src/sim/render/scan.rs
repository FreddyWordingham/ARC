//! Core scanning function.

use crate::{
    sim::render::{Camera, Cell},
    util::ParProgressBar,
};
use log::warn;
use nalgebra::{Point3, Unit};
use ndarray::Array2;
use std::sync::{Arc, Mutex};

/// Render using a single thread.
#[inline]
#[must_use]
pub fn run_thread(
    _id: usize,
    cam: &Camera,
    grid: &Cell,
    pb: &Arc<Mutex<ParProgressBar>>,
    block_size: usize,
) -> Vec<Array2<f64>> {
    let mut layer_0 = Array2::zeros(cam.res());
    let mut layer_1 = Array2::zeros(cam.res());
    let mut layer_2 = Array2::zeros(cam.res());
    let mut layer_3 = Array2::zeros(cam.res());

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size as u64);
        std::mem::drop(pb);
        b
    } {
        for n in start..end {
            let xi = n as usize % cam.res().0;
            let yi = n as usize / cam.res().0;

            let mut ray = cam.gen_ray(xi, yi);

            while let Some((new_ray, dist, norm, group)) = grid.observe(ray) {
                ray = new_ray;

                match group {
                    -1 => {
                        *layer_0.get_mut((xi, yi)).expect("Invalid pixel index.") += -1.0;
                        *layer_1.get_mut((xi, yi)).expect("Invalid pixel index.") += dist;
                        *layer_2.get_mut((xi, yi)).expect("Invalid pixel index.") +=
                            ray.dir().dot(&norm).acos();

                        // {
                        //     let mut trace = ray.clone();
                        //     let light_dir =
                        //         Unit::new_normalize(Point3::new(-10.2, 0.0, 1.2) - trace.pos());
                        //     *trace.dir_mut() = light_dir;
                        //     trace.travel(1.0e-6);
                        //     if let Some((_r, d, n, -1)) = grid.observe(trace) {
                        //         *layer_3.get_mut((xi, yi)).expect("Invalid pixel index.") +=
                        //             n.dot(&light_dir).acos() / (d * d);
                        //     }
                        // }
                        break;
                    }
                    0 => {
                        *layer_0.get_mut((xi, yi)).expect("Invalid pixel index.") += 1.0;
                        *layer_1.get_mut((xi, yi)).expect("Invalid pixel index.") += dist;
                        *layer_2.get_mut((xi, yi)).expect("Invalid pixel index.") +=
                            ray.dir().dot(&norm).acos();

                        {
                            let mut trace = ray.clone();
                            let light_dir =
                                Unit::new_normalize(Point3::new(10.0, 3.0, 15.0) - trace.pos());
                            *trace.dir_mut() = light_dir;
                            trace.travel(1.0e-6);
                            match grid.observe(trace) {
                                None => {
                                    *layer_3.get_mut((xi, yi)).expect("Invalid pixel index.") +=
                                        norm.dot(&light_dir).max(0.0)
                                }
                                Some((_r, _d, _n, 0)) => {
                                    *layer_3.get_mut((xi, yi)).expect("Invalid pixel index.") +=
                                        0.1 * norm.dot(&light_dir).max(0.0)
                                }
                                _ => {}
                            }
                        }
                        break;
                    }
                    1 => {
                        ray.reflect(&norm);
                        ray.travel(1.0e-6);
                    }
                    _ => {
                        warn!("Do not know how to handle group {}.", group);
                        break;
                    }
                }
            }
        }
    }

    vec![layer_0, layer_1, layer_2, layer_3]
}
