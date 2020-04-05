//! Antler rendering engine.

use arc::{
    args,
    file::Load,
    rend::settings::Grid,
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
}

fn main() {
    colog::init();
    banner::title(&exec::name());

    banner::section("Initialisation");
    let (_in_dir, _out_dir, params_path) = init_dirs();

    banner::section("Input");
    let _params = load_parameters(&params_path);
}

/// Get the directories.
fn init_dirs() -> (PathBuf, PathBuf, PathBuf) {
    args!(_bin_path: String;
        params_name: String
    );

    let (in_dir, out_dir) = init::io_dirs(None, None);
    let params_path = in_dir.join(params_name);

    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");
    report!(params_path.display(), "parameters path");

    (in_dir, out_dir, params_path)
}

/// Load the parameters file and report the settings.
fn load_parameters(path: &Path) -> Parameters {
    info!("Loading parameters...");
    let params = Parameters::load(&path);

    report!(&params.grid, "Grid settings");
    // report!(&params.shader_settings, "Shader settings");

    // info!("Cameras:");
    // for (name, cam) in &params.cameras {
    //     report!(cam, name);
    // }

    params
}
