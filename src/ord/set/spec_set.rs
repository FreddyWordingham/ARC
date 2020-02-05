//! Species set.

use crate::{
    chem::Species,
    ord::{Set, SpecKey},
};

/// Alias for the species set.
pub type SpecSet = Set<SpecKey, Species>;
