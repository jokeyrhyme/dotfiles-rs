use lib::task::{self, Status, Task};
use utils;

pub fn task() -> Task {
    Task {
        name: "alacritty".to_string(),
        sync,
        update,
    }
}

fn sync() -> task::Result {
    let src = utils::env::home_dir().join(".dotfiles/config/alacritty.yml");
    let dest = utils::env::home_dir().join(".config/alacritty/alacritty.yml");

    utils::fs::symbolic_link_if_exists(&src, &dest)
}

fn update() -> task::Result {
    Ok(Status::NotImplemented)
}
