//! Save functions.

use ndarray::Array2;
use palette::{LinSrgba, Pixel, Srgba};
use png::{BitDepth, ColorType, Encoder};
use slice_of_array::prelude::*;
use std::{fs::File, io::BufWriter, path::Path};

/// Save an array as a png image.
#[inline]
pub fn png(path: &Path, img: Array2<LinSrgba>) {
    let mut data = Array2::from_elem(
        (
            *img.shape().get(1).expect("Missing dimension."),
            *img.shape().get(0).expect("Missing dimension."),
        ),
        Srgba::default().into_linear(),
    );
    for xi in 0..*img.shape().get(1).expect("Missing dimension.") {
        for yi in 0..*img.shape().get(0).expect("Missing dimension.") {
            data[[
                *img.shape().get(1).expect("Missing dimension.") - xi - 1,
                *img.shape().get(0).expect("Missing dimension.") - yi - 1,
            ]] = img[[yi, xi]];
        }
    }
    let data = data.t();

    let file = File::create(path).expect("Could not create png file.");
    let ref mut w = BufWriter::new(file);
    let mut encoder = Encoder::new(
        w,
        *data.shape().get(0).expect("Missing dimension.") as u32,
        *data.shape().get(1).expect("Missing dimension.") as u32,
    );
    encoder.set_color(ColorType::RGBA);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder
        .write_header()
        .expect("Could not build image writer.");

    let data: Vec<[u8; 4]> = data
        .mapv(|col| Srgba::from_linear(col).into_format().into_raw())
        .into_raw_vec();
    writer
        .write_image_data(data.flat())
        .expect("Failed to save png.");
}
