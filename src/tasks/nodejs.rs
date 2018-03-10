use std;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

use toml;

use utils;

const ERROR_MSG: &str = "error: nodejs";

#[derive(Debug, Deserialize)]
struct Config {
    install: Vec<String>,
    uninstall: Vec<String>,
}

pub fn sync() {
    if !utils::nodejs::has_npm() {
        return;
    }

    let mut cfg_path = utils::env::home_dir();
    cfg_path.push(Path::new(".dotfiles/config/nodejs.toml"));

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
        "cannot read .../nodejs.toml",
    );

    let config: Config = toml::from_str(&contents).expect("cannot parse .../nodejs.toml");

    let pkgs = pkgs_installed();

    let missing: Vec<String> = config
        .install
        .into_iter()
        .filter_map(|pkg| {
            if pkgs.contains(&pkg) {
                return None;
            }
            return Some(String::from(pkg));
        })
        .collect();

    if missing.len() <= 0 {
        return; // nothing to do
    }

    let mut install_args = vec![String::from("install"), String::from("--global")];
    install_args.extend(missing);

    utils::process::command_spawn_wait("npm", &install_args).expect(ERROR_MSG);

    let found: Vec<String> = config
        .uninstall
        .into_iter()
        .filter_map(|pkg| {
            if pkgs.contains(&pkg) {
                return Some(String::from(pkg));
            }
            return None;
        })
        .collect();

    if found.len() <= 0 {
        return; // nothing to do
    }

    let mut uninstall_args = vec![String::from("uninstall"), String::from("--global")];
    uninstall_args.extend(found);

    utils::process::command_spawn_wait("npm", &uninstall_args).expect(ERROR_MSG);
}

pub fn update() {
    if !utils::nodejs::has_npm() {
        return;
    }

    if utils::nodejs::has_npx() {
        // https://www.npmjs.com/package/npm-windows-upgrade
        #[cfg(windows)]
        utils::process::command_spawn_wait(
            "npx",
            &["-q", "npm-windows-upgrade", "--npm-version", "latest"],
        ).expect(ERROR_MSG);
    }

    match utils::process::command_spawn_wait("npx", &["-q", "npm", "update", "--global"]) {
        Ok(_status) => {}
        Err(_error) => {
            // private packages will fail on incorrect networks, ignore this
        }
    }
}

fn pkgs_installed() -> Vec<String> {
    let output = utils::process::command_output("npm", &["ls", "--global", "--depth=0"])
        .expect(ERROR_MSG);
    let stdout = std::str::from_utf8(&output.stdout).unwrap();

    let mut pkgs: Vec<String> = Vec::new();

    for line in stdout.lines() {
        let pkg = String::from(line);
        if pkg.len() >= 1 && !pkg.contains("(empty)") && !pkg.contains(r"\npm") {
            pkgs.push(pkg);
        }
    }
    return pkgs;
}
