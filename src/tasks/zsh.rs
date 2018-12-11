use std::env::consts::OS;

use crate::{
    lib::task::{self, Status, Task},
    utils,
};

pub fn task() -> Task {
    Task {
        name: "zsh".to_string(),
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

    utils::fs::symbolic_link_if_exists(
        utils::env::home_dir().join(".dotfiles/config/profile"),
        utils::env::home_dir().join(".profile"),
    )?;
    utils::fs::symbolic_link_if_exists(
        utils::env::home_dir().join(".dotfiles/config/zshrc"),
        utils::env::home_dir().join(".zshrc"),
    )?;

    let oh_path = utils::env::home_dir().join(".oh-my-zsh");
    if !utils::git::path_is_git_repository(&oh_path) {
        utils::fs::delete_if_exists(&oh_path);
        let oh_url = "https://github.com/robbyrussell/oh-my-zsh.git";
        match utils::git::shallow_clone(oh_url, &oh_path.to_string_lossy()) {
            Ok(()) => {}
            Err(error) => println!("zsh: unable to install oh-my-zsh: {}", error),
        }
    }

    let pure_path = utils::env::home_dir().join(".zsh-pure");
    if !utils::git::path_is_git_repository(&pure_path) {
        utils::fs::delete_if_exists(&pure_path);
        let pure_url = "https://github.com/sindresorhus/pure.git";
        match utils::git::shallow_clone(pure_url, &pure_path.to_string_lossy()) {
            Ok(()) => {}
            Err(error) => println!("zsh: unable to install pure: {}", error),
        }
    }

    utils::fs::symbolic_link_if_exists(
        utils::env::home_dir().join(".zsh-pure/pure.zsh"),
        utils::env::home_dir().join(".oh-my-zsh/custom/pure.zsh-theme"),
    )?;
    utils::fs::symbolic_link_if_exists(
        utils::env::home_dir().join(".zsh-pure/async.zsh"),
        utils::env::home_dir().join(".oh-my-zsh/custom/async.zsh"),
    )?;

    Ok(Status::Done)
}

fn update() -> task::Result {
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

    let pure_path = utils::env::home_dir().join(".zsh-pure");
    if utils::git::path_is_git_repository(&pure_path) {
        match utils::git::shallow_fetch(pure_path.to_string_lossy()) {
            Ok(()) => {}
            Err(error) => println!("zsh: unable to update pure: {}", error),
        }
    }

    Ok(Status::Done)
}
