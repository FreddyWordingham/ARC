//! Monte-Carlo radiative transfer testing binary.

use arc::{
    args,
    file::{Load, Save, Verse as VerseForm},
    geom::Aabb,
    ord::{LightKey, MatKey},
    report,
    sim::mcrt,
    util::{banner, exec, init},
};
use attr::form;
use colog;
use log::info;
use std::{io::Write, path::PathBuf};

#[form]
struct Parameters {
    res: [usize; 3],
    bound: Aabb,
    verse: VerseForm,
    num_phot: f64,
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

    info!("Loading universe files...");
    let verse = params.verse.form(&in_dir);

    info!("Constructing grid...");
    let grid = mcrt::Grid::new(params.res, params.bound, verse.inters(), verse.surfs());

    banner::section("Overview");
    verse.overview();

    banner::section("Pre-Analysis");
    info!("Saving interface map.");
    grid.interfaces().save(&out_dir.join("interfaces.nc"));
    for (key, map) in grid.mat_maps(verse.mats()).map() {
        info!("Saving {} material map.", key);
        map.save(&out_dir.join(format!("mat_map_{}.nc", key)));
    }

    banner::section("Simulation");
    let lm = mcrt::run(
        params.num_phot as u64,
        verse.lights().get(&params.light),
        &grid,
        verse.surfs(),
        verse.mats(),
    );

    banner::section("Post-Analysis");
    lm.save(&out_dir);

    let mat_keys = grid.mat_keys();
    let abs_dens = lm.abs_dens();
    let ave_abs_dens = abs_dens.sum() / abs_dens.len() as f64;
    report!(ave_abs_dens);
    let tumour_abs_sum: f64 = abs_dens
        .iter()
        .zip(&mat_keys)
        .map(|(abs, key)| {
            if *key == &MatKey::new("tumour") {
                *abs
            } else {
                0.0
            }
        })
        .sum();
    let num_tumour_cells = mat_keys
        .map(|key| {
            if *key == &MatKey::new("tumour") {
                1.0
            } else {
                0.0
            }
        })
        .sum();
    report!(num_tumour_cells);
    let tumour_abs_ave = tumour_abs_sum / num_tumour_cells;
    report!(tumour_abs_ave);

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(&in_dir.join("results.txt"))
        .expect("Unable to open results file.");
    writeln!(file, "tumour_abs_ave:\t{}", tumour_abs_ave).expect("Could not write to file.");

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
