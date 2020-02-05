//! Rectangle implementation.

use crate::{
    access,
    geom::{Emit, Parallelogram, Ray, Trace},
};
use nalgebra::{Point3, Unit, Vector3};
use rand::rngs::ThreadRng;
use std::f64::consts::FRAC_PI_2;

/// Maximum absolute deviation [rad].
const RIGHT_ANGLE_TOLERANCE: f64 = 1.0e-3;

/// Rectangle geometry.
/// This is a special case of Parallelogram.
#[derive(Debug, Clone)]
pub struct Rectangle {
    /// Base parallelogram.
    para: Parallelogram,
}

impl Rectangle {
    access!(para, Parallelogram);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(para: Parallelogram) -> Self {
        let ab = para.verts().get(1).unwrap() - para.verts().get(0).unwrap();
        let ac = para.verts().get(2).unwrap() - para.verts().get(0).unwrap();

        if (ab.angle(&ac) - FRAC_PI_2).abs() > RIGHT_ANGLE_TOLERANCE {
            panic!("Rectangle points do not form a right angle.");
        }

        Self { para }
    }

    /// Construct a new instance from three vertex points.
    #[inline]
    #[must_use]
    pub fn from_points(verts: [Point3<f64>; 3]) -> Self {
        Self::new(Parallelogram::new(verts))
    }
}

impl Trace for Rectangle {
    #[inline]
    #[must_use]
    fn hit(&self, ray: &Ray) -> bool {
        self.para.intersection_coors(ray).is_some()
    }

    #[inline]
    #[must_use]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        if let Some((dist, _coors)) = self.para.intersection_coors(ray) {
            return Some(dist);
        }

        None
    }

    #[inline]
    #[must_use]
    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        if let Some((dist, _coors)) = self.para.intersection_coors(ray) {
            return Some((dist, *self.para.plane_norm()));
        }

        None
    }

    #[inline]
    #[must_use]
    fn dist_inside(&self, ray: &Ray) -> Option<(f64, bool)> {
        if let Some(dist) = self.para.dist(ray) {
            Some((dist, self.para.plane_norm().dot(ray.dir()) > 0.0))
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    fn dist_inside_norm(&self, ray: &Ray) -> Option<(f64, bool, Unit<Vector3<f64>>)> {
        if let Some(dist) = self.para.dist(ray) {
            Some((
                dist,
                self.para.plane_norm().dot(ray.dir()) > 0.0,
                *self.para.plane_norm(),
            ))
        } else {
            None
        }
    }
}

impl Emit for Rectangle {
    #[inline]
    #[must_use]
    fn cast(&self, rng: &mut ThreadRng) -> Ray {
        self.para.cast(rng)
    }
}
