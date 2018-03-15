use std::path::Path;

use utils;

pub fn sync() {
    let target = utils::env::home_dir().join(Path::new(".dotfiles"));

    if utils::git::has_git() && utils::git::path_is_git_repository(&target) {
        utils::git::pull(&target);
    }
}

pub fn update() {}