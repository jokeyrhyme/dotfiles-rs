use std::env::consts::OS;

use crate::{
    lib::task::{self, Status, Task},
    utils,
};

pub fn task() -> Task {
    Task {
        name: String::from("bashrc"),
        sync,
        ..Default::default()
    }
}

fn sync() -> task::Result {
    if OS == "windows" {
        return Ok(Status::Skipped);
    }

    utils::fs::symbolic_link_if_exists(
        utils::env::home_dir().join(".dotfiles/config/bashrc"),
        utils::env::home_dir().join(".bashrc"),
    )
}
