use std::path::Path;

use utils;

pub fn sync() {
    let mut src = utils::env::home_dir();
    src.push(Path::new(".dotfiles/config/hyper.js"));

    let mut dest = utils::env::home_dir();
    dest.push(Path::new(".hyper.js"));

    utils::fs::symbolic_link(&src, &dest);
}

pub fn update() {}
