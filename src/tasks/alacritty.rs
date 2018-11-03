use lib::task::{self, Status, Task};
use utils;

pub fn task() -> Task {
    Task { sync, update }
}

fn sync() -> task::Result {
    println!("alacritty: syncing ...");

    let src = utils::env::home_dir().join(".dotfiles/config/alacritty.yml");
    let dest = utils::env::home_dir().join(".config/alacritty/alacritty.yml");

    utils::fs::symbolic_link_if_exists(&src, &dest);
    Ok(Status::Done)
}

fn update() -> task::Result {
    Ok(Status::NotImplemented)
}
