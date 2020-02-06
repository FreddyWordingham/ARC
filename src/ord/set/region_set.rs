//! Region set.

use crate::{
    ord::{RegionKey, Set},
    world::Region,
};

/// Alias for the region set.
pub type RegionSet = Set<RegionKey, Region>;
