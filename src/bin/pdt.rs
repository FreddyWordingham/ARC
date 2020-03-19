//! Photo-dynamic therapy simulation.

use arc::{
    args,
    file::{Load, Save, Verse as VerseForm},
    geom::Aabb,
    report,
    sim::mcrt,
    util::{banner, exec, init},
};
use attr::form;
use log::info;

#[form]
struct Parameters {
    res: [usize; 3],
    bound: Aabb,
    verse: VerseForm,
}

fn main() {
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

    banner::section("Output");
}
