//! Main binary.

use arc::{
    args,
    file::{Load, Save, Verse as VerseForm},
    geom::Aabb,
    report,
    sim::diff,
    util::{banner, exec, init},
};
use attr::form;
use log::info;

#[form]
struct Parameters {
    verse: VerseForm,
    bound: Aabb,
    res: [usize; 3],
}

pub fn main() {
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
    let mut concs = grid.concs(verse.states(), verse.specs());
    let viscs = grid.visc(verse.mats());
    let ala_index = verse.specs().index_of_key(&arc::ord::SpecKey::new("ala"));
    let o2_index = verse.specs().index_of_key(&arc::ord::SpecKey::new("o2"));
    let total_steps = 200;
    for n in 0..total_steps {
        println!("n: {}/{}", n, total_steps);
        concs
            .map(|cs| *cs.get(ala_index).expect("Invalid index."))
            .save(&out_dir.join(format!("ala_{}.nc", n)));
        concs
            .map(|cs| *cs.get(o2_index).expect("Invalid index."))
            .save(&out_dir.join(format!("o2_{}.nc", n)));
        diff::run(6.0, &grid, verse.specs(), &mut concs, &viscs);
    }
    concs
        .map(|cs| *cs.get(ala_index).expect("Invalid index."))
        .save(&out_dir.join(format!("ala_{}.nc", total_steps)));
    concs
        .map(|cs| *cs.get(ala_index).expect("Invalid index."))
        .save(&out_dir.join(format!("o2_{}.nc", total_steps)));
}
