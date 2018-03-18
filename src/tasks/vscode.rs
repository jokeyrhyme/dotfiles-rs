use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::str;

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

    let src = utils::env::home_dir().join(Path::new(".dotfiles/config/vscode.json"));

    #[cfg(target_os = "macos")]
    let settings_path = "Library/Application Support/Code/User/settings.json";
    #[cfg(target_os = "windows")]
    let settings_path = "AppData/Roaming/Code/User/settings.json";
    #[cfg(not(any(target_os = "macos",windows)))]
    let settings_path = ".config/Code/User/settings.json";
    let dest = utils::env::home_dir().join(Path::new(settings_path));

    utils::fs::symbolic_link_if_exists(&src, &dest);

    let cfg_path = utils::env::home_dir().join(Path::new(".dotfiles/config/vscode.toml"));

    let file = match File::open(cfg_path) {
        Ok(file) => file,
        Err(_error) => {
            // probably doesn't exist
            return;
        }
    };
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect(
        "cannot read .../vscode.toml",
    );

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