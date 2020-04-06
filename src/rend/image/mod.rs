//! Image implementation.

pub mod camera;

pub use self::camera::*;

use crate::access;

/// Image structure.
pub struct Image {
    /// Camera.
    cam: Camera,
}

impl Image {
    access!(cam, Camera);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(cam: Camera) -> Self {
        Self { cam }
    }
}
