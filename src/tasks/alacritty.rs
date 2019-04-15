use crate::lib::task::{self, Task};
use crate::utils;

pub fn task() -> Task {
    Task {
        name: String::from("alacritty"),
        sync,
        ..Default::default()
    }
}

fn sync() -> task::Result {
    let src = utils::env::home_dir().join(".dotfiles/config/alacritty.yml");
    let dest = utils::env::home_dir().join(".config/alacritty/alacritty.yml");

    utils::fs::symbolic_link_if_exists(&src, &dest)
}
