use std::collections::HashMap;
use std::path::Path;
use std::{fs, io, str};

use serde_derive::Deserialize;
use serde_json;
use toml;
use which;

use crate::lib::{
    self,
    task::{self, Status, Task},
};
use crate::utils;

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

#[derive(Debug, Deserialize)]
struct Package {
    bin: HashMap<String, String>,
    name: String,
    version: String,
}

pub fn task() -> Task {
    Task {
        name: String::from("npm"),
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

fn is_global_package_bin_linked<S>(name: S) -> bool
where
    S: AsRef<str>,
{
    let pkg = match read_global_package(name.as_ref()) {
        Ok(p) => p,
        Err(_) => {
            return false;
        }
    };
    for pair in pkg.bin {
        match which::which(pair.0) {
            Ok(_) => {}
            Err(_) => {
                return false;
            }
        }
    }
    true
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

    globals
        .dependencies
        .keys()
        .cloned()
        .filter(|name| is_global_package_bin_linked(name.as_str()))
        .collect()
}

fn read_config() -> Config {
    let cfg_path = utils::env::home_dir().join(".dotfiles/config/nodejs.toml");

    let contents = match fs::read_to_string(&cfg_path) {
        Ok(s) => s,
        Err(error) => {
            println!("npm: ignoring config: {}", error);
            return Config::new();
        }
    };

    match toml::from_str(&contents) {
        Ok(c) => c,
        Err(error) => {
            println!(
                "warning: npm: unable to parse {}, {}",
                &cfg_path.display(),
                error
            );
            Config::new()
        }
    }
}

fn read_global_package<S>(name: S) -> io::Result<Package>
where
    S: AsRef<str>,
{
    let pkg_path = utils::nodejs::lib_dir()
        .join("node_modules")
        .join(name.as_ref())
        .join("package.json");
    let contents = fs::read_to_string(&pkg_path)?;
    match serde_json::from_str(&contents) {
        Ok(pkg) => Ok(pkg),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
    }
}

fn sync() -> task::Result {
    if !utils::nodejs::has_node() {
        return Ok(Status::Skipped);
    }

    match configure_npm() {
        Ok(_) => {}
        Err(error) => {
            println!("warning: npm: unable to configure npm: {}", error);
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
                println!("warning: npm: unable to bootstrap npm: {}", error);
            }
        };
    }

    let config = read_config();
    let pkgs = pkgs_installed();

    let missing: Vec<String> = config
        .install
        .into_iter()
        .filter(|pkg| !pkgs.contains(&pkg))
        .collect();

    if missing.is_empty() {
        return Ok(Status::Done);
    }

    let mut install_args = vec![String::from("install"), String::from("--global")];
    install_args.extend(missing);

    match utils::process::command_spawn_wait("npm", &install_args) {
        Ok(_status) => {}
        Err(error) => println!(
            "warning: npm: unable to install missing npm packages: {}",
            error
        ),
    };

    let found: Vec<String> = config
        .uninstall
        .into_iter()
        .filter(|pkg| pkgs.contains(&pkg))
        .collect();

    if found.is_empty() {
        return Ok(Status::Done);
    }

    let mut uninstall_args = vec![String::from("uninstall"), String::from("--global")];
    uninstall_args.extend(found);

    match utils::process::command_spawn_wait("npm", &uninstall_args) {
        Ok(_status) => {}
        Err(error) => println!(
            "warning: npm: unable to uninstall unused npm packages: {}",
            error
        ),
    };

    Ok(Status::Done)
}

fn update(_: Status) -> task::Result {
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

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn is_global_package_bin_linked_for_npx() {
        if !utils::nodejs::has_npx() {
            return;
        }
        assert!(is_global_package_bin_linked("npx"));
    }

    #[test]
    fn pkgs_installed_works() {
        let pkgs = pkgs_installed();
        if utils::nodejs::has_npm() {
            assert!(pkgs.contains("npm"));
        }
        if utils::nodejs::has_npx() {
            assert!(pkgs.contains("npx"));
        }
    }

    #[test]
    fn read_global_package_for_npx() {
        if !utils::nodejs::has_npx() {
            return;
        }
        let pkg = read_global_package("npx").expect("must read");
        assert_eq!(pkg.name, "npx");
        assert!(pkg.bin.get("npx") != None);
    }
}
