//! Colour scheme implementation.

use crate::{access, sim::render::Group};
use palette::{Gradient, LinSrgba};
use std::collections::BTreeMap;

/// Group colour gradients.
pub struct Scheme {
    /// Colour scheme gradients.
    grads: BTreeMap<Group, Gradient<LinSrgba>>,
    /// Backup gradient.
    backup: Gradient<LinSrgba>,
}

impl Scheme {
    access!(grads, BTreeMap<Group, Gradient<LinSrgba>>);
    access!(backup, Gradient<LinSrgba>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(grads: BTreeMap<Group, Gradient<LinSrgba>>) -> Self {
        Self {
            grads,
            backup: Gradient::new(vec![
                LinSrgba::new(0.0, 0.0, 0.0, 1.0),
                LinSrgba::new(1.0, 1.0, 1.0, 1.0),
            ]),
        }
    }

    /// Retrieve a reference to a colour gradient.
    #[inline]
    #[must_use]
    pub fn get(&self, group: Group) -> &Gradient<LinSrgba> {
        if let Some(grad) = self.grads.get(&group) {
            grad
        } else {
            &self.backup
        }
    }
}
