//! Antler rendering engine.

use arc::{
    args,
    file::Load,
    rend::settings::{Grid, Image, Palette, Scene, Shader},
    report,
    util::{banner, exec, init},
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
    colog::init();
    banner::title(&exec::name());

    banner::section("Initialisation");
    let (in_dir, _out_dir, params_filename) = init_dirs();

    banner::section("Input");
    let (_params, _scene, _shader, _palette) = input(&in_dir, &params_filename);
}

/// Get the directories.
fn init_dirs() -> (PathBuf, PathBuf, String) {
    args!(_bin_path: String;
        params_filename: String
    );
    report!(params_filename, "parameters filename");

    let (in_dir, out_dir) = init::io_dirs(None, None);
    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");

    (in_dir, out_dir, params_filename)
}

/// Load the input parameters file and report the settings.
fn input(in_dir: &Path, params_filename: &str) -> (Parameters, Scene, Shader, Palette) {
    let params_path = in_dir.join(params_filename);
    report!(params_path.display(), "Loading parameters file");
    let params = Parameters::load(&params_path);
    report!(params.grid, "Grid settings");
    report!(params.images.len(), "Total images");
    for (name, img) in &params.images {
        report!(img, name);
        // report!(img, format!(""));
    }

    let scene_path = in_dir.join(format!("{}.json", params.scene));
    report!(scene_path.display(), "Loading scene file");
    let scene = Scene::load(&scene_path);
    report!(scene, "Scene settings");

    let shader_path = in_dir.join(format!("{}.json", params.shader));
    report!(shader_path.display(), "Loading shader file");
    let shader = Shader::load(&shader_path);
    report!(shader, "Shader settings");

    let palette_path = in_dir.join(format!("{}.json", params.palette));
    report!(palette_path.display(), "Loading palette file");
    let palette = Palette::load(&palette_path);

    (params, scene, shader, palette)
}
