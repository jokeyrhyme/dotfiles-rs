use crate::lib::task::{self, Task};
use crate::utils;

pub fn task() -> Task {
    Task {
        name: String::from("hyper"),
        sync,
        ..Default::default()
    }
}

fn sync() -> task::Result {
    let src = utils::env::home_dir().join(".dotfiles/config/hyper.js");
    let dest = utils::env::home_dir().join(".hyper.js");

    utils::fs::symbolic_link_if_exists(&src, &dest)
}
