//! Mesh implementation.

use crate::{
    access,
    file::Transform as FileTransform,
    geom::{Mesh, Transform},
    ord::{MeshKey, MeshSet},
};
use attr::json;

/// Mesh building structure.
#[json]
pub struct Surface {
    /// Base mesh key.
    mesh: MeshKey,
    /// Optional transform to apply.
    trans: Option<FileTransform>,
}

impl Surface {
    access!(mesh, mesh_mut, MeshKey);
    access!(trans, Option<FileTransform>);

    /// Build a mesh.
    #[inline]
    #[must_use]
    pub fn build(&self, meshes: &MeshSet) -> Mesh {
        let mut mesh: Mesh = (*meshes.get(&self.mesh)).clone();

        if let Some(trans) = &self.trans {
            mesh.transform(&trans.build());
        }

        mesh
    }
}
