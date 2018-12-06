use std::{env::consts::OS, fs, path::Path, str};

use toml;

use crate::{
    lib::task::{self, Status, Task},
    utils,
};

const COMMAND: &str = "code";

#[derive(Debug, Deserialize)]
struct Config {
    install: Vec<String>,
    uninstall: Vec<String>,
}

pub fn task() -> Task {
    Task {
        name: "vscode".to_string(),
        sync,
        update,
    }
}

fn exts_installed() -> Vec<String> {
    let output = utils::process::command_output(COMMAND, &["--list-extensions"])
        .expect("vscode: error: unable to list extensions");
    let stdout = str::from_utf8(&output.stdout).unwrap();

    let mut exts: Vec<String> = Vec::new();

    for line in stdout.lines() {
        let ext = String::from(line);
        if !ext.is_empty() {
            exts.push(ext);
        }
    }
    exts
}

// fix self-update on macOS
// https://github.com/Microsoft/vscode/issues/7426#issuecomment-277737150
#[cfg(target_os = "macos")]
fn fix_macos() {
    let app_dir = Path::new("/Applications/Visual Studio Code.app");
    if app_dir.is_dir() {
        match utils::process::command_spawn_wait(
            "xattr",
            &[
                "-dr",
                "com.apple.quarantine",
                "/Applications/Visual Studio Code.app",
            ],
        ) {
            Ok(_status) => {}
            Err(_error) => {}
        }
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

    utils::fs::symbolic_link_if_exists(&src, &dest)?;

    let cfg_path = utils::env::home_dir().join(".dotfiles/config/vscode.toml");

    let contents = match fs::read_to_string(&cfg_path) {
        Ok(s) => s,
        Err(error) => {
            return Err(task::Error::IOError("ignoring config".to_string(), error));
        }
    };

    let config: Config = toml::from_str(&contents).expect("cannot parse .../vscode.toml");

    let exts = exts_installed();

    for ext in config.install {
        if !exts.contains(&ext) {
            utils::process::command_spawn_wait(COMMAND, &["--install-extension", &ext])?;
        }
    }

    for ext in config.uninstall {
        if exts.contains(&ext) {
            utils::process::command_spawn_wait(COMMAND, &["--uninstall-extension", &ext])?;
        }
    }

    #[cfg(target_os = "macos")]
    fix_macos();

    Ok(Status::Done)
}

fn update() -> task::Result {
    Ok(Status::NotImplemented)
}
