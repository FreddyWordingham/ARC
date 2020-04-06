//! Antler rendering engine.

use arc::{
    args, columns,
    file::Load,
    fmt,
    rend::{
        settings::{Grid, Image, Palette, Quality, Scene, Shader},
        Camera,
    },
    report,
    util::{exec, init},
    values,
};
use attr::form;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

/// Input parameters.
#[form]
struct Parameters {
    /// Grit settings.
    grid: Grid,
    /// Scene settings.
    scene: String,
    /// Colour settings.
    palette: String,
    /// Shader settings.
    shader: String,
    /// Images.
    images: BTreeMap<String, Image>,
}

fn main() {
    fmt::title(&exec::name());

    fmt::section("Initialisation");
    let (in_dir, _out_dir, params_filename) = init_dirs();

    values!(
        64,
        "these are the numbers",
        arc::fmt::term_width(),
        "one",
        2,
        4 - 1,
        -9 + 13,
        "five",
        "one",
        2,
        4 - 1,
        -9 + 13,
        "five",
        "one",
        2,
        4 - 1,
        -9 + 13,
        "five",
        "one",
        2,
        4 - 1,
        -9 + 13,
        "five"
    );

    fmt::section("Input");
    // let (_scene, _shader, _palette) = input(&in_dir, &params_filename);

    // banner::end("Simulation complete");
}

/// Get the directories.
fn init_dirs() -> (PathBuf, PathBuf, String) {
    fmt::sub_section("Command line arguments");
    args!(_bin_path: String;
        params_filename: String
    );
    // values!(20, params_filename);

    fmt::sub_section("Directories");
    let (in_dir, out_dir) = init::io_dirs(None, None);
    // values!(20, in_dir.display(), out_dir.display());

    (in_dir, out_dir, params_filename)
}

// /// Load the input parameters file and report the settings.
// fn input(in_dir: &Path, params_filename: &str) -> (Scene, Shader, Palette) {
//     let params_path = in_dir.join(params_filename);
//     banner::sub_section("Parameters");
//     report!(params_path.display(), "Loading parameters file");
//     let params = Parameters::load(&params_path);

//     banner::sub_section("Grid");
//     let grid = params.grid;
//     info!("Grid settings:\n{}", grid);

//     banner::sub_section("Images");
//     report!(params.images.len(), "Total images");
//     let mut imgs = Vec::with_capacity(params.images.len());
//     for (name, img) in &params.images {
//         info!("Image {}:\n{}", name, img);

//         let cam = Camera::new(*img.cam_pos(), *img.tar_pos());
//         info!("Camera:\n{}", cam);

//         let qual_path = in_dir.join(format!("{}.json", img.quality()));
//         report!(qual_path.display(), "Loading quality file");
//         let qual = Quality::load(&qual_path);
//         info!("Quality settings:\n{}", qual);

//         imgs.push((cam, qual));
//     }

//     banner::sub_section("Scene");
//     let scene_path = in_dir.join(format!("{}.json", params.scene));
//     report!(scene_path.display(), "Loading scene file");
//     let scene = Scene::load(&scene_path);
//     info!("Scene settings:\n{}", scene);

//     banner::sub_section("Shader");
//     let shader_path = in_dir.join(format!("{}.json", params.shader));
//     report!(shader_path.display(), "Loading shader file");
//     let shader = Shader::load(&shader_path);
//     info!("Shader settings:\n{}", shader);

//     banner::sub_section("Palette");
//     let palette_path = in_dir.join(format!("{}.json", params.palette));
//     report!(palette_path.display(), "Loading palette file");
//     let palette = Palette::load(&palette_path);
//     info!("Palette settings:\n{}", palette);

//     (scene, shader, palette)
// }
