//! Reaction testing function.

use arc::{
    args,
    file::{Load, Verse as VerseForm},
    report,
    util::{banner, exec, init},
    world::Verse,
};
use attr::form;
use colog;
use log::info;
use ndarray_stats::QuantileExt;
use std::path::PathBuf;

/// Maximum fraction a concentration can change during a single timestep.
const MAX_FRAC_DELTA: f64 = 0.1;

#[form]
struct Parameters {
    verse: VerseForm,
    min_timestep: f64,
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

    banner::section("Overview");
    overview(&verse);

    let reactor = Reactor::new(verse.reacts(), verse.specs());
    // println!("Reactor:\n{:?}", reactor);

    let mut time = 0.0;
    let mut dt;
    let mut concs: Array1<f64> = Array1::zeros(verse.specs().map().len());
    for c in concs.iter_mut() {
        *c += 1.0;
    }

    let mut file = std::fs::File::create(&out_dir.join("concs.dat")).expect("Unable to open file.");
    write!(file, "{:>24}", "time".to_string()).expect("Unable to write to file.");
    for key in verse.specs().map().keys() {
        write!(file, ",{:>24}", format!("{}", key)).expect("Unable to write to file.");
    }
    writeln!(file).expect("Unable to write to file.");

    print_vals(&mut file, time, &concs);
    while time < 100.0 {
        println!("t: {}", time);
        let rates = reactor.calc_rates(&concs);
        dt = (&concs / &rates * MAX_FRAC_DELTA)
            .mapv(f64::abs)
            .min()
            .unwrap()
            .max(params.min_timestep);
        time += dt;

        let k1 = &rates * dt;
        let k2 = (&rates + &(&k1 / 2.0)) * dt;
        let k3 = (&rates + &(&k2 / 2.0)) * dt;
        let k4 = (rates + &k3) * dt;

        concs += &((k1 + (2.0 * k2) + (2.0 * k3) + k4) / 6.0);

        // concs += &(rates * dt);

        print_vals(&mut file, time, &concs);
    }
}

fn initialisation() -> (PathBuf, PathBuf, PathBuf) {
    args!(_bin_path: String;
        params_name: String);

    let (in_dir, out_dir) = init::io_dirs(None, None);
    let params_path = &in_dir.join(params_name);

    (in_dir, out_dir, params_path.to_path_buf())
}

fn overview(verse: &Verse) {
    info!("{} reactions:", verse.reacts().map().len());
    for (key, react) in verse.reacts().map() {
        info!("{}", format!("{}:\n{}", key, react));
    }
}

use arc::{
    access,
    math::Multivariate,
    ord::{ReactSet, SpecSet},
};
use ndarray::{Array1, Array2};
use std::{fs::File, io::Write};

#[derive(Debug)]
pub struct Reactor {
    /// Rate formulae.
    rates: Array1<Multivariate>,
    /// Coefficents.
    cs: Array2<f64>,
}

impl Reactor {
    access!(rates, Array1<Multivariate>);
    access!(cs, Array2<f64>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(reacts: &ReactSet, specs: &SpecSet) -> Self {
        let rates = reacts
            .map()
            .values()
            .map(|r| r.rate().create_lambda(specs))
            .collect();

        let mut cs = Array2::zeros((reacts.map().len(), specs.map().len()));

        for (i, react) in reacts.map().values().enumerate() {
            for (r, c) in react.reactants() {
                *cs.get_mut((i, specs.index_of_key(r)))
                    .expect("Invalid index.") -= *c as f64;
            }

            for (p, c) in react.products() {
                *cs.get_mut((i, specs.index_of_key(p)))
                    .expect("Invalid index.") += *c as f64;
            }
        }

        Self { rates, cs }
    }

    /// Determine the current reaction rates.
    #[inline]
    #[must_use]
    pub fn calc_rates(&self, concs: &Array1<f64>) -> Array1<f64> {
        let rs = self.rates.map(|lambda| lambda.y(concs));

        let mut rates = Array1::zeros(concs.len());

        for (i, r) in rs.iter().enumerate() {
            for (j, _c) in concs.iter().enumerate() {
                *rates.get_mut(j).unwrap() += r * self.cs.get((i, j)).unwrap();
            }
        }

        rates
    }
}

/// Print the time and current concentration values to a file.
fn print_vals(file: &mut File, t: f64, cs: &Array1<f64>) {
    write!(file, "{:>24}", t).expect("Unable to write to file.");
    for c in cs {
        write!(file, ",{:>24}", c).expect("Unable to write to file.");
    }
    writeln!(file).expect("Unable to write to file.");
}
