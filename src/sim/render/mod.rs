//! Rendering simulation sub-module.

pub mod camera;
pub mod frame;
pub mod grid;
pub mod group;
pub mod hit;
pub mod lighting;
pub mod pipe;
pub mod scan;
pub mod scene;
pub mod scheme;

pub use self::{camera::*, frame::*, grid::*, group::*, hit::*, scan::*, scene::*, scheme::*};

use crate::{geom::Ray, img::settings::Shader};
use nalgebra::Point3;
use palette::LinSrgba;
use rand::rngs::ThreadRng;
pub type PipeFunc = fn(&Point3<f64>, &Grid, &Shader, &Scheme, Ray, &mut ThreadRng) -> LinSrgba;
