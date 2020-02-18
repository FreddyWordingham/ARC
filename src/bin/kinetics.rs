//! Chemical kinetics testing binary.

use arc::{
    args,
    file::Load,
    ord::{ReactKey, ReactSet, SpecSet},
    report,
    sim::{
        kin,
        kin::{Reactor, Settings},
    },
    util::{banner, exec, init},
    world::State,
};
use attr::form;
use colog;
use log::info;
use ndarray::Array1;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

#[form]
struct Parameters {
    reacts: Vec<ReactKey>,
    state: State,
    sett: Settings,
    iterations: u64,
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

    let reacts = ReactSet::load(&in_dir.join("reactions"), &params.reacts, "json");
    let specs = SpecSet::load(&in_dir.join("species"), &reacts.spec_keys(), "json");
    let mut concs = params.state.new_conc_arr(&specs);
    let _sources = params.state.new_source_arr(&specs);

    banner::section("Overview");
    info!("{} reactions:", reacts.map().len());
    for (i, key) in reacts.map().keys().enumerate() {
        println!("\t{}\t{}", i, key);
    }
    info!("{} species:", specs.map().len());
    for (i, key) in specs.map().keys().enumerate() {
        println!("\t{}\t{}", i, key);
    }

    banner::section("Simulation");
    let mut t = 0.0;

    let mut file = init_file(&out_dir, &specs);
    print_vals(&mut file, t, &concs);

    let reactor = Reactor::new(&reacts, &specs);

    let mut pb = arc::util::ProgressBar::new("Kinetics", params.iterations);
    for _ in 0..params.iterations {
        kin::run_with_reactor(&params.sett, &reactor, &mut concs);
        pb.tick();
        t += params.sett.time();
        print_vals(&mut file, t, &concs);
    }
    pb.finish_with_message("Kinetics complete.");

    // *concs
    //     .get_mut(specs.index_of_key(&arc::ord::SpecKey::new("o2")))
    //     .expect("Invalid index.") += 1.0;
    // *concs
    //     .get_mut(specs.index_of_key(&arc::ord::SpecKey::new("udens")))
    //     .expect("Invalid index.") += 1.0;
    // for _ in 0..params.iterations {
    //     kin::run_with_reactor(&params.sett, &reactor, &mut concs);
    //     pb.tick();
    //     t += params.sett.time();
    //     print_vals(&mut file, t, &concs);
    // }
    // pb.finish_with_message("Kinetics complete.");

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

/// Initialise the output file.
fn init_file(out_dir: &Path, specs: &SpecSet) -> File {
    let mut file = std::fs::File::create(&out_dir.join("concs.dat")).expect("Unable to open file.");

    write!(file, "{:>32}", "time".to_string()).expect("Unable to write to file.");
    for key in specs.map().keys() {
        write!(file, ",{:>32}", format!("{}", key)).expect("Unable to write to file.");
    }
    writeln!(file).expect("Unable to write to file.");

    file
}

/// Print the time and current concentration values to a file.
fn print_vals(file: &mut File, t: f64, cs: &Array1<f64>) {
    write!(file, "{:>32}", t).expect("Unable to write to file.");
    for c in cs {
        write!(file, ",{:>32}", c).expect("Unable to write to file.");
    }
    writeln!(file).expect("Unable to write to file.");
}
