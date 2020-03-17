//! Photo-dynamic therapy simulation.

use arc::{
    args,
    data::Histogram,
    file::{Load, Save},
    report,
    util::{banner, exec, init, ProgressBar},
};
use attr::form;
use log::info;
use rand::thread_rng;

#[form]
struct Parameters {
    min: f64,
    max: f64,
    num_bins: f64,
    num_samples: f64,
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

    banner::section("Overview");
    let min = params.min;
    let max = params.max;
    let num_samples = params.num_samples as u64;
    let num_bins = params.num_bins as u64;
    info!("Range             : {}\t{}", min, max);
    info!("Number of bins    : {}", num_bins);
    info!("Number of samples : {}", num_samples);

    banner::section("Simulation");
    let mut rng = thread_rng();
    let mut hist = Histogram::new(min, max, num_bins);
    let mut bar = ProgressBar::new("Sampling", num_samples);
    for _ in 0..num_samples {
        bar.tick();

        // let x = rng.gen();
        let x = arc::math::distribution::normal(&mut rng);
        hist.try_collect(x);
    }

    banner::section("Output");
    hist.save(&out_dir.join("hist.csv"));
}
