use std::env::consts::OS;

use crate::{
    lib::task::{self, Status, Task},
    utils,
};

pub fn task() -> Task {
    Task {
        name: String::from("bashprofile"),
        sync,
        ..Default::default()
    }
}

fn sync() -> task::Result {
    if OS == "windows" {
        return Ok(Status::Skipped);
    }

    // https://scriptingosx.com/2017/04/about-bash_profile-and-bashrc-on-macos/
    utils::fs::symbolic_link_if_exists(
        utils::env::home_dir().join(".dotfiles/config/bash_profile"),
        utils::env::home_dir().join(".bash_profile"),
    )
}
