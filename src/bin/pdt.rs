//! Photo-dynamic therapy simulation.

// use arc::args;
use arc::util::{banner, exec};
// use attr::form;
use log::info;

fn main() {
    colog::init();
    banner::title(&exec::name());

    info!("Hello world!");
}
