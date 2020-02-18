//! Chemical kinetics testing binary.

use arc::{
    args,
    file::Load,
    ord::{ReactKey, ReactSet, SpecSet},
    report,
    util::{banner, exec, init},
};
use attr::form;
use colog;
use log::info;
use std::path::PathBuf;

#[form]
struct Parameters {
    reacts: Vec<ReactKey>,
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
    let reacts = ReactSet::load(&in_dir.join("reactions"), &params.reacts, "json");
    let specs = SpecSet::load(&in_dir.join("species"), &reacts.spec_keys(), "json");

    banner::section("Overview");
    info!("{} reactions:", reacts.map().len());
    for (i, key) in reacts.map().keys().enumerate() {
        println!("\t{}\t{}", i, key);
    }
    info!("{} species:", specs.map().len());
    for (i, key) in specs.map().keys().enumerate() {
        println!("\t{}\t{}", i, key);
    }
}

fn initialisation() -> (PathBuf, PathBuf, PathBuf) {
    args!(_bin_path: String;
        params_name: String);

    let (in_dir, out_dir) = init::io_dirs(None, None);
    let params_path = &in_dir.join(params_name);

    (in_dir, out_dir, params_path.to_path_buf())
}