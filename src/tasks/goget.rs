use std::fs;

use toml;
use which;

use lib::task::{self, Status, Task};
use utils;

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

pub fn task() -> Task {
    Task {
        name: "goget".to_string(),
        sync,
        update,
    }
}

fn read_config() -> Config {
    let cfg_path = utils::env::home_dir().join(".dotfiles/config/golang.toml");

    let contents = match fs::read_to_string(&cfg_path) {
        Ok(s) => s,
        Err(error) => {
            println!("goget: ignoring config: {}", error);
            return Config::new();
        }
    };

    match toml::from_str(&contents) {
        Ok(c) => c,
        Err(error) => {
            println!(
                "warning: goget: unable to parse {}, {}",
                &cfg_path.display(),
                error
            );
            Config::new()
        }
    }
}

fn sync() -> task::Result {
    let home_dir = utils::env::home_dir();

    // uninstall `dep` installed by `go get -u ...`
    // we install `dep` via GitHub Release instead as recommended
    utils::fs::delete_if_exists(&home_dir.join("go").join("bin").join("dep"));
    utils::fs::delete_if_exists(&home_dir.join("go").join("bin").join("dep.exe"));
    utils::fs::delete_if_exists(
        &home_dir
            .join("go")
            .join("pkg")
            .join("src")
            .join("github.com")
            .join("golang")
            .join("dep"),
    );
    utils::fs::delete_if_exists(
        &home_dir
            .join("go")
            .join("src")
            .join("github.com")
            .join("golang")
            .join("dep"),
    );

    if !utils::golang::is_installed() {
        return Ok(Status::Done);
    }

    let config = read_config();

    let mut install_args = vec![String::from("get"), String::from("-v")];
    install_args.extend(config.install);

    if let Err(error) = utils::process::command_spawn_wait("go", &install_args) {
        println!("warning: goget: unable to install packages: {}", error);
    };

    if which::which("gometalinter").is_ok() {
        if let Err(error) = utils::process::command_spawn_wait("gometalinter", &["--install"]) {
            println!("warning: goget: unable to install linters: {}", error);
        };
    };

    Ok(Status::Done)
}

fn update() -> task::Result {
    if !utils::golang::is_installed() {
        return Ok(Status::Skipped);
    }

    let config = read_config();

    let mut install_args = vec![String::from("get"), String::from("-u"), String::from("-v")];
    install_args.extend(config.install);

    if let Err(error) = utils::process::command_spawn_wait("go", &install_args) {
        println!("warning: goget: unable to update packages: {}", error);
    };

    if which::which("gometalinter").is_ok() {
        if let Err(error) =
            utils::process::command_spawn_wait("gometalinter", &["--install", "--force"])
        {
            println!("warning: goget: unable to update linters: {}", error);
        };
    };

    Ok(Status::Changed(
        "unknown".to_string(),
        "unknown".to_string(),
    ))
}
