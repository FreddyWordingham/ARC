//! Hit-type enumeration.

/// Hit enumeration implementation.
#[derive(Debug)]
pub enum Hit {
    /// Cell collision.
    Cell(f64),
    /// Interface collision.
    Interface(f64),
}

impl Hit {
    /// Construct a new instance.
    pub fn new(cell_dist: f64, inter_dist: Option<f64>) -> Self {
        Self::Cell(cell_dist)
    }
}
