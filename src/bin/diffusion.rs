//! Diffusion testing binary.

use arc::{
    args,
    file::{Load, Save, Verse as VerseForm},
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
    let grid = diff::Grid::new(
        params.res,
        params.bound,
        verse.inters(),
        verse.regions(),
        verse.surfs(),
    );

    banner::section("Overview");
    verse.overview();

    banner::section("Pre-Analysis");
    for key in verse.mats().map().keys() {
        info!("Saving {} material map.", key);
        grid.mats()
            .map(|mat| if mat == &key { 1.0 } else { 0.0 })
            .save(&out_dir.join(format!("mat_map_{}.nc", key)));
    }
    for key in verse.states().map().keys() {
        info!("Saving {} state map.", key);
        grid.states()
            .map(|state| if state == &key { 1.0 } else { 0.0 })
            .save(&out_dir.join(format!("state_map_{}.nc", key)));
    }

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
