//! Camera structure.

use crate::{
    access, clone,
    list::Cartesian::{X, Y},
};
use attr::json;
use nalgebra::{Point3, Unit, Vector3};
use ndarray::Array2;

/// Image forming structure.
#[json]
pub struct Camera {
    /// Viewing position.
    pos: Point3<f64>,
    /// Forward (viewing) direction.
    dir: Unit<Vector3<f64>>,
    /// Right direction.
    right: Unit<Vector3<f64>>,
    /// Up direction.
    up: Unit<Vector3<f64>>,
    /// Field of view.
    fov: f64,
}

impl Camera {
    access!(pos, Point3<f64>);
    access!(dir, Unit<Vector3<f64>>);
    access!(right, Unit<Vector3<f64>>);
    access!(up, Unit<Vector3<f64>>);
    clone!(fov, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(pos: Point3<f64>, dir: Unit<Vector3<f64>>, fov: f64) -> Self {
        debug_assert!(fov > 0.0);

        let right = Unit::new_normalize(dir.cross(&Vector3::z_axis()));
        let up = Unit::new_normalize(dir.cross(&right));

        Self {
            pos,
            dir,
            right,
            up,
            fov,
        }
    }

    /// Observe a weighted point.
    #[inline]
    #[must_use]
    pub fn observe(&self, img: &mut Array2<f64>, p: &Point3<f64>, w: f64) {
        let obs = Unit::new_normalize(p - self.pos);

        let shape = img.shape();
        let h_fov_x = self.fov / 2.0;
        let h_fov_y = h_fov_x
            * (*shape.get(Y as usize).expect("Invalid index.") as f64
                / *shape.get(X as usize).expect("Invalid index.") as f64);

        let phi = (self.right.dot(&obs)).asin();
        if phi.abs() > h_fov_x {
            return;
        }

        let theta = (self.up.dot(&obs)).asin();
        if theta.abs() > h_fov_y {
            return;
        }

        let dx = self.fov / shape[X as usize] as f64;

        let x = ((phi + h_fov_x) / dx).floor() as usize;
        let y = ((theta + h_fov_y) / dx).floor() as usize;

        *img.get_mut([x, y]).expect("Invalid index.") += w;
    }
}
