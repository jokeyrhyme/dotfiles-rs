use std::env::consts::OS;

use crate::{
    lib::task::{self, Status, Task},
    utils,
};

pub fn task() -> Task {
    Task {
        name: String::from("zsh"),
        sync,
        update,
    }
}

fn has_zsh() -> bool {
    match utils::process::command_output("zsh", &["--version"]) {
        Ok(output) => output.status.success(),
        Err(_error) => {
            false // zsh probably not installed
        }
    }
}

fn sync() -> task::Result {
    if OS == "windows" || !has_zsh() {
        return Ok(Status::Skipped);
    }

    let oh_path = utils::env::home_dir().join(".oh-my-zsh");
    if !utils::git::path_is_git_repository(&oh_path) {
        utils::fs::delete_if_exists(&oh_path);
        let oh_url = "https://github.com/robbyrussell/oh-my-zsh.git";
        match utils::git::shallow_clone(oh_url, &oh_path.to_string_lossy()) {
            Ok(()) => {}
            Err(error) => println!("zsh: unable to install oh-my-zsh: {}", error),
        }
    }

    utils::fs::delete_if_exists(utils::env::home_dir().join(".zsh-pure"));
    utils::fs::delete_if_exists(utils::env::home_dir().join(".oh-my-zsh/custom/pure.zsh-theme"));
    utils::fs::delete_if_exists(utils::env::home_dir().join(".oh-my-zsh/custom/async.zsh"));

    Ok(Status::Done)
}

fn update(_: Status) -> task::Result {
    if OS == "windows" || !has_zsh() {
        return Ok(Status::Skipped);
    }

    let oh_path = utils::env::home_dir().join(".oh-my-zsh");
    if utils::git::path_is_git_repository(&oh_path) {
        match utils::git::shallow_fetch(oh_path.to_string_lossy()) {
            Ok(()) => {}
            Err(error) => println!("zsh: unable to update oh-my-zsh: {}", error),
        }
    }

    Ok(Status::Done)
}
