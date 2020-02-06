//! Interface implementation.

use crate::{
    access,
    ord::{MatKey, SurfKey},
};
use attr::json;

/// Material interface structure.
#[json]
pub struct Interface {
    /// Surface mesh.
    surf: SurfKey,
    /// Inside material.
    in_mat: MatKey,
    /// Outside material.
    out_mat: MatKey,
}

impl Interface {
    access!(surf, SurfKey);
    access!(in_mat, MatKey);
    access!(out_mat, MatKey);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(surf: SurfKey, in_mat: MatKey, out_mat: MatKey) -> Self {
        Self {
            surf,
            in_mat,
            out_mat,
        }
    }
}
