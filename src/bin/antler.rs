//! Antler rendering engine.

use arc::{
    args,
    file::Load,
    rend::settings::{Grid, Scene},
    report,
    util::{banner, exec, init},
};
use attr::form;
use log::info;
use std::path::{Path, PathBuf};

/// Input parameters.
#[form]
struct Parameters {
    /// Grit settings.
    grid: Grid,
    /// Scene settings.
    scene: String,
}

fn main() {
    colog::init();
    banner::title(&exec::name());

    banner::section("Initialisation");
    let (in_dir, _out_dir, params_filename) = init_dirs();

    banner::section("Input");
    let (params, scene) = load(&in_dir, &params_filename);
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

/// Load the parameters file and report the settings.
fn load(in_dir: &Path, params_filename: &str) -> (Parameters, arc::rend::settings::Scene) {
    let params_path = in_dir.join(params_filename);
    info!("Loading parameters file: {}", params_path.display());
    let params = Parameters::load(&params_path);

    let scene_path = in_dir.join(format!("{}.json", params.scene));
    info!("Loading scene file: {}", scene_path.display());
    let scene = arc::rend::settings::Scene::load(&scene_path);

    // report!(&params.grid, "Grid settings");

    (params, scene)
}
