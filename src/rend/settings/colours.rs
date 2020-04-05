//! Colours input settings implementation.

// use crate::clone;
use attr::json;
// use std::fmt::{Display, Formatter, Result};

/// Colours settings.
#[json]
pub struct Colours {}

impl Colours {}

// impl Display for Colours {
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
