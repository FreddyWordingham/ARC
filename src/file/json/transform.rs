//! Transform implementation.

use attr::json;
use nalgebra::{Similarity3, Translation3, UnitQuaternion, Vector3};

/// Json parsable transform structure.
#[json]
pub struct Transform {
    /// Optional translation to apply.
    trans: Option<Translation3<f64>>,
    /// Rotation applied as Euler angles.
    rot: Option<Vector3<f64>>,
    /// Optional uniform scaling to apply.
    scale: Option<f64>,
}

impl Transform {
    /// Build a transformation.
    #[inline]
    #[must_use]
    pub fn build(&self) -> Similarity3<f64> {
        let trans = self
            .trans
            .unwrap_or_else(|| Translation3::new(0.0, 0.0, 0.0));
        let rot = self.rot.unwrap_or_else(|| Vector3::new(0.0, 0.0, 0.0));
        let rot = UnitQuaternion::from_euler_angles(
            rot.x.to_radians(),
            rot.y.to_radians(),
            rot.z.to_radians(),
        );
        let scale = self.scale.unwrap_or(1.0);

        Similarity3::from_parts(trans, rot, scale)
    }
}
