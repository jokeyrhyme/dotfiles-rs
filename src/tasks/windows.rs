use std;
use std::path::Path;

use utils;

pub fn sync() {
    println!("pkg: windows: syncing ...");

    let bin = utils::env::home_dir().join(Path::new(".local/bin"));
    std::fs::create_dir_all(&bin).expect(&format!(
        "unable to create directories {}",
        &bin.display()
    ).as_str());
    println!("{:?}", &bin);

    let search_paths = utils::env::path_dirs();
    println!("{:?}", &search_paths);

    if !search_paths.contains(&bin) {
        panic!("%PATH% does not include {:?}, set this first!", &bin);
    }
}

pub fn update() {}