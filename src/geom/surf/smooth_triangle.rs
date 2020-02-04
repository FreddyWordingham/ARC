//! Smooth-triangle implementation.

use crate::{
    access,
    geom::{Aabb, Collide, Emit, Ray, Trace, Transform, Triangle},
    list::Greek::{Alpha, Beta, Gamma},
};
use nalgebra::{Point3, Similarity3, Unit, Vector3};
use rand::{rngs::ThreadRng, Rng};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    result::Result,
};

/// Triangle geometry with Phong normal interpolation.
/// Used to form meshes.
pub struct SmoothTriangle {
    /// Base triangle.
    tri: Triangle,
    /// Normal vectors.
    norms: [Unit<Vector3<f64>>; 3],
}

impl SmoothTriangle {
    access!(tri, Triangle);
    access!(norms, [Unit<Vector3<f64>>; 3]);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(tri: Triangle, norms: [Unit<Vector3<f64>>; 3]) -> Self {
        assert!(norms.iter().all(|&n| n.dot(tri.plane_norm()) > 0.0));

        Self { tri, norms }
    }

    /// Construct a new instance from vertices.
    #[inline]
    #[must_use]
    pub fn new_from_verts(verts: [Point3<f64>; 3], norms: [Unit<Vector3<f64>>; 3]) -> Self {
        Self::new(Triangle::new(verts), norms)
    }

    /// Load a list of triangle from a wavefront file.
    #[inline]
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn load_list(path: &Path) -> Vec<Self> {
        let vertex_lines: Vec<_> = BufReader::new(File::open(path).expect("Unable to open file!"))
            .lines()
            .map(Result::unwrap)
            .filter(|line| line.starts_with("v "))
            .collect();

        let mut verts = Vec::with_capacity(vertex_lines.len());
        for line in vertex_lines {
            let mut words = line.split_whitespace();
            words.next();

            let px = words
                .next()
                .expect("Could not get the next word.")
                .parse::<f64>()
                .expect("Could not parse px value.");
            let py = words
                .next()
                .expect("Could not get the next word.")
                .parse::<f64>()
                .expect("Could not parse py value.");
            let pz = words
                .next()
                .expect("Could not get the next word.")
                .parse::<f64>()
                .expect("Could not parse pz value.");

            verts.push(Point3::new(px, py, pz));
        }

        let normal_lines: Vec<_> = BufReader::new(File::open(path).expect("Unable to open file!"))
            .lines()
            .map(Result::unwrap)
            .filter(|line| line.starts_with("vn "))
            .collect();

        let mut norms = Vec::with_capacity(normal_lines.len());
        for line in normal_lines {
            let mut words = line.split_whitespace();
            words.next();

            let nx = words
                .next()
                .expect("Missing normal value entry.")
                .parse::<f64>()
                .expect("Unable to parse string to usize.");
            let ny = words
                .next()
                .expect("Missing normal value entry.")
                .parse::<f64>()
                .expect("Unable to parse string to usize.");
            let nz = words
                .next()
                .expect("Missing normal value entry.")
                .parse::<f64>()
                .expect("Unable to parse string to usize.");

            norms.push(Unit::new_normalize(Vector3::new(nx, ny, nz)));
        }

        let face_lines: Vec<_> = BufReader::new(File::open(path).expect("Unable to open file!"))
            .lines()
            .map(Result::unwrap)
            .filter(|line| line.starts_with("f "))
            .collect();

        let mut faces = Vec::with_capacity(face_lines.len());
        for line in face_lines {
            let line = line.replace("//", " ");
            let mut words = line.split_whitespace();
            words.next();

            let fx = words
                .next()
                .expect("Missing face index entry.")
                .parse::<usize>()
                .expect("Unable to parse string to usize.")
                - 1;
            let nx = words
                .next()
                .expect("Missing normal index entry.")
                .parse::<usize>()
                .expect("Unable to parse string to usize.")
                - 1;
            let fy = words
                .next()
                .expect("Missing face index entry.")
                .parse::<usize>()
                .expect("Unable to parse string to usize.")
                - 1;
            let ny = words
                .next()
                .expect("Missing normal index entry.")
                .parse::<usize>()
                .expect("Unable to parse string to usize.")
                - 1;
            let fz = words
                .next()
                .expect("Missing face index entry.")
                .parse::<usize>()
                .expect("Unable to parse string to usize.")
                - 1;
            let nz = words
                .next()
                .expect("Missing normal index entry.")
                .parse::<usize>()
                .expect("Unable to parse string to usize.")
                - 1;

            faces.push(((fx, fy, fz), (nx, ny, nz)));
        }

        let mut tris = Vec::with_capacity(faces.len());
        for face in faces {
            tris.push(Self::new(
                Triangle::new([
                    *verts.get((face.0).0).expect("Missing vertex."),
                    *verts.get((face.0).1).expect("Missing vertex."),
                    *verts.get((face.0).2).expect("Missing vertex."),
                ]),
                [
                    *norms.get((face.1).0).expect("Missing normal."),
                    *norms.get((face.1).1).expect("Missing normal."),
                    *norms.get((face.1).2).expect("Missing normal."),
                ],
            ));
        }

        tris
    }
}

impl Trace for SmoothTriangle {
    #[inline]
    #[must_use]
    fn hit(&self, ray: &Ray) -> bool {
        self.tri.intersection_coors(ray).is_some()
    }

    #[inline]
    #[must_use]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        if let Some((dist, _coors)) = self.tri.intersection_coors(ray) {
            return Some(dist);
        }

        None
    }

    #[inline]
    #[must_use]
    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        if let Some((dist, [u, v, w])) = self.tri.intersection_coors(ray) {
            return Some((
                dist,
                Unit::new_normalize(
                    (self
                        .norms
                        .get(Beta as usize)
                        .expect("Missing normal.")
                        .into_inner()
                        * u)
                        + (self
                            .norms
                            .get(Gamma as usize)
                            .expect("Missing normal.")
                            .into_inner()
                            * v)
                        + (self
                            .norms
                            .get(Alpha as usize)
                            .expect("Missing normal.")
                            .into_inner()
                            * w),
                ),
            ));
        }

        None
    }

    #[inline]
    #[must_use]
    fn dist_inside(&self, ray: &Ray) -> Option<(f64, bool)> {
        if let Some(dist) = self.dist(ray) {
            Some((dist, self.tri.plane_norm().dot(ray.dir()) > 0.0))
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    fn dist_inside_norm(&self, ray: &Ray) -> Option<(f64, bool, Unit<Vector3<f64>>)> {
        if let Some((dist, norm)) = self.dist_norm(ray) {
            let inside = ray.dir().dot(self.tri.plane_norm()) > 0.0;
            Some((dist, inside, norm))
        } else {
            None
        }
    }
}

impl Collide for SmoothTriangle {
    #[inline]
    #[must_use]
    fn bounding_box(&self) -> Aabb {
        self.tri.bounding_box()
    }

    #[inline]
    #[must_use]
    fn overlap(&self, aabb: &Aabb) -> bool {
        self.tri.overlap(aabb)
    }
}

impl Transform for SmoothTriangle {
    #[inline]
    fn transform(&mut self, trans: &Similarity3<f64>) {
        self.tri.transform(trans);

        for n in &mut self.norms {
            *n = Unit::new_normalize(trans.transform_vector(n.as_ref()));
        }
    }
}

impl Emit for SmoothTriangle {
    #[inline]
    #[must_use]
    fn cast(&self, rng: &mut ThreadRng) -> Ray {
        let mut u = rng.gen::<f64>();
        let mut v = rng.gen::<f64>();

        if (u + v) > 1.0 {
            u = 1.0 - u;
            v = 1.0 - v;
        }
        let w = 1.0 - u - v;

        let edge_a_b = self
            .tri
            .verts()
            .get(Beta as usize)
            .expect("Missing vertex.")
            - self
                .tri
                .verts()
                .get(Alpha as usize)
                .expect("Missing vertex.");
        let edge_a_c = self
            .tri
            .verts()
            .get(Gamma as usize)
            .expect("Missing vertex.")
            - self
                .tri
                .verts()
                .get(Alpha as usize)
                .expect("Missing vertex.");

        let pos = self
            .tri
            .verts()
            .get(Alpha as usize)
            .expect("Missing vertex.")
            + (edge_a_b * u)
            + (edge_a_c * v);
        let dir = Unit::new_normalize(
            (self
                .norms
                .get(Beta as usize)
                .expect("Missing normal.")
                .into_inner()
                * u)
                + (self
                    .norms
                    .get(Gamma as usize)
                    .expect("Missing normal.")
                    .into_inner()
                    * v)
                + (self
                    .norms
                    .get(Alpha as usize)
                    .expect("Missing normal.")
                    .into_inner()
                    * w),
        );

        Ray::new(pos, dir)
    }
}
