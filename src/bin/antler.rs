//! Antler rendering engine.

use arc::{
    args, columns,
    file::Load,
    fmt,
    img::{
        save,
        settings::{
            Frame as FrameSettings, Quality as QualitySettings, Scene as SceneSettings,
            Scheme as SchemeSettings, Shader as ShaderSettings,
        },
        Settings,
    },
    sim::{
        render,
        render::{Camera, Frame, Grid, Scene, Scheme},
    },
    util::{exec, init},
    values,
};
use attr::form_load;
use ndarray::Array2;
use palette::LinSrgba;
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
    let params = load_parameters(&in_dir, &params_filename);
    let scene = load_scene(&in_dir, &params);
    let grid = build_grid(&params, &scene);

    fmt::section("Rendering");
    for (name, frame_settings) in params.render.frames() {
        fmt::sub_section(name);
        let frame = load_frame_settings(&in_dir, &frame_settings);
        let img = render::image(&grid, &frame);
        save_frame(&out_dir, name, img);
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

fn load_parameters(in_dir: &Path, params_filename: &str) -> Parameters {
    fmt::sub_section("Parameters");
    let params_path = in_dir.join(params_filename);
    values!(2 * COL_WIDTH, params_path.display());

    Parameters::load(&params_path)
}

/// Load the rendering scene.
fn load_scene(in_dir: &Path, params: &Parameters) -> Scene {
    fmt::sub_section("Scene");
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
    fmt::sub_section("Grid");
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
    let scheme = load_scheme(in_dir, frame);
    let camera = build_camera(frame, &quality);

    Frame::new(frame.aspect_ratio(), quality, shader, scheme, camera)
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

/// Load a colour scheme.
pub fn load_scheme(in_dir: &Path, frame: &FrameSettings) -> Scheme {
    fmt::sub_sub_section("scheme");
    let scheme_path = in_dir.join(format!("schemes/{}.json", frame.scheme()));
    values!(2 * COL_WIDTH, scheme_path.display());
    let scheme = SchemeSettings::load(&scheme_path).build();

    for (group, grad) in scheme.grads() {
        // values!(COL_WIDTH, group, fmt::gradient::to_string(&grad, 64));
        let group_width = COL_WIDTH / 2;
        let term_width = arc::fmt::term_width();
        println!(
            "{:>gw$} : {}",
            format!("[{:^3}]", group),
            fmt::gradient::to_string(&grad, term_width - group_width - 3),
            gw = group_width
        );
    }

    scheme
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

/// Save the frame as a png.
pub fn save_frame(out_dir: &Path, name: &str, img: Array2<LinSrgba>) {
    fmt::sub_sub_section("Saving");

    let img_path = out_dir.join(format!("{}.png", name));
    save::png(&img_path, &img);
    println!("Frame {} saved at: {}", name, img_path.display());
}
