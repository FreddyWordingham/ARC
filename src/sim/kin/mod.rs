//! Chemical kinetics simulation sub-module.

pub mod reactor;
pub mod settings;

pub use self::{reactor::*, settings::*};

use ndarray::Array1;
use ndarray_stats::QuantileExt;

/// Run a kinetics reaction simulation with an existing reactor.
#[inline]
pub fn run_with_reactor(
    sett: &Settings,
    reactor: &Reactor,
    concs: &mut Array1<f64>,
    multiplier: f64,
) {
    let mut t = 0.0;
    while t < sett.time() {
        let rates = reactor.calc_rates(concs) * multiplier;
        // println!("{}\t{:?}", t, rates);

        let dt = ((&*concs / &rates) * sett.max_conc_frac_delta())
            .mapv(f64::abs)
            .mapv(|x| if x.is_nan() { std::f64::MAX } else { x })
            .min()
            .expect("Could not determine minimum timestep.")
            .max(sett.min_timestep())
            .min(sett.time() - t);

        let k1 = &rates * dt;
        let k2 = (&rates + &(&k1 / 2.0)) * dt;
        let k3 = (&rates + &(&k2 / 2.0)) * dt;
        let k4 = (rates + &k3) * dt;

        *concs += &((k1 + (2.0 * k2) + (2.0 * k3) + k4) / 6.0);

        t += dt;
    }
}
