//! Zebra ray-tracing adaptive engine.

use arc::{
    args,
    file::{Camera as FileCamera, Load, Save},
    geom::Mesh,
    report,
    sim::render::{Camera, Cell, Group},
    util::{banner, exec, init},
};
use attr::form;
use log::info;
use std::path::Path;

#[form]
struct Parameters {
    camera: FileCamera,
    meshes: Vec<(String, Group)>,
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
    let cam = build_camera(&params.camera);
    let meshes = load_meshes(&in_dir, &params.meshes);
    let grid = build_grid(&meshes);

    banner::section("Rendering");
    let stack = arc::sim::render::run(&cam, &grid);

    banner::section("Saving");
    for (index, img) in stack.iter().enumerate() {
        let path = &out_dir.join(format!("layer_{}.nc", index));
        info!("Saving {}", path.display());
        img.save(path);
    }
}

/// Build the camera.
fn build_camera(camera: &FileCamera) -> Camera {
    info!("Building camera");

    let cam = camera.build();

    report!(cam.num_pix() as f64 / 100000.0, "Total pixels", "Million");
    report!(cam.fov().0.to_degrees(), "Horizontal fov", "Degrees");
    report!(cam.fov().1.to_degrees(), "Vertical fov", "Degrees");

    cam
}

/// Load in the base meshes.
fn load_meshes(in_dir: &Path, names: &Vec<(String, Group)>) -> Vec<(Mesh, Group)> {
    let mut meshes = vec![];

    info!("Loading meshes");
    for (name, group) in names {
        let path = &in_dir.join(format!("entities/{}.obj", name));
        info!("Loading: {}", path.display());
        meshes.push((Mesh::load(path), *group));
    }

    meshes
}

/// Build the grid.
fn build_grid(meshes: &Vec<(Mesh, Group)>) -> Cell {
    info!("Building grid");
    let grid = arc::sim::render::Cell::new_root(6, 4, meshes);

    report!(grid.num_leaves());
    report!(grid.num_empty());
    report!(grid.num_cells());
    report!(grid.num_branches());
    report!(grid.num_tri_refs());
    report!(grid.ave_leaf_tris());

    grid
}
