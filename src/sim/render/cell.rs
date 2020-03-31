use crate::geom::Aabb;

pub enum Cell {
    /// Terminal leaf cell.
    Leaf {
        /// Boundary.
        boundary: Aabb,
    },
}
