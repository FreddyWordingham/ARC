//! Antler rendering engine.

use arc::{
    args, columns,
    file::Load,
    fmt,
    rend::{
        settings::{
            Image as ImageSettings, Palette as PaletteSettings, Quality as QualitySettings,
            Scene as SceneSettings, Shader as ShaderSettings,
        },
        Grid, Scene, Settings,
    },
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
    let _grid = build_grid(&params, &scene);

    fmt::section("Rendering");
    for (name, image) in params.render.images() {
        fmt::sub_section(name);
        let (_quality, _shader, _palette) = load_image_settings(&in_dir, &image);
    }

    fmt::section("Finished");
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

/// Build the gridding scheme.
fn build_grid<'a>(params: &Parameters, scene: &'a Scene) -> Grid<'a> {
    let grid = Grid::new_root(params.render.grid(), &scene);

    values!(
        COL_WIDTH,
        grid.max_depth(),
        grid.num_cells(),
        grid.num_leaf_cells(),
        grid.num_tri_refs(),
        grid.ave_leaf_tris()
    );

    grid
}

/// Load the image settings.
pub fn load_image_settings(
    in_dir: &Path,
    image: &ImageSettings,
) -> (QualitySettings, ShaderSettings, PaletteSettings) {
    fmt::sub_sub_section("quality");
    let quality_path = in_dir.join(format!("quality/{}.json", image.quality()));
    let quality = QualitySettings::load(&quality_path);

    fmt::sub_sub_section("shader");
    let shader_path = in_dir.join(format!("shaders/{}.json", image.shader()));
    let shader = ShaderSettings::load(&shader_path);

    fmt::sub_sub_section("palette");
    let palette_path = in_dir.join(format!("palettes/{}.json", image.palette()));
    let palette = PaletteSettings::load(&palette_path);

    (quality, shader, palette)
}
