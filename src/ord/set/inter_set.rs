//! Intersection set.

use crate::{
    geom::{Aabb, Ray, Trace},
    ord::{InterKey, MatKey, Set, SurfKey, SurfSet},
    world::Interface,
};

/// Alias for the interface set.
pub type InterSet = Set<InterKey, Interface>;

impl InterSet {
    /// Get a list of all surface keys used by the interface set.
    #[inline]
    #[must_use]
    pub fn surf_keys(&self) -> Vec<SurfKey> {
        self.map
            .values()
            .map(|inter| inter.surf().clone())
            .collect()
    }

    /// Get a list of all material keys used by the interface set.
    #[inline]
    #[must_use]
    pub fn mat_keys(&self) -> Vec<MatKey> {
        let in_mats: Vec<_> = self
            .map
            .values()
            .map(|inter| inter.in_mat().clone())
            .collect();

        let mut out_mats: Vec<_> = self
            .map
            .values()
            .map(|inter| inter.out_mat().clone())
            .collect();

        let mut mats = in_mats;
        mats.append(&mut out_mats);

        mats
    }

    /// Determine which material, if any, would be observed with a given ray.
    #[inline]
    #[must_use]
    pub fn observe_mat(&self, surfs: &SurfSet, bound: &Aabb, ray: &Ray) -> Option<MatKey> {
        assert!(bound.contains(ray.pos()));

        let mut nearest: Option<(&MatKey, f64)> = None;

        for inter in self.map().values() {
            if let Some((dist, inside)) = surfs.get(inter.surf()).dist_inside(ray) {
                if nearest.is_none()
                    || dist
                        < nearest
                            .expect("Something went wrong that shouldn't have.")
                            .1
                {
                    nearest = Some((
                        if inside {
                            inter.in_mat()
                        } else {
                            inter.out_mat()
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

            return Some(key.clone());
        }

        None
    }
}
