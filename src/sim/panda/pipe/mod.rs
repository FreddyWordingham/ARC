//! Rendering pipe sub-module.

use crate::{
    geom::Ray,
    sim::panda::{lighting, Cell, ShaderSettings},
};
use nalgebra::Point3;
use palette::Gradient;
use palette::{LinSrgba, Srgba};
use rand::rngs::ThreadRng;

/// Determine the colour of a given ray.
#[inline]
#[must_use]
pub fn colour(
    sett: &ShaderSettings,
    cam_pos: &Point3<f64>,
    root: &Cell,
    mut ray: Ray,
    bump_dist: f64,
    rng: &mut ThreadRng,
) -> LinSrgba {
    debug_assert!(bump_dist > 0.0);

    // let grad = Gradient::new(vec![
    //     palette::Hsv::from(LinSrgba::new(1.0, 0.1, 0.1, 1.0)),
    //     palette::Hsv::from(LinSrgba::new(0.1, 1.0, 1.0, 1.0)),
    // ]);
    let grad_0 = Gradient::new(vec![
        LinSrgba::new(0.0, 0.0, 0.0, 1.0),
        LinSrgba::new(1.0, 1.0, 1.0, 1.0),
    ]);

    while let Some(hit) = root.observe(ray.clone(), bump_dist) {
        ray.travel(hit.dist());
        let mut x = (lighting::ambient(sett)
            + lighting::diffuse(sett, &ray, hit.norm())
            + lighting::specular(sett, &ray, hit.norm(), cam_pos))
            // * lighting::sunlight(sett, &ray, hit.norm(), root, bump_dist);
        // * lighting::sunlight_samples(sett, &ray, hit.norm(), root, bump_dist, rng);
        * lighting::casting_samples(sett, &ray, hit.norm(), root, bump_dist, rng);
        x /= 3.2;

        match hit.group() {
            -2 => {
                ray.refract(hit.norm(), 1.0, 1.05);
                ray.travel(bump_dist);
                // if let Some(second_hit) = root.observe(ray.clone(), bump_dist) {
                //     ray.travel(hit.dist());
                //     ray.refract(second_hit.norm(), 1.1, 1.0);
                //     ray.travel(bump_dist);
                // } else {
                //     return LinSrgba::new(1.0, 0.0, 1.0, 1.0);
                // }

                return (LinSrgba::from(grad_0.get(x as f32)) * 0.1)
                    + (colour(sett, cam_pos, root, ray, bump_dist, rng) * 0.9);
            }
            -3 => {
                ray.reflect(hit.norm());
                let theta = ((ray.pos().x * 6.0).sin().powi(2) * (ray.pos().y * 6.0).sin().powi(2))
                    * 0.5e-1;
                let rot = nalgebra::Rotation3::from_axis_angle(&nalgebra::Vector3::y_axis(), theta);
                *ray.dir_mut() = nalgebra::Unit::new_normalize(rot * ray.dir().as_ref());
                ray.travel(bump_dist);

                return colour(sett, cam_pos, root, ray, bump_dist, rng);
            }
            -1 => {
                ray.reflect(hit.norm());
                ray.travel(bump_dist);
                return (LinSrgba::from(grad_0.get(x as f32)) * 0.1)
                    + (colour(sett, cam_pos, root, ray, bump_dist, rng) * 0.9);
            }
            0 => {
                return LinSrgba::from(grad_0.get(x as f32));
            }
            1..=3 => {
                return LinSrgba::from(grad_0.get(x as f32));
            }
            _ => {
                panic!("Don't know how to handle group {}!", hit.group());
            }
        }
    }
    Srgba::new(0.0, 0.0, 0.0, 0.1).into_linear()
}
