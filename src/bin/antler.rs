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
    // let (_scene, _shader, _palette) = input(&in_dir, &params_filename);
    input(&in_dir, &params_filename);

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

/// Load the input parameters file and report the settings.
fn input(in_dir: &Path, params_filename: &str) -> () {
    let params_path = in_dir.join(params_filename);
    fmt::sub_section("Parameters");
    values!(2 * COL_WIDTH, params_path.display());
    let params = Parameters::load(&params_path);

    fmt::sub_section("Grid");
    let grid = params.render.grid();
    values!(COL_WIDTH, grid.tar_tris(), grid.max_depth(), grid.padding());

    // banner::sub_section("Images");
    // report!(params.images.len(), "Total images");
    // let mut imgs = Vec::with_capacity(params.images.len());
    // for (name, img) in &params.images {
    //     info!("Image {}:\n{}", name, img);

    //     let cam = Camera::new(*img.cam_pos(), *img.tar_pos());
    //     info!("Camera:\n{}", cam);

    //     let qual_path = in_dir.join(format!("{}.json", img.quality()));
    //     report!(qual_path.display(), "Loading quality file");
    //     let qual = Quality::load(&qual_path);
    //     info!("Quality settings:\n{}", qual);

    //     imgs.push((cam, qual));
    // }

    // banner::sub_section("Scene");
    // let scene_path = in_dir.join(format!("{}.json", params.scene));
    // report!(scene_path.display(), "Loading scene file");
    // let scene = Scene::load(&scene_path);
    // info!("Scene settings:\n{}", scene);

    // banner::sub_section("Shader");
    // let shader_path = in_dir.join(format!("{}.json", params.shader));
    // report!(shader_path.display(), "Loading shader file");
    // let shader = Shader::load(&shader_path);
    // info!("Shader settings:\n{}", shader);

    // banner::sub_section("Palette");
    // let palette_path = in_dir.join(format!("{}.json", params.palette));
    // report!(palette_path.display(), "Loading palette file");
    // let palette = Palette::load(&palette_path);
    // info!("Palette settings:\n{}", palette);

    // (scene, shader, palette)
    ()
}
