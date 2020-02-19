//! Region set.

use crate::{
    geom::{Aabb, Ray, Trace},
    ord::{RegionKey, Set, StateKey, SurfKey, SurfSet},
    world::Region,
};

/// Alias for the region set.
pub type RegionSet = Set<RegionKey, Region>;

impl RegionSet {
    /// Get a list of all surface keys used by the interface set.
    #[inline]
    #[must_use]
    pub fn surf_keys(&self) -> Vec<SurfKey> {
        self.map
            .values()
            .map(|inter| inter.surf().clone())
            .collect()
    }

    /// Get a list of all state keys used by the interface set.
    #[inline]
    #[must_use]
    pub fn state_keys(&self) -> Vec<StateKey> {
        let in_states: Vec<_> = self
            .map
            .values()
            .map(|inter| inter.in_state().clone())
            .collect();

        let mut out_states: Vec<_> = self
            .map
            .values()
            .map(|inter| inter.out_state().clone())
            .collect();

        let mut states = in_states;
        states.append(&mut out_states);

        states
    }

    /// Determine which state, if any, would be observed with a given ray.
    #[inline]
    #[must_use]
    pub fn observe_state(&self, surfs: &SurfSet, bound: &Aabb, ray: &Ray) -> Option<&StateKey> {
        debug_assert!(bound.contains(ray.pos()));

        let mut nearest: Option<(&StateKey, f64)> = None;

        for region in self.map().values() {
            if let Some((dist, inside)) = surfs.get(region.surf()).dist_inside(ray) {
                if nearest.is_none()
                    || dist
                        < nearest
                            .expect("Something went wrong that shouldn't have.")
                            .1
                {
                    nearest = Some((
                        if inside {
                            region.in_state()
                        } else {
                            region.out_state()
                        },
                        dist,
                    ));
                }
            }
        }

        if let Some((key, dist)) = nearest {
            let bound_dist = bound
                .dist(ray)
                .expect("Observation ray did not hit boundary.");

            if bound_dist < dist {
                return None;
            }

            return Some(key);
        }

        None
    }
}
