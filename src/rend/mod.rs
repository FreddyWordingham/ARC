//! Rendering module.

pub mod aspect_ratio;
pub mod grid;
pub mod group;
pub mod hit;
pub mod image;
pub mod lighting;
pub mod save;
pub mod scan;
pub mod scene;
pub mod settings;

pub use self::{
    aspect_ratio::*, grid::*, group::*, hit::*, image::*, lighting::*, scan::*, scene::*,
    settings::Settings,
};

use ::palette::{Gradient, LinSrgba};
use ansi_rgb::Background;
use rgb::RGB8;

/// Create a string of a gradients colour.
#[inline]
#[must_use]
pub fn print_colour(grad: &Gradient<LinSrgba>, len: usize) -> String {
    let mut scale = String::new();

    for i in 0..len {
        let x = i as f64 / (len - 1) as f64;

        let col = grad.get(x as f32);

        let (r, g, b) = (
            (col.red * 255.0) as u8,
            (col.green * 255.0) as u8,
            (col.blue * 255.0) as u8,
        );
        scale.push_str(&format!("{}", " ".bg(RGB8::new(r, g, b,))));
    }

    scale
}
