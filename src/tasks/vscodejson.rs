use std::{env::consts::OS, path::Path};

use crate::{
    lib::task::{self, Status, Task},
    utils,
};

pub fn task() -> Task {
    Task {
        name: String::from("vscodejson"),
        sync,
        ..Default::default()
    }
}

fn has_code() -> bool {
    match utils::process::command_output("code", &["--version"]) {
        Ok(output) => output.status.success(),
        Err(_error) => {
            false // Visual Studio Code probably not installed
        }
    }
}

fn sync() -> task::Result {
    if !has_code() {
        return Ok(Status::Skipped);
    }

    let src = utils::env::home_dir().join(".dotfiles/config/vscode.json");

    let settings_path = match OS {
        "macos" => "Library/Application Support/Code/User/settings.json",
        "windows" => "AppData/Roaming/Code/User/settings.json",
        _ => ".config/Code/User/settings.json",
    };
    let dest = utils::env::home_dir().join(Path::new(settings_path));

    utils::fs::symbolic_link_if_exists(&src, &dest)
}
