//! Scene input settings implementation.

use crate::{access, file::Transform, rend::Group, util::print::format};
use attr::json;
use nalgebra::Point3;
use std::fmt::{Display, Formatter, Result};

/// Scene settings.
#[allow(clippy::type_complexity)]
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
        for (group, meshes) in &self.groups {
            let names: Vec<_> = meshes.iter().map(|(name, _trans)| name.clone()).collect();
            writeln!(
                fmt,
                "{:>30} : {} meshes",
                format!("group [{:^3}]", group),
                meshes.len()
            )?;
            writeln!(fmt, "{}", format::cols(&names, 4, 24))?;
        }

        Ok(())
    }
}
