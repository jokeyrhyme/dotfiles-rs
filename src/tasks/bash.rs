use std::env::consts::OS;

use crate::{
    lib::task::{self, Status, Task},
    utils,
};

pub fn task() -> Task {
    Task {
        name: String::from("bash"),
        sync,
        update,
    }
}

fn has_bash() -> bool {
    match utils::process::command_output("bash", &["--version"]) {
        Ok(output) => output.status.success(),
        Err(_error) => {
            false // bash probably not installed
        }
    }
}

fn sync() -> task::Result {
    if OS == "windows" || !has_bash() {
        return Ok(Status::Skipped);
    }

    let it_path = utils::env::home_dir().join(".bash_it");
    if !utils::git::path_is_git_repository(&it_path) {
        utils::fs::delete_if_exists(&it_path);
        let it_url = "https://github.com/Bash-it/bash-it.git";
        match utils::git::shallow_clone(it_url, &it_path.to_string_lossy()) {
            Ok(()) => {}
            Err(error) => println!("bash: unable to install bash-it: {}", error),
        }
    }

    match utils::process::command_spawn_wait(
        "bash",
        &[
            utils::env::home_dir()
                .join(".bash_it/install.sh")
                .to_string_lossy()
                .as_ref(),
            "--silent",
            "--no-modify-config",
        ],
    ) {
        Ok(_status) => {}
        Err(error) => println!("bash: unable to run bash-it installer: {}", error),
    }

    Ok(Status::Done)
}

fn update(_: Status) -> task::Result {
    if OS == "windows" || !has_bash() {
        return Ok(Status::Skipped);
    }

    let it_path = utils::env::home_dir().join(".bash_it");
    if utils::git::path_is_git_repository(&it_path) {
        match utils::git::shallow_fetch(it_path.to_string_lossy()) {
            Ok(()) => {}
            Err(error) => println!("bash: unable to update bash-it: {}", error),
        }
    }

    Ok(Status::Done)
}
