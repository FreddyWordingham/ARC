//! Trace trait.

use crate::geom::{Ray, Side};
use nalgebra::{Point3, Unit, Vector3};

/// Trace trait implementation.
/// Types implementing this trait can be traced using rays.
pub trait Trace {
    /// Determine if a ray hit occurs.
    fn hit(&self, ray: &Ray) -> bool;

    /// Distance to the surface along the ray's line of travel.
    fn dist(&self, ray: &Ray) -> Option<f64>;

    /// Distance to the surface along the ray's line of travel and normal unit vector at the point of collision.
    fn dist_norm(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)>;

    /// Distance to the surface along the ray's line of travel and side of collision.
    fn dist_inside(&self, ray: &Ray) -> Option<(f64, bool)>;

    /// Distance to the surface along the ray's line of travel and side of collision.
    fn dist_side(&self, ray: &Ray) -> Option<(f64, Side)> {
        if let Some((dist, norm)) = self.dist_norm(ray) {
            Some((dist, Side::new(ray.dir(), norm)))
        } else {
            None
        }
    }

    /// Distance to the surface along the ray's line of travel, side of collision, and normal unit vector at the point of collision.
    fn dist_inside_norm(&self, ray: &Ray) -> Option<(f64, bool, Unit<Vector3<f64>>)>;

    /// Calculate the hit point of a ray.
    #[inline]
    #[must_use]
    fn hit_point(&self, ray: &Ray) -> Option<Point3<f64>> {
        if let Some(dist) = self.dist(ray) {
            let mut scan = ray.clone();
            scan.travel(dist);
            Some(*scan.pos())
        } else {
            None
        }
    }
}
