use crate::lib::task::{self, Status, Task};
use crate::utils;

pub fn task() -> Task {
    Task {
        name: String::from("dotfiles"),
        sync,
        update,
    }
}

fn sync() -> task::Result {
    let target = utils::env::home_dir().join(".dotfiles");

    if utils::git::has_git() && utils::git::path_is_git_repository(&target) {
        utils::git::pull(&target);
    }
    Ok(Status::Done)
}

fn update() -> task::Result {
    Ok(Status::NotImplemented)
}
