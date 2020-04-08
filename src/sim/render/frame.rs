//! Frame implementation.

use crate::util::{ParProgressBar, ProgressBar};
use crate::{
    access,
    geom::Ray,
    img::{AspectRatio, Quality, Shader},
    math::sample::golden,
    sim::render::{Camera, Grid, PipeFunc, Scheme},
};
use nalgebra::{Point3, Rotation3, Unit, Vector3};
use ndarray::Array2;
use palette::LinSrgba;
use rand::thread_rng;
use rand::{rngs::ThreadRng, Rng};
use rayon::prelude::*;
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

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

    /// Run a rendering simulation.
    #[inline]
    #[must_use]
    pub fn image(&self, pipe: PipeFunc, grid: &Grid) -> Array2<LinSrgba> {
        debug_assert!(self.camera.res().0 % self.quality.section_splits().0 == 0);
        debug_assert!(self.camera.res().1 % self.quality.section_splits().1 == 0);

        let num_sections = self.quality.section_splits().0 * self.quality.section_splits().1;
        let pb = ParProgressBar::new("Rendering", num_sections as u64);
        let pb = Arc::new(Mutex::new(pb));

        let sections: Vec<usize> = (0..num_sections).collect();
        let sections: Vec<(usize, Array2<LinSrgba>)> = sections
            .par_iter()
            .map(|index| {
                pb.lock().expect("Could not lock progress bar.").tick();
                let section = self.render_section(pipe, *index, grid);
                (*index, section)
            })
            .collect();
        pb.lock()
            .expect("Could not lock progress bar.")
            .finish_with_message("Render complete.");

        self.stitch(sections)
    }

    /// Render a section of the image.
    #[inline]
    #[must_use]
    fn render_section(&self, pipe: PipeFunc, index: usize, grid: &Grid) -> Array2<LinSrgba> {
        let section_res = self.camera.frame_res(self.quality.section_splits());

        let fx = index % self.quality.section_splits().0;
        let fy = index / self.quality.section_splits().0;
        let start = (section_res.0 * fx, section_res.1 * fy);

        let mut rng = thread_rng();
        let mut section = Array2::default(section_res);

        for xi in 0..section_res.0 {
            let rx = start.0 + xi; // TODO: Put this into xi
            for yi in 0..section_res.1 {
                let ry = start.1 + yi;

                *section
                    .get_mut((xi, yi))
                    .expect("Could not access section pixel.") =
                    self.colour_pixel(pipe, (rx, ry), grid, &mut rng);
            }
        }

        section
    }

    /// Calculate the colour of a pixel.
    #[inline]
    #[must_use]
    pub fn colour_pixel(
        &self,
        pipe: PipeFunc,
        pixel: (usize, usize),
        grid: &Grid,
        rng: &mut ThreadRng,
    ) -> LinSrgba {
        let super_samples = self.quality.super_samples().unwrap_or(1);
        let dof_samples = self.quality.dof_samples().unwrap_or(1);

        let weighting = 1.0 / (super_samples * dof_samples) as f32;

        let mut col = LinSrgba::default();

        for sub_sample in 0..super_samples {
            let offset = rng.gen_range(0.0, 2.0 * PI);
            for depth_sample in 0..dof_samples {
                let ray = self.gen_ray(pixel, offset, sub_sample, depth_sample);

                col += pipe(
                    &ray.pos().clone(),
                    grid,
                    &self.shader,
                    &self.scheme,
                    ray,
                    rng,
                ) * weighting;
            }
        }

        col
    }

    /// Generate a new observation ray.
    #[inline]
    #[must_use]
    pub fn gen_ray(
        &self,
        pixel: (usize, usize),
        offset: f64,
        sub_sample: usize,
        depth_sample: usize,
    ) -> Ray {
        let pos = if let Some(dof_samples) = self.quality.dof_samples() {
            debug_assert!(dof_samples > 1);

            let (rho, theta) = golden::circle(depth_sample as i32, dof_samples as i32);
            let mut pos = *self.camera.pos();

            let forward = Unit::new_normalize(self.camera.tar() - pos);
            let up = Vector3::z_axis();
            let right = Unit::new_normalize(forward.cross(&up));

            pos += right.as_ref() * (rho * (theta + offset).sin() * self.shader.dof_radius());
            pos += up.as_ref() * (rho * (theta + offset).cos() * self.shader.dof_radius());

            pos
        } else {
            *self.camera.pos()
        };

        if self.quality.super_samples().is_some() {
            return self.gen_ss_ray(pos, pixel, sub_sample);
        }

        self.gen_pix_ray(pos, pixel)
    }

    /// Generate a sub-pixel ray.
    #[inline]
    #[must_use]
    pub fn gen_ss_ray(&self, pos: Point3<f64>, pixel: (usize, usize), sub_sample: usize) -> Ray {
        let (xi, yi) = pixel;

        let forward = Unit::new_normalize(self.camera.tar() - pos);
        let up = Vector3::z_axis();
        let right = Unit::new_normalize(forward.cross(&up));

        let mut theta = (xi as f64 * self.camera.delta().0) - (self.camera.fov().0 * 0.5);
        let mut phi = (yi as f64 * self.camera.delta().1) - (self.camera.fov().1 * 0.5);

        let super_samples = self
            .quality
            .super_samples()
            .expect("Bad attempt super sample.");
        let sub_delta = self.camera.sub_delta().expect("Bad attempt super sample.");
        let sx = (sub_sample % super_samples) as f64 + 0.5;
        let sy = (sub_sample / super_samples) as f64 + 0.5;
        theta += (sx * sub_delta.0) - (self.camera.delta().0 * 0.5);
        phi += (sy * sub_delta.1) - (self.camera.delta().1 * 0.5);

        let mut ray = Ray::new(pos, forward);
        *ray.dir_mut() = Rotation3::from_axis_angle(&up, theta)
            * Rotation3::from_axis_angle(&right, phi)
            * ray.dir();

        ray
    }

    /// Generate a pixel central ray.
    #[inline]
    #[must_use]
    pub fn gen_pix_ray(&self, pos: Point3<f64>, pixel: (usize, usize)) -> Ray {
        let (xi, yi) = pixel;

        let forward = Unit::new_normalize(self.camera.tar() - pos);
        let up = Vector3::z_axis();
        let right = Unit::new_normalize(forward.cross(&up));

        let theta = (xi as f64 * self.camera.delta().0) - (self.camera.fov().0 * 0.5);
        let phi = (yi as f64 * self.camera.delta().1) - (self.camera.fov().1 * 0.5);

        let mut ray = Ray::new(pos, forward);
        *ray.dir_mut() = Rotation3::from_axis_angle(&up, theta)
            * Rotation3::from_axis_angle(&right, phi)
            * ray.dir();

        ray
    }

    /// Stitch together the stack of sections.
    #[inline]
    #[must_use]
    fn stitch(&self, sections: Vec<(usize, Array2<LinSrgba>)>) -> Array2<LinSrgba> {
        let mut img = unsafe { Array2::uninitialized(self.camera.res()) };

        let img_res = self.camera.res();
        let section_res = (
            img_res.0 / self.quality.section_splits().0,
            img_res.1 / self.quality.section_splits().1,
        );

        let mut pb = ProgressBar::new("Stitching", sections.len() as u64);
        for (index, section) in sections {
            pb.tick();

            let nx = index % self.quality.section_splits().0;
            let ny = index / self.quality.section_splits().0;

            let start_x = nx * section_res.0;
            let start_y = ny * section_res.1;

            for (px, fx) in (start_x..(start_x + section_res.0)).zip(0..section_res.0) {
                for (py, fy) in (start_y..(start_y + section_res.1)).zip(0..section_res.1) {
                    *img.get_mut((px, py)).expect("Could not write to image.") =
                        *section.get((fx, fy)).expect("Could not read from frame.");
                }
            }
        }
        pb.finish_with_message("Stitching complete.");

        img
    }
}
