//! Camera implementation.

use crate::geom::Ray;
use crate::{access, clone};
use nalgebra::{Unit, Vector3};

/// Image forming camera.
pub struct Camera {
    /// Forward direction.
    forward: Ray,
    /// Up axis.
    up: Unit<Vector3<f64>>,
    /// Right axis.
    right: Unit<Vector3<f64>>,
    /// Field of view.
    fov: (f64, f64),
    /// Image resolution.
    res: (usize, usize),
    /// Scanning deltas.
    delta: (f64, f64),
}

impl Camera {
    access!(forward, Ray);
    access!(up, Vector3<f64>);
    access!(right, Vector3<f64>);
    clone!(fov, (f64, f64));
    clone!(res, (usize, usize));
    clone!(delta, (f64, f64));
}
