use std::collections::HashMap;
use std::path::Path;
use std::{self, fs, io, str};

use serde_json;
use toml;

use utils::{
    self, fs::mktemp, nodejs::{arch, os},
};

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

impl Config {
    fn new() -> Config {
        Config {
            install: Vec::<String>::new(),
            uninstall: Vec::<String>::new(),
        }
    }
}

pub fn sync() {
    println!("nodejs: syncing ...");

    if !utils::nodejs::has_node() {
        let latest = utils::nodejs::latest_version();
        match install_nodejs(latest) {
            Ok(()) => {}
            Err(error) => println!("error: nodejs: unable to install Node.js: {:?}", error),
        };
    }

    configure_npm();
    sync_npm_packages();
}

pub fn update() {
    if !utils::nodejs::has_node() {
        return;
    }

    println!("nodejs: updating ...");

    let current = utils::nodejs::current_version();
    let latest = utils::nodejs::latest_version();
    println!("current={} latest={}", &current, &latest);

    if current != latest {
        match install_nodejs(latest) {
            Ok(()) => {}
            Err(error) => println!("error: nodejs: unable to install Node.js: {:?}", error),
        };
        sync_npm_packages();
    }

    if utils::nodejs::has_npx() {
        match utils::process::command_spawn_wait("npx", &["-q", "npm", "update", "--global"]) {
            Ok(_status) => {}
            Err(_error) => {
                // private packages will fail on incorrect networks, ignore this
            }
        }
    }
}

fn configure_npm() {
    match utils::process::command_spawn_wait("npm", &["config", "set", "send-metric", "true"]) {
        Ok(_status) => {}
        Err(error) => {
            println!("warning: nodejs: unable to enable npm metrics: {}", error);
        }
    }
}

fn install_nodejs<S>(version: S) -> io::Result<()>
where
    S: Into<String> + AsRef<str>,
{
    let temp_path = mktemp()?;

    let prefix = format!("node-{}-{}-{}", version.as_ref(), os(), arch());

    #[cfg(windows)]
    let remote_url = format!(
        "https://nodejs.org/dist/{}/{}.zip",
        version.as_ref(),
        &prefix
    );
    #[cfg(not(windows))]
    let remote_url = format!(
        "https://nodejs.org/dist/{}/{}.tar.gz",
        version.as_ref(),
        &prefix
    );

    match utils::http::download(remote_url, &temp_path) {
        Ok(()) => {}
        Err(error) => {
            println!("error: cannot download: {}", error);
            return Err(error);
        }
    };

    let local_path = utils::env::home_dir().join(".local");

    // archive contains a directory with name matching `prefix`
    let interim_path = local_path.join(&prefix);
    utils::fs::delete_if_exists(&interim_path);

    #[cfg(windows)]
    utils::archive::extract_zip(&temp_path, &local_path)?;
    #[cfg(not(windows))]
    utils::archive::extract_tar_gz(&temp_path, &local_path)?;

    let target_path = local_path.join("node");
    utils::fs::delete_if_exists(&target_path);

    std::fs::rename(&interim_path, &target_path)?;

    utils::fs::delete_if_exists(&temp_path);

    Ok(())
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

fn read_config() -> Config {
    let cfg_path = utils::env::home_dir().join(".dotfiles/config/nodejs.toml");

    let contents = match fs::read_to_string(&cfg_path) {
        Ok(s) => s,
        Err(error) => {
            println!("nodejs: ignoring config: {}", error);
            return Config::new();
        }
    };

    match toml::from_str(&contents) {
        Ok(c) => c,
        Err(error) => {
            println!(
                "warning: nodejs: unable to parse {}, {}",
                &cfg_path.display(),
                error
            );
            Config::new()
        }
    }
}

fn sync_npm_packages() {
    // these often are included with the Windows version,
    // and prevent `npm` from updating itself
    for filename in &["npm", "npm.cmd", "npx", "npx.cmd"] {
        utils::fs::delete_if_exists(&utils::nodejs::bin_dir().join(Path::new(&filename)));
    }

    if !utils::nodejs::has_npm() {
        let npm_cli_path = utils::nodejs::lib_dir().join("node_modules/npm/bin/npm-cli.js");
        let npm_cli_path_string = npm_cli_path.as_os_str().to_string_lossy().into_owned();
        let npm_cli_path_str = npm_cli_path_string.as_str();

        match utils::process::command_spawn_wait(
            "node",
            &[npm_cli_path_str, "install", "--global", "npm"],
        ) {
            Ok(_status) => {}
            Err(error) => {
                println!("warning: nodejs: unable to bootstrap npm: {}", error);
            }
        };
    }

    let config = read_config();
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

    match utils::process::command_spawn_wait("npm", &install_args) {
        Ok(_status) => {}
        Err(error) => println!(
            "warning: nodejs: unable to install missing npm packages: {}",
            error
        ),
    };

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

    match utils::process::command_spawn_wait("npm", &uninstall_args) {
        Ok(_status) => {}
        Err(error) => println!(
            "warning: nodejs: unable to uninstall unused npm packages: {}",
            error
        ),
    };
}
