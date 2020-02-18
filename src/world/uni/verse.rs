//! Material implementation.

use crate::{
    access,
    ord::{InterSet, LightSet, MatSet, ReactSet, RegionSet, SpecSet, StateSet, SurfSet},
};
use log::info;

/// Material physical properties.
pub struct Verse {
    /// Interfaces.
    inters: InterSet,
    /// Regions.
    regions: RegionSet,
    /// Reactions.
    reacts: ReactSet,
    /// Lights.
    lights: LightSet,
    /// Meshes.
    mats: MatSet,
    /// States.
    states: StateSet,
    /// Species.
    specs: SpecSet,
    /// Surfaces.
    surfs: SurfSet,
}

impl Verse {
    access!(inters, InterSet);
    access!(regions, RegionSet);
    access!(reacts, ReactSet);
    access!(lights, LightSet);
    access!(mats, MatSet);
    access!(states, StateSet);
    access!(specs, SpecSet);
    access!(surfs, SurfSet);

    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[must_use]
    pub fn new(
        inters: InterSet,
        regions: RegionSet,
        reacts: ReactSet,
        lights: LightSet,
        mats: MatSet,
        states: StateSet,
        specs: SpecSet,
        surfs: SurfSet,
    ) -> Self {
        Self {
            inters,
            regions,
            reacts,
            lights,
            mats,
            states,
            specs,
            surfs,
        }
    }

    /// Report an overview of the verse.
    #[inline]
    pub fn overview(&self) {
        let num_inters = self.inters().map().len();
        if num_inters > 0 {
            info!("{} interfaces:", num_inters);
            for key in self.inters.map().keys() {
                println!("\t{}", key);
            }
        }

        let num_reacts = self.reacts().map().len();
        if num_reacts > 0 {
            info!("{} reactions:", num_reacts);
            for key in self.reacts.map().keys() {
                println!("\t{}", key);
            }
        }

        let num_lights = self.lights().map().len();
        if num_lights > 0 {
            info!("{} lights:", num_lights);
            for key in self.lights.map().keys() {
                println!("\t{}", key);
            }
        }

        let num_mats = self.mats().map().len();
        if num_mats > 0 {
            info!("{} materials:", num_mats);
            for key in self.mats.map().keys() {
                println!("\t{}", key);
            }
        }

        let num_states = self.states().map().len();
        if num_states > 0 {
            info!("{} states:", num_states);
            for key in self.states.map().keys() {
                println!("\t{}", key);
            }
        }

        let num_specs = self.specs().map().len();
        if num_specs > 0 {
            info!("{} species:", num_specs);
            for key in self.specs.map().keys() {
                println!("\t{}", key);
            }
        }

        let num_surfs = self.surfs().map().len();
        if num_surfs > 0 {
            info!("{} surfaces:", num_surfs);
            for key in self.surfs.map().keys() {
                println!("\t{}", key);
            }
        }
    }
}
