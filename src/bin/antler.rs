//! Antler rendering engine.

use arc::{
    args, columns,
    file::Load,
    fmt,
    rend::{settings::Scene as SceneSettings, Grid, Scene, Settings},
    util::{exec, init},
    values,
};
use attr::form_load;
use std::path::{Path, PathBuf};

/// Column width.
const COL_WIDTH: usize = 48;

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

    fmt::sub_section("Scene");
    let scene = load_scene(&in_dir, &params);

    fmt::sub_section("Grid");
    let grid = Grid::new_root(params.render.grid(), &scene);
    values!(
        COL_WIDTH,
        grid.max_depth(),
        grid.num_cells(),
        grid.num_leaf_cells(),
        grid.num_tri_refs(),
        grid.ave_leaf_tris()
    );

    // /// Build the images.
    // #[inline]
    // #[must_use]
    // pub fn build_images(&self, _in_dir: &Path) -> Vec<RenderImage> {
    //     vec![]
    // }

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

/// Load the rendering scene.
fn load_scene(in_dir: &Path, params: &Parameters) -> Scene {
    let scene_path = in_dir.join(&format!("scenes/{}.json", params.render.scene()));
    values!(2 * COL_WIDTH, scene_path.display());

    let scene = SceneSettings::load(&scene_path).build(&in_dir.join("meshes"));
    values!(
        COL_WIDTH,
        scene.sun_pos(),
        scene.boundary().mins(),
        scene.boundary().maxs(),
        scene.groups().len(),
        scene.total_tris()
    );

    println!("Group triangle breakdown...");
    fmt::values(
        COL_WIDTH,
        scene
            .groups()
            .keys()
            .map(|group| (group, scene.group_tris(*group))),
    );

    scene
}
