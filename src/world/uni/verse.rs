//! Material implementation.

use crate::{
    access,
    ord::{InterSet, LightSet, MatSet, ReactSet, RegionSet, SpecSet, StateSet, SurfSet},
};

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
}
