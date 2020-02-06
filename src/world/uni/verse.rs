//! Material implementation.

use crate::{
    access,
    ord::{InterSet, LightSet, MatSet, ReactSet, RegionSet, SpecSet, SurfSet},
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
        specs: SpecSet,
        surfs: SurfSet,
    ) -> Self {
        Self {
            inters,
            regions,
            reacts,
            lights,
            mats,
            specs,
            surfs,
        }
    }
}
