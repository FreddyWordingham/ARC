//! Zebra ray-tracing adaptive engine.

use arc::{
    args,
    file::Camera as FileCamera,
    ord::key::MeshKey,
    report,
    util::{banner, exec, init},
};
use attr::form;

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
}
