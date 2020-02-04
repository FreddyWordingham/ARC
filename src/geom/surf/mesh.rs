//! Mesh implementation.

use crate::{
    access,
    file::Load,
    geom::{Aabb, Collide, Emit, Ray, SmoothTriangle, Trace, Transform},
    list::Greek::Alpha,
};
use nalgebra::{Similarity3, Unit, Vector3};
use rand::{rngs::ThreadRng, Rng};
use std::path::Path;

/// Mesh geometry.
pub struct Mesh {
    /// Bounding box.
    aabb: Aabb,
    /// List of component triangles.
    tris: Vec<SmoothTriangle>,
}

impl Mesh {
    access!(aabb, Aabb);
    access!(tris, Vec<SmoothTriangle>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(tris: Vec<SmoothTriangle>) -> Self {
        Self {
            aabb: Self::init_aabb(&tris),
            tris,
        }
    }

    /// Initialise the bounding box for the mesh.
    fn init_aabb(tris: &[SmoothTriangle]) -> Aabb {
        let mut mins = *tris
            .get(0)
            .expect("No triangles.")
            .tri()
            .verts()
            .get(Alpha as usize)
            .expect("Missing vertex.");
        let mut maxs = mins;

        for tri in tris {
            for v in tri.tri().verts().iter() {
                for (v, (min, max)) in v.iter().zip(mins.iter_mut().zip(maxs.iter_mut())) {
                    if *min > *v {
                        *min = *v;
                    } else if *max < *v {
                        *max = *v;
                    }
                }
            }
        }

        Aabb::new(mins, maxs)
    }

    /// Calculate the surface area.
    #[inline]
    #[must_use]
    pub fn area(&self) -> f64 {
        let mut area = 0.0;

        for tri in &self.tris {
            area += tri.tri().area();
        }

        area
    }
}

impl Trace for Mesh {
    #[inline]
    #[must_use]
    fn hit(&self, ray: &Ray) -> bool {
        if !self.aabb.hit(ray) {
            return false;
        }

        self.tris.iter().any(|t| t.hit(ray))
    }

    #[inline]
    #[must_use]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .filter_map(|tri| tri.dist(ray))
            .min_by(|a, b| a.partial_cmp(b).expect("Failed comparison."))
    }

    #[inline]
    #[must_use]
    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .filter_map(|tri| tri.dist_norm(ray))
            .min_by(|a, b| a.0.partial_cmp(&b.0).expect("Failed comparison."))
    }

    #[inline]
    #[must_use]
    fn dist_inside(&self, ray: &Ray) -> Option<(f64, bool)> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .filter_map(|tri| tri.dist_inside(ray))
            .min_by(|a, b| a.0.partial_cmp(&b.0).expect("Failed comparison."))
    }

    #[inline]
    #[must_use]
    fn dist_inside_norm(&self, ray: &Ray) -> Option<(f64, bool, Unit<Vector3<f64>>)> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .filter_map(|tri| tri.dist_inside_norm(ray))
            .min_by(|a, b| a.0.partial_cmp(&b.0).expect("Failed comparison."))
    }
}

impl Collide for Mesh {
    #[inline]
    #[must_use]
    fn bounding_box(&self) -> Aabb {
        self.aabb.clone()
    }

    #[inline]
    #[must_use]
    fn overlap(&self, aabb: &Aabb) -> bool {
        if !self.aabb.overlap(aabb) {
            return false;
        }

        for tri in &self.tris {
            if tri.overlap(aabb) {
                return true;
            }
        }

        false
    }
}

impl Transform for Mesh {
    #[inline]
    fn transform(&mut self, trans: &Similarity3<f64>) {
        for tri in &mut self.tris {
            tri.transform(trans);
        }

        self.aabb = Self::init_aabb(&self.tris);
    }
}

impl Emit for Mesh {
    #[inline]
    #[must_use]
    fn cast(&self, rng: &mut ThreadRng) -> Ray {
        let areas: ndarray::Array1<f64> = self.tris.iter().map(|tri| tri.tri().area()).collect();
        let total_area = areas.sum();

        let r = rng.gen_range(0.0, total_area);
        let mut sum = 0.0;
        for (area, tri) in areas.iter().zip(self.tris.iter()) {
            sum += area;
            if sum > r {
                return tri.cast(rng);
            }
        }

        unreachable!("Can not be here...");
    }
}

impl Load for Mesh {
    #[inline]
    #[must_use]
    fn load(path: &Path) -> Self {
        Self::new(SmoothTriangle::load_list(path))
    }
}
