//! Main function.

use arc::{
    args,
    file::{Grid as GridForm, Load, Verse as VerseForm},
    report,
    util::{banner, exec, init},
    world::Verse,
};
use attr::form;
use colog;
use log::info;
use std::path::PathBuf;

#[form]
struct Parameters {
    num_phot: f64,
    verse: VerseForm,
    grid: GridForm,
}

fn main() {
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

    info!("Loading universe files...");
    let verse = params.verse.form(&in_dir);

    banner::section("Building");
    info!("Building grid...");
    let _grid = params.grid.form(&verse);

    banner::section("Overview");
    overview(&verse);

    // for inter in params.verse.inters() {
    //     println!("Loading interface: {}", inter);
    // }

    banner::section("Finished");
}

fn initialisation() -> (PathBuf, PathBuf, PathBuf) {
    args!(_bin_path: String;
        params_name: String);

    let (in_dir, out_dir) = init::io_dirs(None, None);
    let params_path = &in_dir.join(params_name);

    (in_dir, out_dir, params_path.to_path_buf())
}

fn overview(verse: &Verse) {
    info!("{} interfaces:", verse.inters().map().len());
    for key in verse.inters().map().keys() {
        info!("\t{}", key);
    }

    info!("{} reactions:", verse.reacts().map().len());
    for key in verse.reacts().map().keys() {
        info!("\t{}", key);
    }

    info!("{} lights:", verse.lights().map().len());
    for key in verse.lights().map().keys() {
        info!("\t{}", key);
    }

    info!("{} materials:", verse.mats().map().len());
    for key in verse.mats().map().keys() {
        info!("\t{}", key);
    }

    info!("{} species:", verse.specs().map().len());
    for key in verse.specs().map().keys() {
        info!("\t{}", key);
    }

    info!("{} surfaces:", verse.surfs().map().len());
    for key in verse.surfs().map().keys() {
        info!("\t{}", key);
    }
}
