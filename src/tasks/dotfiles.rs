use crate::lib::task::{self, Status, Task};
use crate::utils;

pub fn task() -> Task {
    Task {
        name: String::from("dotfiles"),
        sync,
        ..Default::default()
    }
}

fn sync() -> task::Result {
    let target = utils::env::home_dir().join(".dotfiles");

    if utils::git::has() && utils::git::path_is_git_repository(&target) {
        utils::git::pull(&target);
    }
    Ok(Status::Done)
}
