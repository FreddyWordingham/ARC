//! Antler rendering engine.

use arc::{
    args, columns,
    file::Load,
    fmt,
    rend::Settings,
    util::{exec, init},
    values,
};
use attr::form_load;
use std::path::PathBuf;

/// Column width.
const COL_WIDTH: usize = 40;

/// Input parameters.
#[form_load]
struct Parameters {
    /// Rendering settings.
    render: Settings,
}

fn main() {
    fmt::title(&exec::name());

    fmt::section("Initialisation");
    let (in_dir, _out_dir, params_filename) = init_dirs();

    fmt::section("Loading");
    fmt::sub_section("Parameters");
    let params_path = in_dir.join(params_filename);
    values!(2 * COL_WIDTH, params_path.display());
    let params = Parameters::load(&params_path);
    // fmt::sub_section("Scene");
    // let scene = params.render.load_scene(&in_dir);
    // fmt::sub_section("Grid");
    // let _grid = params.render.build_grid(&scene);
    // fmt::sub_section("Images");
    // let _images = params.render.build_images(&in_dir);
    let (_scene, _grid, _images) = params.render.build(&in_dir);

    // banner::end("Simulation complete");
}

/// Get the directories.
fn init_dirs() -> (PathBuf, PathBuf, String) {
    fmt::sub_section("Command line arguments");
    args!(_bin_path: String;
        params_filename: String
    );
    values!(COL_WIDTH, params_filename);

    fmt::sub_section("Directories");
    let (in_dir, out_dir) = init::io_dirs(None, None);
    values!(2 * COL_WIDTH, in_dir.display(), out_dir.display());

    (in_dir, out_dir, params_filename)
}
