use std;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::str;

use mktemp;
use serde_json;
use toml;

use utils::{self, nodejs::{arch,os}};

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
    println!("pkg: nodejs: syncing ...");

    if !utils::nodejs::has_node() {
        let latest = utils::nodejs::latest_version();
        install_nodejs(&latest);
    }

    if !utils::nodejs::has_npm() {
        let npm_cli_path = utils::env::home_dir().join(".local/node/lib/node_modules/npm/bin/npm-cli.js");
        let npm_cli_path_string = npm_cli_path.as_os_str().to_string_lossy().into_owned();
        let npm_cli_path_str = npm_cli_path_string.as_str();

        utils::process::command_spawn_wait("node", &[npm_cli_path_str, "install", "--global", "npm"]).expect(ERROR_MSG);
    }

    let cfg_path = utils::env::home_dir().join(".dotfiles/config/nodejs.toml");

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
    if !utils::nodejs::has_node() {
        return;
    }

    println!("pkg: nodejs: updating ...");

    let current = utils::nodejs::current_version();
    let latest = utils::nodejs::latest_version();
    println!("current={} latest={}", &current, &latest);

    if current != latest {
        install_nodejs(&latest);
    }

    if utils::nodejs::has_npx() {
        // https://www.npmjs.com/package/npm-windows-upgrade
        #[cfg(windows)]
        utils::process::command_spawn_wait(
            "npx",
            &["-q", "npm-windows-upgrade", "--npm-version", "latest"],
        ).expect(ERROR_MSG);

        match utils::process::command_spawn_wait("npx", &["-q", "npm", "update", "--global"]) {
            Ok(_status) => {}
            Err(_error) => {
                // private packages will fail on incorrect networks, ignore this
            }
        }
    }
}

fn install_nodejs(version: &str) {
    let temp_path;
    {
        let mut temp = mktemp::Temp::new_file().unwrap();
        temp_path = temp.to_path_buf();
        temp.release();
    }

    let prefix = format!("node-{}-{}-{}", version, os(), arch());

    #[cfg(windows)]
    let remote_url = format!("https://nodejs.org/dist/{}/{}.zip", version, &prefix);
    #[cfg(not(windows))]
    let remote_url = format!("https://nodejs.org/dist/{}/{}.tar.gz", version, &prefix);

    match utils::http::download(&remote_url, &temp_path) {
        Ok(()) => {}
        Err(error) => {
            println!("error: cannot download: {}", error);
            return;
        }
    };

    let local_path = utils::env::home_dir().join(".local");

    // archive contains a directory with name matching `prefix`
    let interim_path = local_path.join(&prefix);
    utils::fs::delete_if_exists(&interim_path);

    #[cfg(windows)]
    utils::archive::extract_zip(&temp_path, &local_path);
    #[cfg(not(windows))]
    utils::archive::extract_tar_gz(&temp_path, &local_path);

    let target_path = local_path.join("node");
    utils::fs::delete_if_exists(&target_path);

    std::fs::rename(&interim_path, &target_path).unwrap();

    utils::fs::delete_if_exists(&temp_path);
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