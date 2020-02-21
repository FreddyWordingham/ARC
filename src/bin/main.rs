//! Main binary.

use arc::{
    args,
    file::{Load, Save, Verse as VerseForm},
    geom::Aabb,
    ord::{LightKey, SpecKey},
    report,
    sim::{diff, mcrt},
    util::{banner, exec, init},
};
use attr::form;
use log::info;

#[form]
struct Parameters {
    verse: VerseForm,
    bound: Aabb,
    res: [usize; 3],
    num_phot: u64,
    light: LightKey,
}

pub fn main() {
    colog::init();
    banner::title(&exec::name());

    banner::section("initialisation");
    args!(_bin_path: String;
        params_name: String);

    let (in_dir, out_dir) = init::io_dirs(None, None);
    let params_path = &in_dir.join(params_name);

    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");
    report!(params_path.display(), "parameters path");

    banner::section("Loading");
    info!("Loading parameters file...");
    let params = Parameters::load(&params_path);

    info!("Loading universe files...");
    let verse = params.verse.form(&in_dir);

    banner::section("Overview");
    verse.overview();

    banner::section("Diffusion");
    let mut concs = {
        info!("Constructing grid...");
        let diff_grid = diff::Grid::new(
            params.res,
            params.bound.clone(),
            verse.inters(),
            verse.regions(),
            verse.surfs(),
        );

        for key in verse.mats().map().keys() {
            info!("Saving {} material map.", key);
            diff_grid
                .mats()
                .map(|mat| if mat == &key { 1.0 } else { 0.0 })
                .save(&out_dir.join(format!("mat_map_{}.nc", key)));
        }
        for key in verse.states().map().keys() {
            info!("Saving {} state map.", key);
            diff_grid
                .states()
                .map(|state| if state == &key { 1.0 } else { 0.0 })
                .save(&out_dir.join(format!("state_map_{}.nc", key)));
        }

        let mut concs = diff_grid.concs(verse.states(), verse.specs());
        let viscs = diff_grid.visc(verse.mats());
        let ala_index = verse.specs().index_of_key(&arc::ord::SpecKey::new("ala"));
        let o2_index = verse.specs().index_of_key(&arc::ord::SpecKey::new("o2"));
        let total_steps = 200;
        for n in 0..total_steps {
            println!("n: {}/{}", n, total_steps);
            concs
                .map(|cs| *cs.get(ala_index).expect("Invalid index."))
                .save(&out_dir.join(format!("ala_{}.nc", n)));
            concs
                .map(|cs| *cs.get(o2_index).expect("Invalid index."))
                .save(&out_dir.join(format!("o2_{}.nc", n)));
            diff::run(6.0, &diff_grid, verse.specs(), &mut concs, &viscs);
        }
        concs
            .map(|cs| *cs.get(ala_index).expect("Invalid index."))
            .save(&out_dir.join(format!("ala_{}.nc", total_steps)));
        concs
            .map(|cs| *cs.get(ala_index).expect("Invalid index."))
            .save(&out_dir.join(format!("o2_{}.nc", total_steps)));

        concs
    };
    for (i, key) in verse.specs().map().keys().enumerate() {
        concs
            .map(|cs| *cs.get(i).expect("Invalid index."))
            .save(&out_dir.join(format!("concs_{}.nc", key)));
    }

    banner::section("MCRT");
    let lm = {
        let mcrt_grid = mcrt::Grid::new(params.res, params.bound, verse.inters(), verse.surfs());

        info!("Saving interface map.");
        mcrt_grid.interfaces().save(&out_dir.join("interfaces.nc"));

        mcrt::run(
            params.num_phot as u64,
            verse.lights().get(&params.light),
            &mcrt_grid,
            verse.surfs(),
            verse.mats(),
        )
    };
    lm.save(&out_dir);

    banner::section("Kinetics");
    let udens_index = verse.specs().index_of_key(&SpecKey::new("udens"));
    concs.map_mut(|cs| *cs.get_mut(udens_index).expect("Invalid index.") += 1.0);

    banner::section("Finished");
}
