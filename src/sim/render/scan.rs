//! Core scanning function.

use crate::{
    sim::render::{Camera, Cell},
    util::ParProgressBar,
};
use log::warn;
use ndarray::Array2;
use std::sync::{Arc, Mutex};

// /// Render using a single thread.
// #[inline]
// #[must_use]
// pub fn run_thread(
//     _id: usize,
//     cam: &Camera,
//     grid: &Cell,
//     pb: &Arc<Mutex<ParProgressBar>>,
//     block_size: usize,
// ) -> Vec<Array2<f64>> {
//     let mut layer_0 = Array2::zeros(cam.res());
//     let mut layer_1 = Array2::zeros(cam.res());
//     let mut layer_2 = Array2::zeros(cam.res());
//     let mut layer_3 = Array2::zeros(cam.res());
//     let mut layer_4 = Array2::zeros(cam.res());

//     let super_samples = cam.ss_power().pow(2);

//     while let Some((start, end)) = {
//         let mut pb = pb.lock().expect("Could not lock progress bar.");
//         let b = pb.block(block_size as u64);
//         std::mem::drop(pb);
//         b
//     } {
//         for n in start..end {
//             let xi = n as usize % cam.res().0;
//             let yi = n as usize / cam.res().0;

//             for n in 0..super_samples {
//                 let mut ray = cam.gen_ss_ray(xi, yi, n);

//                 while let Some((new_ray, dist, norm, group)) = grid.observe(ray) {
//                     ray = new_ray;

//                     match group {
//                         -1 => {
//                             *layer_0.get_mut((xi, yi)).expect("Invalid pixel index.") += -1.0;
//                             *layer_1.get_mut((xi, yi)).expect("Invalid pixel index.") += dist;
//                             *layer_2.get_mut((xi, yi)).expect("Invalid pixel index.") +=
//                                 ray.dir().dot(&norm).acos();
//                             break;
//                         }
//                         0 => {
//                             *layer_0.get_mut((xi, yi)).expect("Invalid pixel index.") += 1.0;
//                             *layer_1.get_mut((xi, yi)).expect("Invalid pixel index.") += dist;
//                             *layer_2.get_mut((xi, yi)).expect("Invalid pixel index.") +=
//                                 ray.dir().dot(&norm).acos();

//                             {
//                                 let mut trace = ray.clone();
//                                 let light_dir =
//                                     Unit::new_normalize(Point3::new(9.0, 3.0, 7.0) - trace.pos());
//                                 *trace.dir_mut() = light_dir;
//                                 trace.travel(1.0e-6);
//                                 match grid.observe(trace) {
//                                     None => {
//                                         *layer_3
//                                             .get_mut((xi, yi))
//                                             .expect("Invalid pixel index.") +=
//                                             norm.dot(&light_dir).abs();
//                                     }
//                                     Some((_r, _d, _n, 0)) => {
//                                         *layer_3
//                                             .get_mut((xi, yi))
//                                             .expect("Invalid pixel index.") +=
//                                             0.25 * norm.dot(&light_dir).abs();
//                                     }
//                                     _ => {}
//                                 }
//                             }
//                             {
//                                 let mut trace = ray.clone();
//                                 let light_dir =
//                                     Unit::new_normalize(Point3::new(9.0, 3.0, 7.0) - trace.pos());
//                                 *trace.dir_mut() = light_dir;
//                                 trace.travel(1.0e-6);
//                                 match grid.observe(trace) {
//                                     None => {
//                                         *layer_4
//                                             .get_mut((xi, yi))
//                                             .expect("Invalid pixel index.") +=
//                                             (norm.dot(&light_dir).abs())
//                                                 * (100.0
//                                                     - nalgebra::distance(
//                                                         cam.forward().pos(),
//                                                         ray.pos(),
//                                                     ))
//                                                 .max(0.0)
//                                                 .pow(3);
//                                     }
//                                     Some((_r, _d, _n, 0)) => {
//                                         *layer_4
//                                             .get_mut((xi, yi))
//                                             .expect("Invalid pixel index.") += (0.25
//                                             * norm.dot(&light_dir).abs())
//                                             * (100.0
//                                                 - nalgebra::distance(
//                                                     cam.forward().pos(),
//                                                     ray.pos(),
//                                                 ))
//                                             .max(0.0)
//                                             .pow(3);
//                                     }
//                                     _ => {}
//                                 }
//                             }
//                             break;
//                         }
//                         1 => {
//                             ray.reflect(&norm);
//                             ray.travel(1.0e-6);
//                         }
//                         2 => {
//                             ray.reflect(&norm);

//                             let theta =
//                                 (ray.pos().x.sin().powi(2) * ray.pos().y.sin().powi(2)) / 6.2;
//                             let rot = nalgebra::Rotation3::from_axis_angle(
//                                 &nalgebra::Vector3::y_axis(),
//                                 theta,
//                             );
//                             *ray.dir_mut() = Unit::new_normalize(rot * ray.dir().as_ref());
//                             ray.travel(1.0e-6);
//                         }
//                         _ => {
//                             warn!("Do not know how to handle group {}.", group);
//                             break;
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     vec![layer_0, layer_1, layer_2, layer_3, layer_4]
// }

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
    // let mut layer_0 = Array2::zeros(cam.res());

    let super_samples = cam.ss_power().pow(2);

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size as u64);
        std::mem::drop(pb);
        b
    } {
        for n in start..end {
            let xi = n as usize % cam.res().0;
            let yi = n as usize / cam.res().0;

            for n in 0..super_samples {
                let mut ray = cam.gen_ss_ray(xi, yi, n);

                while let Some((new_ray, _dist, _norm, group)) = grid.observe(ray) {
                    ray = new_ray;

                    match group {
                        _ => {
                            warn!("Don't know how to handle group {}.", group);
                            break;
                        }
                    }
                }
            }
        }
    }

    // vec![layer_0]
    vec![]
}
