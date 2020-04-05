//! Scene input settings implementation.

use crate::{access, rend::Group};
use attr::json;
// use std::fmt::{Display, Formatter, Result};

/// Scene settings.
#[json]
pub struct Scene {
    /// Traceable surfaces.
    surfaces: Vec<(Group, Vec<(String, Option<FileTransform>)>)>,
}

impl Scene {
    access!(surfaces, Vec<(Group, Vec<(String, Option<FileTransform>)>)>);
}

// impl Display for Scene {
//     fn fmt(&self, fmt: &mut Formatter) -> Result {
//         writeln!(fmt)?;
//         writeln!(fmt, "{:>30} : {}", "target triangles", self.tar_tris)?;
//         writeln!(fmt, "{:>30} : {}", "max depth", self.max_depth)?;
//         writeln!(
//             fmt,
//             "{:>30} : {}%",
//             "collision detection padding",
//             self.padding * 100.0
//         )
//     }
// }
