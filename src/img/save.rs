//! Save functions.

use ndarray::{Array2, ShapeBuilder};
use palette::{LinSrgba, Pixel, Srgba};
use png::{BitDepth, ColorType, Encoder};
use slice_of_array::prelude::*;
use std::{fs::File, io::BufWriter, path::Path};

/// Save an array as a png image.
#[inline]
pub fn png(path: &Path, img: Array2<LinSrgba>) {
    let res = (
        *img.shape().get(0).expect("Missing shape."),
        *img.shape().get(1).expect("Missing shape."),
    );
    let mut data: Array2<[u8; 4]> = Array2::from_elem((res.0, res.1).f(), [0; 4]);
    for xi in 0..res.0 {
        for yi in 0..res.1 {
            let col = *img.get((xi, yi)).expect("Invalid colour index.");
            *data
                .get_mut((xi, res.1 - yi - 1))
                .expect("Invalid png index.") = Srgba::from_linear(col).into_format().into_raw();
        }
    }

    let file = File::create(path).expect("Could not create png file.");
    let w = BufWriter::new(file);

    let mut encoder = Encoder::new(w, res.0 as u32, res.1 as u32);
    encoder.set_color(ColorType::RGBA);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder
        .write_header()
        .expect("Could not build image writer.");

    writer
        .write_image_data(data.into_raw_vec().flat())
        .expect("Failed to save png.");
}
