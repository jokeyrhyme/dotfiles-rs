use std;
use std::path::Path;

use utils;

pub fn sync() {
    println!("pkg: windows: syncing ...");

    let bin = utils::env::home_dir().join(".local/bin");
    std::fs::create_dir_all(&bin).expect(&format!(
        "unable to create directories {}",
        &bin.display()
    ).as_str());
    println!("{:?}", &bin);

    let search_paths = utils::env::path_dirs();

    if !search_paths.contains(&bin) {
        panic!("%PATH% does not include {:?}, set this first!", &bin);
    }
}

pub fn update() {}
