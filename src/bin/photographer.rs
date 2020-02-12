//! Photographer function.

use arc::{
    args,
    file::{Grid as GridForm, Load, Verse as VerseForm},
    report, rows,
    util::{banner, exec, init},
    world::Verse,
};
use attr::form;
use colog;
use log::info;
use std::path::PathBuf;

#[form]
struct Parameters {
    num_threads: usize,
    res: [u64; 2],
    verse: VerseForm,
    grid: GridForm,
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
    report!(params.res.get(0).expect("Missing index."), "Hr");
    report!(params.res.get(1).expect("Missing index."), "Vr");

    info!("Loading universe files...");
    let verse = params.verse.form(&in_dir);

    banner::section("Building");
    info!("Building grid...");
    let grid = params.grid.form(params.num_threads, &verse);
    let mat_maps = grid.mat_maps(verse.mats());

    banner::section("Overview");
    overview(&verse);
    info!("Material mapping breakdown:");
    for (key, map) in mat_maps.map() {
        let count = map.sum();
        let fraction = count / map.len() as f64 * 100.0;
        rows!(format!("{}", key), count, format!("{}%", fraction));
    }
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

    info!("{} lights:", verse.lights().map().len());
    for key in verse.lights().map().keys() {
        info!("\t{}", key);
    }

    info!("{} materials:", verse.mats().map().len());
    for key in verse.mats().map().keys() {
        info!("\t{}", key);
    }

    info!("{} surfaces:", verse.surfs().map().len());
    for key in verse.surfs().map().keys() {
        info!("\t{}", key);
    }
}
