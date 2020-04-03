//! Panda rendering engine!

use arc::{
    args,
    file::{Load, Transform},
    report,
    sim::panda::{GridSettings, ShaderSettings},
    util::{banner, exec, init},
};
use attr::form;
use log::info;
use std::path::Path;

#[form]
struct Parameters {
    /// Grid setup information.
    grid_settings: GridSettings,
    /// Shader information.
    shader_settings: ShaderSettings,
    /// Traceable surfaces.
    surfaces: Vec<(i32, Vec<(String, Option<Transform>)>)>,
}

fn main() {
    colog::init();
    banner::title(&exec::name());
    banner::section("initialisation");
    args!(_bin_path: String;
        params_name: String);

    let (in_dir, out_dir) = init::io_dirs(None, None);
    let params_path = &in_dir.join(params_name);

    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");
    report!(params_path.display(), "parameters path");

    banner::section("Loading");
    let _params = load_parameters(params_path);
}

/// Load the parameters file and report the settings.
#[inline]
#[must_use]
fn load_parameters(path: &Path) -> Parameters {
    info!("Loading parameters file");
    let params = Parameters::load(&path);

    report!(&params.grid_settings, "Grid settings");
    report!(&params.shader_settings, "Shader settings");

    params
}
