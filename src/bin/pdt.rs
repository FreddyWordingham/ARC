//! Photo-dynamic therapy simulation.

use arc::{
    args,
    file::{Load, Verse as VerseForm},
    report,
    util::{banner, exec, init},
};
use attr::form;
use log::info;

#[form]
struct Parameters {
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

    banner::section("Overview");
    verse.overview();

    banner::section("Simulation");

    banner::section("Output");
}
