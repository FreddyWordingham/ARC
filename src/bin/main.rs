//! Main function.

use arc::util::{banner, exec};
use attr::form;
use colog;
use log::info;

#[form]
struct Parameters {
    num_phot: f64,
}

fn main() {
    colog::init();
    banner::title(&exec::name());

    info!("Hello world!");
}
