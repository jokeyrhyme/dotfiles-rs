use std::env::consts::OS;

use crate::{
    lib::task::{self, Status, Task},
    utils,
};

pub fn task() -> Task {
    Task {
        name: String::from("zshlogin"),
        sync,
        ..Default::default()
    }
}

fn sync() -> task::Result {
    if OS == "windows" {
        return Ok(Status::Skipped);
    }

    // http://zsh.sourceforge.net/Doc/Release/Files.html
    utils::fs::symbolic_link_if_exists(
        utils::env::home_dir().join(".dotfiles/config/zlogin"),
        utils::env::home_dir().join(".zlogin"),
    )
}
