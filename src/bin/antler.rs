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
        Camera, Grid, Image, Scene, Settings,
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
    for (name, image_settings) in params.render.images() {
        fmt::sub_section(name);
        let _image = load_image_settings(&in_dir, &image_settings);
        println!("Rendering...");
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
    println!("Building...");
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
pub fn load_image_settings(in_dir: &Path, image: &ImageSettings) -> Image {
    fmt::sub_sub_section("loading");
    let quality_path = in_dir.join(format!("quality/{}.json", image.quality()));
    values!(2 * COL_WIDTH, quality_path.display());
    let quality = QualitySettings::load(&quality_path);

    let shader_path = in_dir.join(format!("shaders/{}.json", image.shader()));
    values!(2 * COL_WIDTH, shader_path.display());
    let shader = ShaderSettings::load(&shader_path);

    let palette_path = in_dir.join(format!("palettes/{}.json", image.palette()));
    values!(2 * COL_WIDTH, palette_path.display());
    let palette = PaletteSettings::load(&palette_path).build();

    let camera = Camera::new(
        *image.cam_pos(),
        *image.tar_pos(),
        image.fov().to_radians(),
        image.aspect_ratio(),
        quality.total_pixels(),
        quality.super_samples(),
    );

    let image = Image::new(image.aspect_ratio(), quality, shader, palette, camera);
    values!(COL_WIDTH, image.aspect_ratio());

    fmt::sub_sub_section("quality");
    let qual = image.quality();
    values!(
        COL_WIDTH,
        qual.total_pixels(),
        qual.super_samples(),
        qual.dof_samples(),
        qual.shadow_samples(),
        qual.samples_per_pixel(),
        qual.total_samples()
    );

    fmt::sub_sub_section("shader");
    let light_weights = image.shader().light_weights();
    let shadow_weights = image.shader().shadow_weights();
    values!(
        COL_WIDTH,
        light_weights.ambient(),
        light_weights.diffuse(),
        light_weights.specular(),
        shadow_weights.direct(),
        shadow_weights.local(),
        shadow_weights.ambient()
    );

    fmt::sub_sub_section("palette");

    fmt::sub_sub_section("camera");
    let cam = image.camera();
    values!(
        COL_WIDTH,
        cam.total_pixels(),
        cam.res().0,
        cam.res().1,
        cam.pos(),
        cam.tar()
    );

    image
}
