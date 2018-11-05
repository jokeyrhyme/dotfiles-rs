use std::collections::HashMap;
use std::path::Path;
use std::{fs, io, str};

use serde_json;
use toml;

use lib::{
    self,
    task::{self, Status, Task},
};
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

impl Config {
    fn new() -> Config {
        Config {
            install: Vec::<String>::new(),
            uninstall: Vec::<String>::new(),
        }
    }
}

pub fn task() -> Task {
    Task {
        name: "npm".to_string(),
        sync,
        update,
    }
}

fn configure_npm() -> io::Result<()> {
    match lib::python::which_v2() {
        Some(exe) => {
            utils::process::command_spawn_wait(
                "npm",
                &["config", "set", "python", exe.to_str().unwrap_or_default()],
            )?;
        }
        None => {
            utils::process::command_spawn_wait("npm", &["config", "delete", "python"])?;
        }
    };
    utils::process::command_spawn_wait("npm", &["config", "set", "send-metric", "true"])?;
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
    pkgs
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

fn sync() -> task::Result {
    if !utils::nodejs::has_node() {
        return Ok(Status::Skipped);
    }

    match configure_npm() {
        Ok(_) => {}
        Err(error) => {
            println!("warning: nodejs: unable to configure npm: {}", error);
        }
    };

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
            Some(pkg)
        }).collect();

    if missing.is_empty() {
        return Ok(Status::Done);
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
                return Some(pkg);
            }
            None
        }).collect();

    if found.is_empty() {
        return Ok(Status::Done);
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

    Ok(Status::Done)
}

fn update() -> task::Result {
    if !utils::nodejs::has_node() {
        return Ok(Status::Skipped);
    }

    if utils::nodejs::has_npx() {
        match utils::process::command_spawn_wait("npx", &["-q", "npm", "update", "--global"]) {
            Ok(_status) => {}
            Err(_error) => {
                // private packages will fail on incorrect networks, ignore this
            }
        }
    }

    Ok(Status::Done)
}
