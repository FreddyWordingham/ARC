//! Core scanning function.

use crate::{
    sim::render::{Camera, Cell, Settings, Tracer},
    util::ParProgressBar,
};
use log::warn;
use ndarray::Array2;
use std::sync::{Arc, Mutex};

/// Render using a single thread.
#[allow(clippy::never_loop)]
#[allow(clippy::single_match_else)]
#[inline]
#[must_use]
pub fn run_thread(
    _id: usize,
    cam: &Camera,
    grid: &Cell,
    pb: &Arc<Mutex<ParProgressBar>>,
    block_size: usize,
    sett: &Settings,
) -> Vec<Array2<f64>> {
    let mut layer_0 = Array2::zeros(cam.res());
    let mut layer_1 = Array2::zeros(cam.res());
    let mut layer_2 = Array2::zeros(cam.res());
    let mut layer_3 = Array2::zeros(cam.res());
    let mut layer_4 = Array2::zeros(cam.res());
    let mut layer_5 = Array2::zeros(cam.res());
    let mut layer_6 = Array2::zeros(cam.res());

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
                let mut tracer = Tracer::new(cam.gen_ss_ray(xi, yi, n));

                while let Some((new_tracer, _dist, norm, group)) = grid.observe(tracer) {
                    tracer = new_tracer;

                    match group {
                        -1 | 0 => {
                            *layer_0.get_mut((xi, yi)).expect("Invalid pixel index.") += 1.0;
                            *layer_1.get_mut((xi, yi)).expect("Invalid pixel index.") +=
                                tracer.dist_travelled();

                            let amb = ambient(&sett);
                            *layer_2.get_mut((xi, yi)).expect("Invalid pixel index.") += amb;

                            let diff = diffuse(&tracer, &norm, &sett);
                            *layer_3.get_mut((xi, yi)).expect("Invalid pixel index.") += diff;

                            let spec = specular(&cam, &tracer, &norm, &sett);
                            *layer_4.get_mut((xi, yi)).expect("Invalid pixel index.") += spec;

                            let shadow = shadow(&grid, tracer.clone(), &norm, &sett);
                            *layer_5.get_mut((xi, yi)).expect("Invalid pixel index.") += shadow;

                            *layer_6.get_mut((xi, yi)).expect("Invalid pixel index.") +=
                                (amb + diff + spec) * (1.0 - shadow);

                            break;
                        }
                        1 => {
                            tracer.ray_mut().reflect(&norm);
                            tracer.travel(1.0e-6);
                        }
                        2 => {
                            tracer.ray_mut().reflect(&norm);

                            let theta = ((tracer.ray().pos().x * 6.0).sin().powi(2)
                                * (tracer.ray().pos().y * 6.0).sin().powi(2))
                                * 1.0e-1;
                            let rot = nalgebra::Rotation3::from_axis_angle(
                                &nalgebra::Vector3::y_axis(),
                                theta,
                            );
                            *tracer.ray_mut().dir_mut() =
                                Unit::new_normalize(rot * tracer.ray().dir().as_ref());
                            tracer.travel(1.0e-6);
                        }
                        _ => {
                            // warn!("Do not know how to handle group {}.", group);
                            break;
                        }
                    }
                }
            }
        }
    }

    vec![
        layer_0, layer_1, layer_2, layer_3, layer_4, layer_5, layer_6,
    ]
}

/// Calculate the ambient lighting coefficient.
fn ambient(sett: &Settings) -> f64 {
    sett.ambient()
}

/// Calculate the diffuse lighting coefficient.
use nalgebra::{Unit, Vector3};
fn diffuse(tracer: &Tracer, norm: &Unit<Vector3<f64>>, sett: &Settings) -> f64 {
    let light_dir = Unit::new_normalize(sett.sun_pos() - tracer.ray().pos());

    sett.diffuse() * norm.dot(&light_dir).abs()
}

/// Calculate the specular lighting coefficient.
fn specular(cam: &Camera, tracer: &Tracer, norm: &Unit<Vector3<f64>>, sett: &Settings) -> f64 {
    let light_dir = Unit::new_normalize(sett.sun_pos() - tracer.ray().pos());
    let view_dir = Unit::new_normalize(cam.forward().pos() - tracer.ray().pos());

    let ref_dir = reflect(&-light_dir, norm);

    sett.specular() * view_dir.dot(&ref_dir).max(0.0).powi(sett.specular_pow())
}

/// Calculate the shadowing factor.
fn shadow(grid: &Cell, mut tracer: Tracer, norm: &Unit<Vector3<f64>>, sett: &Settings) -> f64 {
    *tracer.ray_mut().dir_mut() = *norm;
    tracer.travel(1.0e-3);

    *tracer.ray_mut().dir_mut() = Unit::new_normalize(sett.sun_pos() - tracer.ray().pos());

    let mut light = 1.0;

    while let Some((new_tracer, _dist, _norm, group)) = grid.observe(tracer) {
        tracer = new_tracer;

        match group {
            -1 => {
                light *= 1.0 - sett.transparency();
            }
            0 | 1 | 2 => {
                return sett.shadow();
            }
            _ => {
                warn!("Do not know how to handle group {}.", group);
                return 0.0;
            }
        }

        tracer.travel(1.0e-3);
    }

    sett.shadow() * (1.0 - light)
}

/// Calculate the reflection vector for a given input unit vector and surface normal.
fn reflect(inc: &Unit<Vector3<f64>>, norm: &Unit<Vector3<f64>>) -> Unit<Vector3<f64>> {
    Unit::new_normalize(inc.as_ref() + (2.0 * (-inc.dot(norm)) * norm.as_ref()))
}
