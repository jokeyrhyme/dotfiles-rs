use crate::lib::task::{self, Task};
use crate::utils;

pub fn task() -> Task {
    Task {
        name: String::from("brewfile"),
        sync,
        ..Default::default()
    }
}

fn sync() -> task::Result {
    let src = utils::env::home_dir().join(".dotfiles/config/Brewfile");
    let dest = utils::env::home_dir().join(".Brewfile");

    utils::fs::symbolic_link_if_exists(&src, &dest)
}
