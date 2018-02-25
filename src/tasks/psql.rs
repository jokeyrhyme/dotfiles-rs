use std::path::Path;

use utils;

pub fn sync() {
    let mut src = utils::env::home_dir();
    src.push(Path::new(".dotfiles/config/psqlrc"));

    let mut dest = utils::env::home_dir();
    dest.push(Path::new(".psqlrc"));

    utils::fs::symbolic_link_if_exists(&src, &dest);
}

pub fn update() {}
