//! Scene input settings implementation.

use crate::{access, file::Transform, rend::Group, util::print::banner::term_width};
use attr::json;
use nalgebra::Point3;
use std::fmt::{Display, Formatter, Result};

/// Scene settings.
#[json]
pub struct Scene {
    /// Sun position.
    sun_pos: Point3<f64>,
    /// Traceable surface groups.
    groups: Vec<(Group, Vec<(String, Option<Transform>)>)>,
}

impl Scene {
    access!(sun_pos, Point3<f64>);
    access!(groups, Vec<(Group, Vec<(String, Option<Transform>)>)>);
}

impl Display for Scene {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt)?;
        writeln!(fmt, "{:>30} : {} [m]", "sun position", self.sun_pos)?;
        writeln!(fmt, "{:>30} : {}", "number of groups", self.groups.len())?;

        let cols = ((term_width() - 12) / 28).min(4).max(1);
        let mut add_newline = false;
        for (group, meshes) in &self.groups {
            write!(fmt, "[{:^3}] : ", group)?;
            for (i, (name, trans)) in meshes.iter().enumerate() {
                write!(
                    fmt,
                    "[{:^24}]{} ",
                    name,
                    if trans.is_some() { "*" } else { " " }
                )?;
                if i % cols == cols - 1 {
                    writeln!(fmt)?;
                    write!(fmt, "        ")?;
                }
            }

            add_newline = meshes.len() % cols != 0;
        }

        if add_newline {
            writeln!(fmt)?;
        }

        Ok(())
    }
}
