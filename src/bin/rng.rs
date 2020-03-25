//! Random-number testing binary.

use arc::{
    args,
    data::Histogram,
    file::{Load, Save},
    report,
    util::{banner, exec, init, ParProgressBar},
};
use attr::form;
use log::info;
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

#[form]
struct Parameters {
    samples: f64,
    block_size: f64,
    min: f64,
    max: f64,
    num_bins: f64,
    pdf: arc::file::json::Probability,
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
    let samples = params.samples as u64;
    report!(samples);
    let block_size = params.block_size as u64;
    report!(block_size);
    let min = params.min;
    report!(min);
    let max = params.max;
    report!(max);
    let num_bins = params.num_bins as u64;
    report!(num_bins);
    let pdf = params.pdf.build();

    banner::section("Simulation");
    let pb = ParProgressBar::new("Sampling", samples);
    let pb = Arc::new(Mutex::new(pb));
    let thread_ids: Vec<usize> = (0..num_cpus::get()).collect();

    let mut hists: Vec<_> = thread_ids
        .par_iter()
        .map(|_| {
            run_thread(
                &Arc::clone(&pb),
                block_size,
                min,
                max,
                num_bins,
                pdf.clone(),
            )
        })
        .collect();
    pb.lock()
        .expect("Could not lock progress bar.")
        .finish_with_message("Complete.");

    let mut data = hists.pop().expect("Did not receive any histogram data.");
    for hist in hists {
        data += &hist;
    }

    banner::section("Output");
    data.save(&out_dir.join("counts.csv"));
}

fn run_thread(
    pb: &Arc<Mutex<ParProgressBar>>,
    block_size: u64,
    min: f64,
    max: f64,
    num_bins: u64,
    pdf: arc::math::rng::Probability,
) -> Histogram {
    let mut rng = thread_rng();
    let mut hist = Histogram::new(min, max, num_bins);

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        for _ in start..end {
            // let x = rng.gen();
            let x = pdf.gen(&mut rng);
            // arc::math::distribution::henyey_greenstein(&mut rng, 0.5) / std::f64::consts::PI;

            hist.collect(x);
        }
    }

    hist
}
