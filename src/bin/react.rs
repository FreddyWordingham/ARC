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
// use ndarray::Array1;
use std::path::PathBuf;

#[form]
struct Parameters {
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

    banner::section("Overview");
    overview(&verse);

    let _reactor = Reactor::new(verse.reacts(), verse.specs());
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
        self.rates.map(|lambda| lambda.y(concs))
    }
}
