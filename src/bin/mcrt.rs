//! Chemical kinetics testing binary.

use arc::{
    args,
    file::Load,
    ord::{InterKey, InterSet, LightKey, LightSet, MatSet},
    report,
    util::{banner, exec, init},
};
use attr::form;
use colog;
use log::info;
use std::path::PathBuf;

#[form]
struct Parameters {
    inters: Vec<InterKey>,
    light: LightKey,
}

pub fn main() {
    colog::init();
    banner::title(&exec::name());

    banner::section("initialisation");
    let (in_dir, out_dir, params_path) = initialisation();
    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");
    report!(params_path.display(), "parameters path");

    banner::section("Loading");
    info!("Loading parameters file...");
    let params = Parameters::load(&params_path);

    info!("Loading interfaces");
    let inters = InterSet::load(&in_dir.join("interfaces"), &params.inters, "json");
    info!("Loading lights");
    let lights = LightSet::load(&in_dir.join("lights"), &[params.light], "json");
    info!("Loading materials");
    let mats = MatSet::load(&in_dir.join("materials"), &inters.mat_keys(), "json");

    banner::section("Overview");
    // info!("{} reactions:", reacts.map().len());
    // for (i, key) in reacts.map().keys().enumerate() {
    //     println!("\t{}\t{}", i, key);
    // }

    banner::section("Simulation");

    banner::section("Finished");
}

/// Initialise the directories.
fn initialisation() -> (PathBuf, PathBuf, PathBuf) {
    args!(_bin_path: String;
        params_name: String);

    let (in_dir, out_dir) = init::io_dirs(None, None);
    let params_path = &in_dir.join(params_name);

    (in_dir, out_dir, params_path.to_path_buf())
}
