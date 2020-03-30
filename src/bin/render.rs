//! Random-number testing binary.

use arc::{
    args,
    file::{Camera as FileCamera, Load},
    ord::{key::MeshKey, MeshSet},
    report,
    util::{banner, exec, init},
};
use attr::form;
use log::info;

#[form]
struct Parameters {
    camera: FileCamera,
    entities: Vec<MeshKey>,
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
    info!("Loading parameters file...");
    let params = Parameters::load(&params_path);
    let cam = params.camera.build();
    report!(cam.num_pix());
    let _ents = MeshSet::load(&in_dir.join("entities"), &params.entities, "obj");

    banner::section("Rendering");
    let stack = arc::sim::render::run(&cam);

    banner::section("Saving");
    for (key, _img) in stack {
        let path = &out_dir.join(format!("{}.nc", key));
        report!(path.display());
    }
}
