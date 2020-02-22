//! Main binary.

use arc::{
    args,
    file::{Load, Save, Verse as VerseForm},
    geom::Aabb,
    ord::{LightKey, SpecKey},
    report,
    sim::{diff, kin, mcrt},
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

    banner::section("MCRT");
    let lm = {
        let mcrt_grid = mcrt::Grid::new(
            params.res,
            params.bound.clone(),
            verse.inters(),
            verse.surfs(),
        );

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

        let total_steps = 20;

        for (i, key) in verse.specs().map().keys().enumerate() {
            concs
                .map(|cs| *cs.get(i).expect("Invalid index."))
                .save(&out_dir.join(format!("diff_{}_{}.nc", key, 0)));
        }
        for n in 0..total_steps {
            println!("n: {}/{}", n, total_steps);
            diff::run(60.0, &diff_grid, verse.specs(), &mut concs, &viscs);

            for (i, key) in verse.specs().map().keys().enumerate() {
                concs
                    .map(|cs| *cs.get(i).expect("Invalid index."))
                    .save(&out_dir.join(format!("diff_{}_{}.nc", key, n + 1)));
            }
        }

        concs
    };

    let udens_index = verse.specs().index_of_key(&SpecKey::new("udens"));
    for (cs, abs_dens) in concs.iter_mut().zip(&lm.abs_dens()) {
        *cs.get_mut(udens_index).expect("Invalid index.") += abs_dens / 186470120150714370.0;
    }

    // for (i, key) in verse.specs().map().keys().enumerate() {
    //     concs.map_mut(|cs| *cs.get_mut(i).expect("Invalid index.") += 1.0e-9);
    //     let kns = concs.map(|cs| *cs.get(i).expect("Invalid index."));
    //     println!("react {}: {}", key, kns.sum());
    // }

    banner::section("Kinetics");
    let reactor = kin::Reactor::new(verse.reacts(), verse.specs());
    println!("Reactor: {:#?}", reactor);
    let total_steps = 100;
    for (i, key) in verse.specs().map().keys().enumerate() {
        let kns = concs.map(|cs| *cs.get(i).expect("Invalid index."));
        println!("key: {}", kns.sum());
        kns.save(&out_dir.join(format!("kin_{}_{}.nc", key, "pre")));
    }
    for k in 0..total_steps {
        println!("k: {}/{}", k, total_steps);
        for mut cs in concs.iter_mut() {
            if cs.sum() > 0.0 {
                // println!("cs: {:#?}", cs);
                kin::run_with_reactor(&kin::Settings::new(0.1, 0.1, 1.0e-6), &reactor, &mut cs);
            }
        }

        for (i, key) in verse.specs().map().keys().enumerate() {
            let kns = concs.map(|cs| *cs.get(i).expect("Invalid index."));
            println!("key: {}", kns.sum());
            kns.save(&out_dir.join(format!("kin_{}_{}.nc", key, k)));
        }
    }
    // for concs in concs.iter_mut() {}

    banner::section("Finished");
}
