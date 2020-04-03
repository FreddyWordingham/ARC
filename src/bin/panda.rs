//! Panda rendering engine!

use arc::{
    args,
    file::{Camera, Load, Transform as FileTransform},
    geom::{Mesh, Transform},
    report,
    sim::panda::{Cell, GridSettings, Group, ShaderSettings},
    util::{banner, exec, init},
};
use attr::form;
use log::info;
use std::{collections::BTreeMap, path::Path};

#[form]
struct Parameters {
    /// Grid setup information.
    grid_settings: GridSettings,
    /// Shader information.
    shader_settings: ShaderSettings,
    /// Traceable surfaces.
    surfaces: Vec<(Group, Vec<(String, Option<FileTransform>)>)>,
    /// Cameras to take images with.
    cameras: BTreeMap<String, Camera>,
}

fn main() {
    colog::init();
    banner::title(&exec::name());
    banner::section("Initialisation");
    args!(_bin_path: String;
        params_name: String);

    let (in_dir, out_dir) = init::io_dirs(None, None);
    let params_path = &in_dir.join(params_name);

    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");
    report!(params_path.display(), "parameters path");

    banner::section("Input");
    let params = load_parameters(params_path);

    banner::section("Loading");
    let surfs = load_surfs(&in_dir, &params.surfaces);

    banner::section("Building");
    let _grid = build_grid(&params.grid_settings, &surfs);

    banner::section("Rendering");
    for (name, cam) in params.cameras {
        let cam = cam.build();
        info!("{} camera{}", name, cam);
    }
}

/// Load the parameters file and report the settings.
#[inline]
#[must_use]
fn load_parameters(path: &Path) -> Parameters {
    info!("Loading parameters...");
    let params = Parameters::load(&path);

    report!(&params.grid_settings, "Grid settings");
    report!(&params.shader_settings, "Shader settings");

    info!("Cameras:");
    for (name, cam) in &params.cameras {
        report!(cam, name);
    }

    params
}

/// Load the base meshes and transform them into their final surfaces.
#[inline]
#[must_use]
fn load_surfs(
    in_dir: &Path,
    list: &[(Group, Vec<(String, Option<FileTransform>)>)],
) -> Vec<(Group, Vec<Mesh>)> {
    info!("Loading surfaces...");
    let mut surfs: BTreeMap<Group, Vec<_>> = BTreeMap::new();

    for (group, meshes) in list {
        for (name, transform) in meshes {
            let path = &in_dir.join(format!("surfaces/{}.obj", name));
            info!("Loading {}", path.display());
            let mut mesh = Mesh::load(path);

            if let Some(trans) = transform {
                info!("applying transformation");
                mesh.transform(&trans.build());
            }

            info!("{} mesh triangles: {}", name, mesh.tris().len());

            if let Some(entry) = surfs.get_mut(group) {
                entry.push(mesh);
            } else {
                surfs.insert(*group, vec![mesh]);
            }
        }
    }
    info!("{} meshes loaded.\n", surfs.len());

    let mut surfaces = Vec::with_capacity(surfs.len());
    info!("Total groups: {}", surfs.len());
    for (group, meshes) in surfs {
        let mut ms = Vec::with_capacity(meshes.len());
        for mesh in meshes {
            ms.push(mesh);
        }
        info!("Group {} contains {} surfaces.", group, ms.len());
        surfaces.push((group, ms));
    }

    surfaces
}

/// Build the world grid.
#[inline]
#[must_use]
fn build_grid<'a>(grid_settings: &GridSettings, surfaces: &'a [(Group, Vec<Mesh>)]) -> Cell<'a> {
    info!("Building grid...");
    report!(grid_settings.max_depth(), "max depth");
    report!(grid_settings.tar_tris(), "target triangles");

    let grid = Cell::new_root(grid_settings, surfaces);

    grid
}
