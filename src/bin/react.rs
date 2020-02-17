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

use arc::{access, math::Multivariate};
use ndarray::Array1;

pub struct Reactor {
    /// Rate formulae.
    rates: Array1<Multivariate>,
}

impl Reactor {
    access!(rates, Array1<Multivariate>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(rates: Array1<Multivariate>) -> Self {
        Self { rates }
    }
}
