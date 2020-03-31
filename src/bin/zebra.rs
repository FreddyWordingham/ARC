//! Zebra ray-tracing adaptive engine.

use arc::{
    args,
    file::{Camera as FileCamera, Load},
    geom::Mesh,
    report,
    sim::render::{Camera, Group},
    util::{banner, exec, init},
};
use attr::form;
use log::info;
use std::path::Path;

#[form]
struct Parameters {
    camera: FileCamera,
    entities: Vec<(String, Group)>,
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
    let _cam = build_camera(&params.camera);
    let _meshes = load_meshes(&in_dir, &params.entities);
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
fn load_meshes(in_dir: &Path, ents: &Vec<(String, Group)>) -> Vec<(Mesh, Group)> {
    let mut meshes = vec![];

    info!("Loading meshes");
    for (key, group) in ents {
        let path = &in_dir.join(format!("entities/{}.obj", key));
        info!("Loading: {}", path.display());
        meshes.push((Mesh::load(path), *group));
    }

    meshes
}
