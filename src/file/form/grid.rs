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
<<<<<<< HEAD
    pub fn form<'a>(&self, num_threads: usize, verse: &'a Verse) -> WorldGrid<'a> {
=======
    pub fn form(&self, num_threads: usize, verse: &Verse) -> WorldGrid {
>>>>>>> 671c3d8935608ac0c3232ccb50f845e19b0e7372
        assert!(num_threads > 0);

        WorldGrid::new(
            num_threads,
            Aabb::new(self.mins, self.maxs),
            self.res,
            verse,
        )
    }
}
