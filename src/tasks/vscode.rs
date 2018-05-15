use std::{fs, str};
use std::path::Path;

use toml;

use utils;

const ERROR_MSG: &str = "error: vscode";

const COMMAND: &str = "code";

#[derive(Debug, Deserialize)]
struct Config {
    install: Vec<String>,
    uninstall: Vec<String>,
}

pub fn sync() {
    if !has_code() {
        return;
    }

    println!("pkg: vscode: syncing ...");

    let src = utils::env::home_dir().join(".dotfiles/config/vscode.json");

    #[cfg(target_os = "macos")]
    let settings_path = "Library/Application Support/Code/User/settings.json";
    #[cfg(target_os = "windows")]
    let settings_path = "AppData/Roaming/Code/User/settings.json";
    #[cfg(not(any(target_os = "macos",windows)))]
    let settings_path = ".config/Code/User/settings.json";
    let dest = utils::env::home_dir().join(Path::new(settings_path));

    utils::fs::symbolic_link_if_exists(&src, &dest);

    let cfg_path = utils::env::home_dir().join(".dotfiles/config/vscode.toml");

    let contents = match fs::read_to_string(&cfg_path) {
        Ok(s) => s,
        Err(error) => {
            println!("pkg: vscode: ignoring config: {}", error);
            return;
        }
    };

    let config: Config = toml::from_str(&contents).expect("cannot parse .../vscode.toml");

    let exts = exts_installed();

    for ext in config.install {
        if !exts.contains(&ext) {
            utils::process::command_spawn_wait(COMMAND, &["--install-extension", &ext])
                .expect(ERROR_MSG);
        }
    }

    for ext in config.uninstall {
        if exts.contains(&ext) {
            utils::process::command_spawn_wait(COMMAND, &["--uninstall-extension", &ext])
                .expect(ERROR_MSG);
        }
    }

    #[cfg(target_os = "macos")] fix_macos();
}

pub fn update() {}

fn exts_installed() -> Vec<String> {
    let output = utils::process::command_output(COMMAND, &["--list-extensions"]).expect(ERROR_MSG);
    let stdout = str::from_utf8(&output.stdout).unwrap();

    let mut exts: Vec<String> = Vec::new();

    for line in stdout.lines() {
        let ext = String::from(line);
        if ext.len() >= 1 {
            exts.push(ext);
        }
    }
    return exts;
}

// fix self-update on macOS
// https://github.com/Microsoft/vscode/issues/7426#issuecomment-277737150
#[cfg(target_os = "macos")]
fn fix_macos() {
    let app_dir = Path::new("/Applications/Visual Studio Code.app");
    if utils::fs::is_dir(&app_dir) {
        match utils::process::command_spawn_wait(
            "xattr",
            &[
                "xattr",
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
        Ok(output) => {
            return output.status.success();
        }
        Err(_error) => {
            return false; // Visual Studio Code probably not installed
        }
    }
}