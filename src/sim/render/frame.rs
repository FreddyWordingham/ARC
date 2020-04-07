//! Frame implementation.

use crate::{
    access,
    geom::Ray,
    img::{AspectRatio, Quality, Shader},
    sim::render::{Camera, Scheme},
};
use nalgebra::{Rotation3, Unit, Vector3};
use std::f64::consts::PI;

/// Frame structure.
pub struct Frame {
    /// Target aspect ratio.
    aspect_ratio: AspectRatio,
    /// Quality settings.
    quality: Quality,
    /// Shader settings.
    shader: Shader,
    /// Colour scheme gradients.
    scheme: Scheme,
    /// Camera.
    camera: Camera,
}

impl Frame {
    access!(aspect_ratio, AspectRatio);
    access!(quality, Quality);
    access!(shader, Shader);
    access!(scheme, Scheme);
    access!(camera, Camera);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        aspect_ratio: AspectRatio,
        quality: Quality,
        shader: Shader,
        scheme: Scheme,
        camera: Camera,
    ) -> Self {
        Self {
            aspect_ratio,
            quality,
            shader,
            scheme,
            camera,
        }
    }

    /// Generate a super-sampling depth-of-field ray for the corresponding pixel indices.
    #[inline]
    #[must_use]
    pub fn gen_ray(
        &self,
        xi: usize,
        yi: usize,
        sub_sample: usize,
        _depth_sample: usize,
        offset: f64,
    ) -> Ray {
        debug_assert!(xi < self.camera.res().0);
        debug_assert!(yi < self.camera.res().1);
        debug_assert!(sub_sample < self.quality.super_samples().pow(2));
        debug_assert!(offset >= 0.0);
        debug_assert!(offset <= (2.0 * PI));

        let theta = (xi as f64 * self.camera.delta().0) - (self.camera.fov().0 * 0.5);
        let phi = (yi as f64 * self.camera.delta().1) - (self.camera.fov().1 * 0.5);

        // let sx = (sub_sample % self.quality.super_samples()) as f64 + 0.5;
        // let sy = (sub_sample / self.quality.super_samples()) as f64 + 0.5;
        // theta += (sx * self.camera.sub_delta().0) - (self.camera.delta().0 * 0.5);
        // phi += (sy * self.camera.sub_delta().1) - (self.camera.delta().1 * 0.5);

        // let (r, t) = golden::circle(depth_sample as i32, self.quality.dof_samples() as i32);
        let pos = self.camera.pos();
        // pos += self.camera.right().as_ref() * (r * (t + offset).sin() * self.shader.dof_radius());
        // pos += self.camera.up().as_ref() * (r * (t + offset).cos() * self.shader.dof_radius());

        let forward = Unit::new_normalize(self.camera.tar() - pos);
        let up = Vector3::z_axis();
        let right = Unit::new_normalize(forward.cross(&up));

        let mut ray = Ray::new(*pos, forward);
        *ray.dir_mut() = Rotation3::from_axis_angle(&up, theta)
            * Rotation3::from_axis_angle(&right, phi)
            * ray.dir();

        ray
    }
}
