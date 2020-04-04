//! Rendering pipe sub-module.

use crate::{
    geom::Ray,
    sim::panda::{lighting, Camera, Cell, ShaderSettings},
};
use palette::{LinSrgba, Srgba};

/// Determine the colour of a given ray.
#[inline]
#[must_use]
pub fn colour(sett: &ShaderSettings, cam: &Camera, root: &Cell, mut ray: Ray) -> LinSrgba {
    use palette::{Gradient, Hsv};

    let grad = Gradient::new(vec![
        Hsv::from(LinSrgba::new(1.0, 0.1, 0.1, 1.0)),
        Hsv::from(LinSrgba::new(0.1, 1.0, 1.0, 1.0)),
    ]);

    if let Some(hit) = root.observe(ray.clone(), 1.0e-6) {
        ray.travel(hit.dist());
        let mut x = lighting::ambient(sett)
            + lighting::diffuse(sett, &ray, hit.norm())
            + lighting::specular(sett, &ray, hit.norm(), cam);
        x /= 3.2;
        LinSrgba::from(grad.get(x as f32))
    // Srgba::new(1.0, 1.0, 1.0, 1.0).into_linear()
    // Srgba::new(
    //     ray.dir().dot(&nalgebra::Vector3::x_axis()).abs() as f32,
    //     ray.dir().dot(&nalgebra::Vector3::y_axis()).abs() as f32,
    //     ray.dir().dot(&nalgebra::Vector3::z_axis()).abs() as f32,
    //     1.0,
    // )
    // .into_linear()
    } else {
        Srgba::new(0.0, 0.0, 0.0, 0.1).into_linear()
    }
}
