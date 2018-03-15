use std::path::Path;

use utils;

pub fn sync() {
    let src = utils::env::home_dir().join(Path::new(".dotfiles/config/alacritty.yml"));
    let dest = utils::env::home_dir().join(Path::new(".config/alacritty/alacritty.yml"));

    utils::fs::symbolic_link_if_exists(&src, &dest);
}

pub fn update() {}