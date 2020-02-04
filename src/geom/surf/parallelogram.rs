//! Parallelogram implementation.

use crate::{
    access,
    geom::{Emit, Ray, Trace},
    list::Greek::{Alpha, Beta, Gamma},
};
use nalgebra::{Point3, Unit, Vector3};
use rand::{rngs::ThreadRng, Rng};

/// Parallelogram geometry.
/// Used to form `Rectangles`.
pub struct Parallelogram {
    /// Vertex points.
    verts: [Point3<f64>; 3],
    /// Surface plane normal.
    plane_norm: Unit<Vector3<f64>>,
}

impl Parallelogram {
    access!(verts, [Point3<f64>; 3]);
    access!(plane_norm, Unit<Vector3<f64>>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(verts: [Point3<f64>; 3]) -> Self {
        let plane_norm = Unit::new_normalize(
            (verts.get(Beta as usize).expect("Missing vertex.")
                - verts.get(Alpha as usize).expect("Missing vertex."))
            .cross(
                &(verts.get(Gamma as usize).expect("Missing vertex.")
                    - verts.get(Alpha as usize).expect("Missing vertex.")),
            ),
        );

        Self { verts, plane_norm }
    }

    /// Create a pair of side vectors.
    #[must_use]
    fn edges(&self) -> (Vector3<f64>, Vector3<f64>) {
        let edge_a_b = self.verts.get(Beta as usize).expect("Missing vertex.")
            - self.verts.get(Alpha as usize).expect("Missing vertex.");
        let edge_a_c = self.verts.get(Gamma as usize).expect("Missing vertex.")
            - self.verts.get(Alpha as usize).expect("Missing vertex.");

        (edge_a_b, edge_a_c)
    }

    /// Calculate the perimeter length.
    #[inline]
    #[must_use]
    pub fn perimeter(&self) -> f64 {
        let ab = nalgebra::distance(
            self.verts.get(Alpha as usize).expect("Missing vertex."),
            self.verts.get(Beta as usize).expect("Missing vertex."),
        );
        let bc = nalgebra::distance(
            self.verts.get(Beta as usize).expect("Missing vertex."),
            self.verts.get(Gamma as usize).expect("Missing vertex."),
        );

        (ab + bc) * 2.0
    }

    /// Calculate the surface area.
    #[inline]
    #[must_use]
    pub fn area(&self) -> f64 {
        let ab = nalgebra::distance(
            self.verts.get(Alpha as usize).expect("Missing vertex."),
            self.verts.get(Beta as usize).expect("Missing vertex."),
        );
        let bc = nalgebra::distance(
            self.verts.get(Beta as usize).expect("Missing vertex."),
            self.verts.get(Gamma as usize).expect("Missing vertex."),
        );
        let ca = nalgebra::distance(
            self.verts.get(Gamma as usize).expect("Missing vertex."),
            self.verts.get(Alpha as usize).expect("Missing vertex."),
        );

        let s = (ab + bc + ca) * 0.5;

        (s * (s - ab) * (s - bc) * (s - ca)).sqrt() * 2.0
    }

    /// Centre point.
    #[inline]
    #[must_use]
    pub fn centre(&self) -> Point3<f64> {
        let (edge_a_b, edge_a_c) = self.edges();

        self.verts.get(Alpha as usize).expect("Missing vertex.")
            + (0.5 * edge_a_b)
            + (0.5 * edge_a_c)
    }

    /// Determine the intersection distance along a ray's direction.
    /// Also return the barycentric intersection coordinates.
    #[inline]
    #[must_use]
    pub fn intersection_coors(&self, ray: &Ray) -> Option<(f64, [f64; 2])> {
        let verts = self.verts;

        let (e1, e2) = self.edges();

        let d_cross_e2 = ray.dir().cross(&e2);
        let e1_dot_d_cross_e2 = e1.dot(&d_cross_e2);

        if e1_dot_d_cross_e2.abs() <= 0.0 {
            return None;
        }

        let inv_e1_dot_d_cross_e2 = 1.0 / e1_dot_d_cross_e2;
        let rel_pos = ray.pos() - verts.get(Alpha as usize).expect("Missing vertex.");
        let u = inv_e1_dot_d_cross_e2 * rel_pos.dot(&d_cross_e2);

        if (u < 0.0) || (u > 1.0) {
            return None;
        }

        let q = rel_pos.cross(&e1);
        let v = inv_e1_dot_d_cross_e2 * ray.dir().dot(&q);

        if (v < 0.0) || (v > 1.0) {
            return None;
        }

        let dist = inv_e1_dot_d_cross_e2 * e2.dot(&q);

        if dist <= 0.0 {
            return None;
        }

        Some((dist, [u, v]))
    }
}

impl Trace for Parallelogram {
    #[inline]
    #[must_use]
    fn hit(&self, ray: &Ray) -> bool {
        self.intersection_coors(ray).is_some()
    }

    #[inline]
    #[must_use]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        if let Some((dist, _coors)) = self.intersection_coors(ray) {
            return Some(dist);
        }

        None
    }

    #[inline]
    #[must_use]
    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        if let Some((dist, _coors)) = self.intersection_coors(ray) {
            return Some((dist, self.plane_norm));
        }

        None
    }

    #[inline]
    #[must_use]
    fn dist_inside(&self, ray: &Ray) -> Option<(f64, bool)> {
        if let Some(dist) = self.dist(ray) {
            Some((dist, self.plane_norm.dot(ray.dir()) > 0.0))
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    fn dist_inside_norm(&self, ray: &Ray) -> Option<(f64, bool, Unit<Vector3<f64>>)> {
        if let Some(dist) = self.dist(ray) {
            Some((dist, self.plane_norm.dot(ray.dir()) > 0.0, self.plane_norm))
        } else {
            None
        }
    }
}

impl Emit for Parallelogram {
    #[inline]
    #[must_use]
    fn cast(&self, rng: &mut ThreadRng) -> Ray {
        let (edge_a_b, edge_a_c) = self.edges();

        let pos = self.verts.get(Alpha as usize).expect("Missing vertex.")
            + (edge_a_b * rng.gen::<f64>())
            + (edge_a_c * rng.gen::<f64>());

        Ray::new(pos, self.plane_norm)
    }
}
