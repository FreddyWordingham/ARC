//! Diffusion testing binary.

use arc::{
    args,
    file::{Load, Verse as VerseForm},
    geom::Aabb,
    report,
    sim::diff,
    util::{banner, exec, init},
};
use attr::form;
use colog;
use log::info;
use std::path::PathBuf;

#[form]
struct Parameters {
    res: [usize; 3],
    bound: Aabb,
    verse: VerseForm,
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

    info!("Loading universe files...");
    let verse = params.verse.form(&in_dir);

    info!("Constructing grid...");
    let grid = diff::Grid::new(params.res, params.bound);

    banner::section("Overview");
    verse.overview();

    banner::section("Pre-Analysis");
    // info!("Saving region map.");
    // grid.regions().save(&out_dir.join("regions.nc"));
    // for (key, map) in grid.state_maps(verse.mats()).map() {
    //     info!("Saving {} state map.", key);
    //     map.save(&out_dir.join(format!("state_map_{}.nc", key)));
    // }

    banner::section("Simulation");

    banner::section("Post-Analysis");

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
