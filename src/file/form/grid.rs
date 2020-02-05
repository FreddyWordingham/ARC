//! Grid implementation.

use crate::{
    access,
    geom::Aabb,
    world::{Grid as WorldGrid, Verse},
};
use attr::json;
use nalgebra::Point3;

/// Grid construction form.
#[json]
pub struct Grid {
    /// Grid resolution.
    res: [usize; 3],
    /// Min point.
    mins: Point3<f64>,
    /// Max point.
    maxs: Point3<f64>,
}

impl Grid {
    access!(res, [usize; 3]);
    access!(mins, Point3<f64>);
    access!(maxs, Point3<f64>);

    /// Form a new instance.
    #[inline]
    #[must_use]
    pub fn form(&self, _verse: &Verse) -> WorldGrid {
        WorldGrid::new(Aabb::new(self.mins, self.maxs))
    }
}
