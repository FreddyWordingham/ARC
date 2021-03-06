//! Zebra ray-tracing adaptive engine.

use arc::{
    args,
    file::{Camera as FileCamera, Load, Save},
    geom::Mesh,
    report,
    sim::render::{Camera, Cell, Group, Settings},
    util::{banner, exec, init},
};
use attr::form;
use log::info;
use std::path::Path;

#[form]
struct Parameters {
    max_depth: usize,
    tar_tris: usize,
    camera: FileCamera,
    meshes: Vec<(Group, Vec<String>)>,
    sett: Settings,
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
    info!("Loading parameters file");
    let params = Parameters::load(&params_path);
    let max_depth = params.max_depth;
    let tar_tris = params.tar_tris;
    let cam = build_camera(&params.camera);
    let meshes = load_meshes(&in_dir, &params.meshes);
    let grid = build_grid(max_depth, tar_tris, &meshes);
    let sett = params.sett;

    banner::section("Rendering");
    let mut stack = arc::sim::render::run(&cam, &grid, &sett);

    banner::section("Saving");
    let path = &out_dir.join("scene.nc");
    info!("Saving {}", path.display());
    stack.pop().expect("Missing scene image.").save(path);

    let path = &out_dir.join("sky.nc");
    info!("Saving {}", path.display());
    stack.pop().expect("Missing sky image.").save(path);

    let path = &out_dir.join("lights.nc");
    info!("Saving {}", path.display());
    stack.pop().expect("Missing lights image.").save(path);

    for (index, img) in stack.iter().enumerate() {
        let path = &out_dir.join(format!("layer_{}.nc", index));
        info!("Saving {}", path.display());
        img.save(path);
    }
}

/// Build the camera.
fn build_camera(camera: &FileCamera) -> Camera {
    info!("Building camera");

    let cam = camera.build_zebra_cam();

    report!(cam.num_pix() as f64 / 100000.0, "Total pixels", "Million");
    report!(cam.ss_power().pow(2), "Samples per pixel");
    report!(cam.fov().0.to_degrees(), "Horizontal fov", "Degrees");
    report!(cam.fov().1.to_degrees(), "Vertical fov", "Degrees");

    cam
}

/// Load in the base meshes.
fn load_meshes(in_dir: &Path, names: &Vec<(Group, Vec<String>)>) -> Vec<(Mesh, Group)> {
    let mut meshes = vec![];

    info!("Loading meshes");
    for (group, list) in names {
        for name in list {
            let path = &in_dir.join(format!("entities/{}.obj", name));
            info!("Loading: {}", path.display());
            meshes.push((Mesh::load(path), *group));
        }
    }

    meshes
}

/// Build the grid.
fn build_grid(max_depth: usize, tar_tris: usize, meshes: &Vec<(Mesh, Group)>) -> Cell {
    info!("Building grid");
    report!(max_depth);
    report!(tar_tris);
    let grid = arc::sim::render::Cell::new_root(max_depth, tar_tris, meshes);

    report!(grid.num_leaves());
    report!(grid.num_empty());
    report!(grid.num_cells());
    report!(grid.num_branches());
    report!(grid.num_tri_refs());
    report!(grid.ave_leaf_tris());

    grid
}
