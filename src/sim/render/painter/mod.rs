//! Painter function alias.

pub mod arctic;
pub mod lumin;

use crate::{
    geom::Ray,
    img::settings::Shader,
    sim::render::{Grid, Scheme},
};
use nalgebra::Point3;
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Fragment painter function.
pub type Painter = fn(&Point3<f64>, &Grid, &Shader, &Scheme, Ray, &mut ThreadRng, f64) -> LinSrgba;
