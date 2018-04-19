use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::str;

use serde_json;
use toml;

use utils;

const ERROR_MSG: &str = "error: nodejs";

#[derive(Debug, Deserialize)]
struct Globals {
    #[serde(default)]
    dependencies: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct Config {
    install: Vec<String>,
    uninstall: Vec<String>,
}

pub fn sync() {
    if !utils::nodejs::has_npm() {
        return;
    }

    println!("pkg: nodejs: syncing ...");

    let cfg_path = utils::env::home_dir().join(Path::new(".dotfiles/config/nodejs.toml"));

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

    println!("pkg: nodejs: updating ...");

    println!(
        "pkg: nodes: latest nodejs version: {}",
        utils::nodejs::latest_version()
    );
    // TODO: compare to current nodejs version
    // TODO: download to temporary directory
    // TODO: windows: unzip/un-7zip to ~/.local/node/
    // TODO: not-windows: gunzip+untar to ~/.local/node/
    // TODO: delete temporary file(s)

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
    let output = utils::process::command_output("npm", &["ls", "--global", "--depth=0", "--json"])
        .expect(ERROR_MSG);
    let stdout = str::from_utf8(&output.stdout).unwrap();

    let globals: Globals = match serde_json::from_str(stdout) {
        Ok(globals) => globals,
        Err(error) => {
            println!("unable to parse JSON from `npm`: {:?}", error);
            return Vec::<String>::new();
        }
    };

    let mut pkgs: Vec<String> = Vec::new();

    for pair in globals.dependencies {
        pkgs.push(pair.0);
    }

    return pkgs;
}