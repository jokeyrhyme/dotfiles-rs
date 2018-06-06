use std::fs;

use mktemp;
use toml;

use utils::{self, golang::{arch, os}};

#[derive(Debug, Deserialize)]
struct Config {
    install: Vec<String>,
}

impl Config {
    fn new() -> Config {
        Config {
            install: Vec::<String>::new(),
        }
    }
}

pub fn sync () {
    println!("pkg: golang: syncing ...");

    if !utils::golang::is_installed() {
        let latest_version = match utils::golang::latest_version() {
            Ok(v) => v,
            Err(error) => {
                println!("error: pkg: golang: unable to check for updates: {:?}", error);
                return;
            }
        };

        match install_golang(&latest_version) {
            Ok(_) => {}
            Err(error) => {
                println!("error: pkg: golang: unable to install: {:?}", error);
                return;
            }
        };
    }

    if utils::golang::is_installed() {
        let config = read_config();

        let mut install_args = vec![String::from("get"), String::from("-v")];
        install_args.extend(config.install);

        match utils::process::command_spawn_wait("go", &install_args) {
            Ok(_status) => {}
            Err(error) => {
                println!("warning: pkg: golang: unable to install packages: {}", error)
            }
        };
    }
}

pub fn update () {
    if !utils::golang::is_installed() {
        return;
    }

    println!("pkg: golang: updating ...");

    let current_version = utils::golang::current_version();
    let latest_version = match utils::golang::latest_version() {
        Ok(v) => v,
        Err(error) => {
            println!("error: pkg: golang: unable to check for updates: {:?}", error);
            return;
        }
    };

    println!("current={} latest={}", &current_version, &latest_version);
    if current_version != latest_version {
        match install_golang(&latest_version) {
            Ok(_) => {}
            Err(error) => {
                println!("error: pkg: golang: unable to install: {:?}", error);
                return;
            }
        };
    }

    let config = read_config();

    let mut install_args = vec![String::from("get"), String::from("-u"), String::from("-v")];
    install_args.extend(config.install);

    match utils::process::command_spawn_wait("go", &install_args) {
        Ok(_status) => {}
        Err(error) => {
            println!("warning: pkg: golang: unable to update packages: {}", error)
        }
    };
}

fn install_golang(version: &str) -> Result<(), utils::golang::GolangError> {
    println!("pkg: golang: installing {} ...", &version);

    let temp_path;
    {
        let mut temp = mktemp::Temp::new_file()?;
        temp_path = temp.to_path_buf();
        temp.release();
    }

    #[cfg(windows)]
    let remote_url = format!("https://dl.google.com/go/{}.{}-{}.zip", version, os(), arch());
    #[cfg(not(windows))]
    let remote_url = format!("https://dl.google.com/go/{}.{}-{}.tar.gz", version, os(), arch());

    utils::http::download(&remote_url, &temp_path)?;

    let local_path = utils::env::home_dir().join(".local");
    let target_path = local_path.join("go");
    utils::fs::delete_if_exists(&target_path);

    #[cfg(windows)]
    utils::archive::extract_zip(&temp_path, &local_path)?;
    #[cfg(not(windows))]
    utils::archive::extract_tar_gz(&temp_path, &local_path)?;

    utils::fs::delete_if_exists(&temp_path);

    Ok(())
}

fn read_config() -> Config {
    let cfg_path = utils::env::home_dir().join(".dotfiles/config/golang.toml");

    let contents = match fs::read_to_string(&cfg_path) {
        Ok(s) => s,
        Err(error) => {
            println!("pkg: golang: ignoring config: {}", error);
            return Config::new();
        }
    };

    match toml::from_str(&contents) {
        Ok(c) => c,
        Err(error) => {
            println!("warning: pkg: golang: unable to parse {}, {}", &cfg_path.display(), error);
            Config::new()
        }
    }
}