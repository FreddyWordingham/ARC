use arc::{
    file::{Load, Save},
    util::install,
};

fn main() {
    colog::init();

    let path = install::root().join("input/mcrt/surfaces/tumour.json");
    let mut surf = arc::file::Surface::load(&path);

    let list = [
        "tumour_0125",
        "tumour_0250",
        "tumour_0375",
        "tumour_0500",
        "tumour_0625",
        "tumour_0750",
        "tumour_0875",
        "tumour_1000",
        "tumour_1125",
        "tumour_1250",
        "tumour_1375",
        "tumour_1500",
        "tumour_1625",
        "tumour_1750",
        "tumour_1875",
        "tumour_2000",
    ];

    for (item, next) in list.iter().zip(list.iter().skip(1)) {
        if surf.mesh().str() == *item {
            println!("Found!");
            println!("Replacing with {}", next);
            *surf.mesh_mut() = arc::ord::MeshKey::new(next);
            break;
        }
    }

    surf.save(&path);
}
