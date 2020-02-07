//! Surface set.

use crate::{
    file::Surface as SurfaceForm,
    geom::Mesh,
    ord::{MeshSet, Set, SurfKey},
};
use std::collections::BTreeMap;

/// Alias for the surface set.
pub type SurfSet = Set<SurfKey, Mesh>;

impl SurfSet {
    /// Form a surface set from a mesh set.
    #[inline]
    #[must_use]
    pub fn build(proto_surfs: &Set<SurfKey, SurfaceForm>, mesh_set: &MeshSet) -> Self {
        let mut map = BTreeMap::new();

        for (key, form) in proto_surfs.map() {
            map.insert(key.clone(), form.build(mesh_set));
        }

        Self::new(map)
    }
}
