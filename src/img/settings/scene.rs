//! Scene input settings implementation.

use crate::{
    access,
    file::{Load, Transform as FileTransform},
    geom::{Mesh, Transform},
    sim::render::{Group, Scene as RenderScene},
};
use attr::json;
use nalgebra::Point3;
use std::{collections::BTreeMap, path::Path};

/// Scene settings.
#[allow(clippy::type_complexity)]
#[json]
pub struct Scene {
    /// Sun position.
    sun_pos: Point3<f64>,
    /// Traceable surface groups.
    groups: Vec<(Group, Vec<(String, Option<FileTransform>)>)>,
}

impl Scene {
    access!(sun_pos, Point3<f64>);
    access!(groups, Vec<(Group, Vec<(String, Option<FileTransform>)>)>);

    /// Build a rendering scene.
    #[inline]
    #[must_use]
    pub fn build(&self, in_dir: &Path) -> RenderScene {
        let mut surfs: BTreeMap<Group, Vec<_>> = BTreeMap::new();
        for (group, meshes) in &self.groups {
            for (name, transform) in meshes {
                let path = in_dir.join(format!("{}.obj", name));
                let mut mesh = Mesh::load(&path);

                if let Some(transform) = transform {
                    mesh.transform(&transform.build());
                }

                if let Some(entry) = surfs.get_mut(group) {
                    entry.push(mesh);
                } else {
                    surfs.insert(*group, vec![mesh]);
                }
            }
        }

        RenderScene::new(self.sun_pos, surfs)
    }
}
