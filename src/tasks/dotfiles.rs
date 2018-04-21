use utils;

pub fn sync() {
    println!("pkg: dotfiles: syncing ...");

    let target = utils::env::home_dir().join(".dotfiles");

    if utils::git::has_git() && utils::git::path_is_git_repository(&target) {
        utils::git::pull(&target);
    }
}

pub fn update() {}