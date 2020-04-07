//! Antler rendering engine.

use arc::{
    args, columns,
    file::Load,
    fmt,
    img::{
        save,
        settings::{
            Frame as FrameSettings, Palette as PaletteSettings, Quality as QualitySettings,
            Scene as SceneSettings, Shader as ShaderSettings,
        },
        Settings,
    },
    sim::render::{Camera, Frame, Grid, Group, Scene},
    util::{exec, init},
    values,
};
use attr::form_load;
use palette::{Gradient, LinSrgba};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Column width.
const COL_WIDTH: usize = 64;

/// Input parameters.
#[form_load]
struct Parameters {
    /// Rendering settings.
    render: Settings,
}

fn main() {
    fmt::title(&exec::name());

    fmt::section("Initialisation");
    let (in_dir, out_dir, params_filename) = init_dirs();

    fmt::section("Loading");
    fmt::sub_section("Parameters");
    let params_path = in_dir.join(params_filename);
    values!(2 * COL_WIDTH, params_path.display());
    let params = Parameters::load(&params_path);
    fmt::sub_section("Scene");
    let scene = load_scene(&in_dir, &params);
    fmt::sub_section("Grid");
    let grid = build_grid(&params, &scene);

    fmt::section("Rendering");
    for (name, frame_settings) in params.render.frames() {
        fmt::sub_section(name);
        let frame = load_frame_settings(&in_dir, &frame_settings);
        fmt::sub_sub_section("Rendering");
        let img = arc::sim::render::image(&grid, &frame);
        fmt::sub_sub_section("Saving");
        let img_path = out_dir.join(format!("{}.png", name));
        save::png(&img_path, &img);
        println!("Frame {} saved at: {}", name, img_path.display());
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

/// Load the frame settings.
pub fn load_frame_settings(in_dir: &Path, frame: &FrameSettings) -> Frame {
    let quality = load_quality(in_dir, frame);
    let shader = load_shader(in_dir, frame);
    let palette = load_palette(in_dir, frame);
    let camera = build_camera(frame, &quality);

    Frame::new(frame.aspect_ratio(), quality, shader, palette, camera)
}

/// Load quality settings.
pub fn load_quality(in_dir: &Path, frame: &FrameSettings) -> QualitySettings {
    fmt::sub_sub_section("quality");
    let quality_path = in_dir.join(format!("quality/{}.json", frame.quality()));
    values!(2 * COL_WIDTH, quality_path.display());
    let quality = QualitySettings::load(&quality_path);

    values!(
        COL_WIDTH,
        quality.total_pixels(),
        quality.super_samples(),
        quality.dof_samples(),
        quality.shadow_samples(),
        quality.samples_per_pixel(),
        quality.total_samples()
    );

    quality
}

/// Load shader settings.
pub fn load_shader(in_dir: &Path, frame: &FrameSettings) -> ShaderSettings {
    fmt::sub_sub_section("shader");
    let shader_path = in_dir.join(format!("shaders/{}.json", frame.shader()));
    values!(2 * COL_WIDTH, shader_path.display());
    let shader = ShaderSettings::load(&shader_path);

    let light_weights = shader.light_weights();
    let shadow_weights = shader.shadow_weights();
    values!(
        COL_WIDTH,
        light_weights.ambient(),
        light_weights.diffuse(),
        light_weights.specular(),
        shadow_weights.direct(),
        shadow_weights.local(),
        shadow_weights.ambient()
    );

    shader
}

/// Load a colour palette.
pub fn load_palette(in_dir: &Path, frame: &FrameSettings) -> HashMap<Group, Gradient<LinSrgba>> {
    fmt::sub_sub_section("palette");
    let palette_path = in_dir.join(format!("palettes/{}.json", frame.palette()));
    values!(2 * COL_WIDTH, palette_path.display());
    let palette = PaletteSettings::load(&palette_path).build();

    for (group, grad) in &palette {
        values!(COL_WIDTH, group, fmt::gradient::to_string(&grad, 64));
    }

    palette
}

/// Build a camera.
pub fn build_camera(frame: &FrameSettings, quality: &QualitySettings) -> Camera {
    fmt::sub_sub_section("camera");
    let camera = Camera::new(
        *frame.cam_pos(),
        *frame.tar_pos(),
        frame.fov().to_radians(),
        &frame.aspect_ratio(),
        quality.total_pixels(),
        quality.super_samples(),
    );

    values!(
        COL_WIDTH,
        camera.total_pixels(),
        camera.res().0,
        camera.res().1,
        camera.pos(),
        camera.tar()
    );

    camera
}
