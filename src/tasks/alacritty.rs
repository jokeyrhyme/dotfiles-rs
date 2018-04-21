use utils;

pub fn sync() {
    println!("pkg: alacritty: syncing ...");

    let src = utils::env::home_dir().join(".dotfiles/config/alacritty.yml");
    let dest = utils::env::home_dir().join(".config/alacritty/alacritty.yml");

    utils::fs::symbolic_link_if_exists(&src, &dest);
}

pub fn update() {}