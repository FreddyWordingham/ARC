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
    sim::render::painter,
    sim::render::{Camera, Frame, Grid, Scene, Scheme},
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
        let img = frame.image(painter::lumin::paint, &grid);
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
    {
        values!(COL_WIDTH, params_filename);
    }

    fmt::sub_section("Directories");
    let (in_dir, out_dir) = init::io_dirs(None, None);
    {
        let input_directory = in_dir.display();
        let output_directory = out_dir.display();
        values!(2 * COL_WIDTH, input_directory, output_directory);
    }

    (in_dir, out_dir, params_filename)
}

/// Load the parameters structure.
fn load_parameters(in_dir: &Path, params_filename: &str) -> Parameters {
    fmt::sub_section("Parameters");
    let params_path = in_dir.join(params_filename);
    {
        let parameters_path = params_path.display();
        values!(2 * COL_WIDTH, parameters_path);
    }

    Parameters::load(&params_path)
}

/// Load the rendering scene.
fn load_scene(in_dir: &Path, params: &Parameters) -> Scene {
    fmt::sub_section("Scene");
    let scene_path = in_dir.join(&format!("scenes/{}.json", params.render.scene()));
    {
        let scene_path = scene_path.display();
        values!(2 * COL_WIDTH, scene_path);
    }

    let scene = SceneSettings::load(&scene_path).build(&in_dir.join("meshes"));
    {
        let scene_minimum = scene.boundary().mins();
        let scene_maximum = scene.boundary().maxs();
        let number_of_groups = scene.groups().len();
        let total_triangles = scene.total_tris();
        values!(
            COL_WIDTH,
            scene_minimum,
            scene_maximum,
            number_of_groups,
            total_triangles
        );
    }
    println!("Group triangle breakdown...");
    {
        fmt::values(
            COL_WIDTH,
            scene
                .groups()
                .keys()
                .map(|group| (group, scene.group_tris(*group))),
        );
    }

    scene
}

/// Build the gridding scheme.
fn build_grid<'a>(params: &Parameters, scene: &'a Scene) -> Grid<'a> {
    fmt::sub_section("Grid");
    println!("Building...");
    let grid = Grid::new_root(params.render.grid(), &scene);
    {
        let max_depth = grid.max_depth();
        let total_cells = grid.num_cells();
        let total_leaves = grid.num_leaf_cells();
        let total_references = grid.num_tri_refs();
        let average_tris_per_leaf = grid.ave_leaf_tris();
        values!(
            COL_WIDTH,
            max_depth,
            total_cells,
            total_leaves,
            total_references,
            average_tris_per_leaf
        );
    }

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
    {
        let quality_path = quality_path.display();
        values!(2 * COL_WIDTH, quality_path);
    }

    let quality = QualitySettings::load(&quality_path);
    {
        let target_pixels = quality.target_pixels();
        let samples_per_pixel = quality.samples_per_pixel();
        let super_samples = if let Some(power) = quality.super_samples() {
            format!("{}", power)
        } else {
            "OFF".to_owned()
        };
        let dof_samples = if let Some(samples) = quality.dof_samples() {
            format!("{}", samples)
        } else {
            "OFF".to_owned()
        };
        let shadow_samples = quality.shadow_samples();
        let horizontal_sections = quality.section_splits().0;
        let vertical_sections = quality.section_splits().1;
        values!(
            COL_WIDTH,
            target_pixels,
            samples_per_pixel,
            super_samples,
            dof_samples,
            shadow_samples,
            horizontal_sections,
            vertical_sections
        );
    }

    quality
}

/// Load shader settings.
pub fn load_shader(in_dir: &Path, frame: &FrameSettings) -> ShaderSettings {
    fmt::sub_sub_section("shader");
    let shader_path = in_dir.join(format!("shaders/{}.json", frame.shader()));
    {
        let shader_path = shader_path.display();
        values!(2 * COL_WIDTH, shader_path);
    }

    let shader = ShaderSettings::load(&shader_path);
    {
        let ambient_lighting = shader.light_weights().ambient();
        let diffuse_lighting = shader.light_weights().diffuse();
        let specular_lighting = shader.light_weights().specular();
        let direct_shadowing = shader.shadow_weights().direct();
        let local_shadowing = shader.shadow_weights().local();
        let ambient_shadowing = shader.shadow_weights().ambient();
        let sun_pos = shader.sun_pos();
        values!(
            COL_WIDTH,
            ambient_lighting,
            diffuse_lighting,
            specular_lighting,
            direct_shadowing,
            local_shadowing,
            ambient_shadowing,
            sun_pos
        );
    }

    shader
}

/// Load a colour scheme.
pub fn load_scheme(in_dir: &Path, frame: &FrameSettings) -> Scheme {
    fmt::sub_sub_section("scheme");
    let scheme_path = in_dir.join(format!("schemes/{}.json", frame.scheme()));
    {
        let scheme_path = scheme_path.display();
        values!(2 * COL_WIDTH, scheme_path);
    }

    let scheme = SchemeSettings::load(&scheme_path).build();
    {
        let group_width = COL_WIDTH / 2;
        let term_width = arc::fmt::term_width();
        println!(
            "{:>gw$} : {}",
            "[backup]",
            fmt::gradient::to_string(scheme.backup(), term_width - group_width - 3),
            gw = group_width
        );
        for (group, grad) in scheme.grads() {
            println!(
                "{:>gw$} : {}",
                format!("[{:^3}]", group),
                fmt::gradient::to_string(&grad, term_width - group_width - 3),
                gw = group_width
            );
        }
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
        quality.section_splits(),
        quality.target_pixels(),
        quality.super_samples(),
    );
    {
        let horizontal_res = camera.res().0;
        let vertical_res = camera.res().1;
        let total_pixels = camera.total_pixels();
        let total_samples = total_pixels * quality.samples_per_pixel();
        let horizontal_fov = camera.fov().0.to_degrees();
        let vertical_fov = camera.fov().1.to_degrees();
        let camera_position = camera.pos();
        let camera_target = camera.tar();
        values!(
            COL_WIDTH,
            horizontal_res,
            vertical_res,
            total_pixels,
            total_samples,
            horizontal_fov,
            vertical_fov,
            camera_position,
            camera_target
        );
    }

    camera
}

/// Save the frame as a png.
pub fn save_frame(out_dir: &Path, name: &str, img: Array2<LinSrgba>) {
    fmt::sub_sub_section("Saving");

    let img_path = out_dir.join(format!("{}.png", name));
    save::png(&img_path, &img);
    println!("Frame {} saved at: {}", name, img_path.display());
}
