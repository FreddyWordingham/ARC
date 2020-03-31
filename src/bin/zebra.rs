//! Zebra ray-tracing adaptive engine.

use arc::{
    args,
    file::{Camera, Load},
    report,
    sim::render::Group,
    util::{banner, exec, init},
};
use attr::form;
use log::info;

#[form]
struct Parameters {
    camera: Camera,
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
    info!("Building camera");
    let cam = params.camera.build();
    report!(cam.num_pix() as f64 / 100000.0, "Total pixels", "Million");
    report!(cam.fov().0.to_degrees(), "Horizontal fov", "Degrees");
    report!(cam.fov().1.to_degrees(), "Vertical fov", "Degrees");
    info!("Loading meshes");
    for (key, _group) in params.entities {
        let path = &in_dir.join(format!("entities/{}.obj", key));
        info!("Loading: {}", path.display());
    }
}
