//! Palette implementation.

use crate::{access, rend::Group};
use palette::{Gradient, LinSrgba};
use std::collections::HashMap;

/// Image colouring structure.
pub struct Palette {
    /// Group colours.
    grads: HashMap<Group, Gradient<LinSrgba>>,
}

impl Palette {
    access!(grads, HashMap<Group, Gradient<LinSrgba>>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(grads: HashMap<Group, Gradient<LinSrgba>>) -> Self {
        Self { grads }
    }
}
