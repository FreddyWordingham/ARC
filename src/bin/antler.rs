//! Antler rendering engine.

use arc::{
    args, report,
    util::{banner, exec, init},
};
use attr::form;
use log::info;
use std::path::PathBuf;

#[form]
struct Parameters {}

fn main() {
    colog::init();
    banner::title(&exec::name());

    banner::section("Initialisation");
    let (_in_dir, _out_dir, _param_path) = start();
}

fn start() -> (PathBuf, PathBuf, PathBuf) {
    args!(_bin_path: String;
        params_name: String
    );

    let (in_dir, out_dir) = init::io_dirs(None, None);
    let params_path = in_dir.join(params_name);

    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");
    report!(params_path.display(), "parameters path");

    (in_dir, out_dir, params_path)
}
