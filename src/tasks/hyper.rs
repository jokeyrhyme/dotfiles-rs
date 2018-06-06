use utils;

pub fn sync() {
    println!("hyper: syncing ...");

    let src = utils::env::home_dir().join(".dotfiles/config/hyper.js");
    let dest = utils::env::home_dir().join(".hyper.js");

    utils::fs::symbolic_link_if_exists(&src, &dest);
}

pub fn update() {}
