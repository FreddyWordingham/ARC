//! Palette colours input settings implementation.

use crate::{access, sim::render::Group};
use attr::json;
use palette::{Gradient, LinSrgba};
use std::collections::HashMap;

/// Palette settings.
#[json]
pub struct Palette {
    /// Group colours.
    grads: Vec<(Group, Vec<[f64; 4]>)>,
}

impl Palette {
    access!(grads, Vec<(Group, Vec<[f64; 4]>)>);

    /// Build a complete instance.
    #[inline]
    #[must_use]
    pub fn build(&self) -> HashMap<Group, Gradient<LinSrgba>> {
        let mut list = HashMap::with_capacity(self.grads.len());

        for (group, cols) in &self.grads {
            if list.contains_key(group) {
                panic!("Duplicate gradient for group: {}", group);
            }

            list.insert(
                *group,
                Gradient::new(
                    cols.iter()
                        .map(|col| {
                            LinSrgba::new(
                                *col.get(0).expect("Missing alpha channel entry.") as f32,
                                *col.get(1).expect("Missing red channel entry.") as f32,
                                *col.get(2).expect("Missing blue channel entry.") as f32,
                                *col.get(3).expect("Missing green channel entry.") as f32,
                            )
                        })
                        .collect::<Vec<_>>(),
                ),
            );
        }

        list
    }
}
