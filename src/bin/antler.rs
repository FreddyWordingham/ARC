//! Antler rendering engine.

use arc::{
    args, columns,
    file::Load,
    fmt,
    rend::{Grid, Image, Scene, Settings},
    util::{exec, init},
    values,
};
use attr::form_load;
use std::path::{Path, PathBuf};

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

    fmt::section("Input");
    // let (_scene, _images) = input(&in_dir, &params_filename);
    // input(&in_dir, &params_filename);

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

// /// Load the scene and images.
// fn input(in_dir: &Path, params_filename: &str) -> (Scene, Grid<'a>, Vec<Image>) {
//     fmt::sub_section("Parameters");
//     let params_path = in_dir.join(params_filename);
//     values!(2 * COL_WIDTH, params_path.display());
//     let params = Parameters::load(&params_path);

//     params.render.build(in_dir)
// }
