//! Main function.

use arc::{
    args,
    file::{Grid as GridForm, Load, Save, Verse as VerseForm},
    report, rows,
    sim::diff,
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
    num_threads: usize,
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
    report!(params.num_threads);

    info!("Loading universe files...");
    let verse = params.verse.form(&in_dir);

    banner::section("Building");
    info!("Building grid...");
    let mut grid = params.grid.form(params.num_threads, &verse);
    let mat_maps = grid.mat_maps(verse.mats());
    let state_maps = grid.state_maps(verse.states());
    let spec_maps = grid.spec_set_refs(verse.specs());

    banner::section("Overview");
    overview(&verse);
    info!("Material mapping breakdown:");
    for (key, map) in mat_maps.map() {
        let count = map.sum();
        let fraction = count / map.len() as f64 * 100.0;
        rows!(format!("{}", key), count, format!("{}%", fraction));
    }
    info!("State mapping breakdown:");
    for (key, map) in state_maps.map() {
        let count = map.sum();
        let fraction = count / map.len() as f64 * 100.0;
        rows!(format!("{}", key), count, format!("{}%", fraction));
    }
    info!("Species mapping breakdown:");
    for (key, map) in spec_maps.map() {
        let total: f64 = map.map(|x| **x).sum();
        rows!(format!("{}", key), total);
    }
    let inter_boundaries = grid.inter_boundaries();

    banner::section("Saving");
    for (key, map) in mat_maps.map() {
        map.save(&out_dir.join(format!("mat_map_{}.nc", key)));
    }
    for (key, map) in state_maps.map() {
        map.save(&out_dir.join(format!("state_map_{}.nc", key)));
    }
    for (key, map) in spec_maps.map() {
        map.map(|x| **x)
            .save(&out_dir.join(format!("spec_map_{}.nc", key)));
    }
    inter_boundaries.save(&out_dir.join("boundaries_interfaces.nc"));

    banner::section("Simulation");

    let ala_index = verse.specs().index_of_key(&arc::ord::SpecKey::new("ala"));
    // let o2_index = verse.specs().index_of_key(&arc::ord::SpecKey::new("o2"));

    let mut time = 0.0_f64;
    let end_time = 60.0 * 20.0;
    let dt = 10.0;
    let steps = ((end_time - time) / dt).floor() as i64;

    grid.cells()
        .map(|c| *c.concs().get(ala_index).unwrap())
        .save(&out_dir.join(format!("ala_{}.nc", (time * 1.0e3).floor() as i64)));

    for _ in 0..steps {
        time += dt;
        grid.cells()
            .map(|c| *c.concs().get(ala_index).unwrap())
            .save(&out_dir.join(format!("ala_{}.nc", time.floor() as i64)));
        // grid.cells()
        //     .map(|c| *c.concs().get(o2_index).unwrap())
        //     .save(&out_dir.join(format!("o2_{}.nc", (time * 1.0e3).floor() as i64)));
        diff::run(params.num_threads, dt, &verse, &mut grid);
    }

    // let lm = mcrt::run(
    //     params.num_threads,
    //     params.num_phot as u64,
    //     &LightKey::new("led"),
    //     &verse,
    //     &grid,
    // );

    // banner::section("Saving II");
    // lm.save(&out_dir);

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

    info!("{} states:", verse.states().map().len());
    for key in verse.states().map().keys() {
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
