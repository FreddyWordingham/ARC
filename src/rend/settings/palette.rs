//! Palette colours input settings implementation.

use crate::{access, rend::Group};
use attr::json;
use std::fmt::{Display, Formatter, Result};

/// Palette settings.
#[json]
pub struct Palette {
    /// Group colours.
    cols: Vec<(Group, Vec<[f64; 4]>)>,
}

impl Palette {
    access!(cols, Vec<(Group, Vec<[f64; 4]>)>);
}

impl Display for Palette {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        for (_group, _cs) in &self.cols {
            // writeln!(
            //     fmt,
            //     "{:>30} : {}",
            //     format!("[{:^3}]", group),
            //     cols(
            //         cs.iter()
            //             .map(|c| {
            //             //     let r = c.get(0).expect("Missing red component.");
            //             //     let g = c.get(1).expect("Missing green component.");
            //             //     let b = c.get(2).expect("Missing blue component.");
            //             //     let a = c.get(3).expect("Missing alpha component.");

            //                     format!("[{:.2}, {:.2}, {:.2}, {:.2}]> ", r, g, b, a)
            //                 }
            //             //     format!("[{:.2}, {:.2}, {:.2}, {:.2}]> ", r, g, b, a).bg(RGB8::new(10, 100, 20))}
            //             )
            //         7, 24
            //     )
            // )?;
            writeln!(fmt, "TODO TODO TODO TODO TODO TODO TODO ")?;
        }

        Ok(())
    }
}
