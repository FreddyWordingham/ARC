//! Colour scheme implementation.

use crate::{access, sim::render::Group};
use palette::{Gradient, LinSrgba};
use std::collections::BTreeMap;

/// Group colour gradients.
pub struct Scheme {
    grads: BTreeMap<Group, Gradient<LinSrgba>>,
}

impl Scheme {
    access!(grads, BTreeMap<Group, Gradient<LinSrgba>>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(grads: BTreeMap<Group, Gradient<LinSrgba>>) -> Self {
        Self { grads }
    }
}
