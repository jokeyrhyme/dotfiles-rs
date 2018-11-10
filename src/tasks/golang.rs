use std::fs;

use toml;
use which;

use lib::task::{self, Status, Task};
use utils::{
    self,
    fs::mktemp,
    golang::{arch, os},
};

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
        name: "golang".to_string(),
        sync,
        update,
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn install_golang<S>(version: S) -> task::Result
where
    S: Into<String> + AsRef<str>,
{
    println!("golang: installing {} ...", version.as_ref());

    let temp_path = mktemp()?;

    #[cfg(windows)]
    let remote_url = format!(
        "https://dl.google.com/go/{}.{}-{}.zip",
        version.as_ref(),
        os(),
        arch()
    );
    #[cfg(not(windows))]
    let remote_url = format!(
        "https://dl.google.com/go/{}.{}-{}.tar.gz",
        version.as_ref(),
        os(),
        arch()
    );

    utils::http::download(remote_url, &temp_path)?;

    let local_path = utils::env::home_dir().join(".local");
    let target_path = local_path.join("go");
    utils::fs::delete_if_exists(&target_path);

    #[cfg(windows)]
    utils::archive::extract_zip(&temp_path, &local_path)?;
    #[cfg(not(windows))]
    utils::archive::extract_tar_gz(&temp_path, &local_path)?;

    utils::fs::delete_if_exists(&temp_path);

    Ok(Status::Changed(
        "unknown".to_string(),
        version.as_ref().to_string(),
    ))
}

fn read_config() -> Config {
    let cfg_path = utils::env::home_dir().join(".dotfiles/config/golang.toml");

    let contents = match fs::read_to_string(&cfg_path) {
        Ok(s) => s,
        Err(error) => {
            println!("golang: ignoring config: {}", error);
            return Config::new();
        }
    };

    match toml::from_str(&contents) {
        Ok(c) => c,
        Err(error) => {
            println!(
                "warning: golang: unable to parse {}, {}",
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
        let latest_version = match utils::golang::latest_version() {
            Ok(v) => v,
            Err(error) => {
                return Err(error);
            }
        };

        if let Err(error) = install_golang(latest_version) {
            return Err(error);
        };
    }

    if utils::golang::is_installed() {
        let config = read_config();

        let mut install_args = vec![String::from("get"), String::from("-v")];
        install_args.extend(config.install);

        if let Err(error) = utils::process::command_spawn_wait("go", &install_args) {
            println!("warning: golang: unable to install packages: {}", error);
        };
    }

    if which::which("gometalinter").is_ok() {
        if let Err(error) = utils::process::command_spawn_wait("gometalinter", &["--install"]) {
            println!("warning: golang: unable to install linters: {}", error);
        };
    };

    Ok(Status::Done)
}

fn update() -> task::Result {
    if !utils::golang::is_installed() {
        return Ok(Status::Skipped);
    }

    println!("golang: updating ...");

    let current_version = utils::golang::current_version();
    let latest_version = match utils::golang::latest_version() {
        Ok(v) => v,
        Err(error) => {
            return Err(error);
        }
    };

    println!("current={} latest={}", &current_version, &latest_version);
    if current_version != latest_version {
        if let Err(error) = install_golang(latest_version.clone()) {
            return Err(error);
        };
    }

    let config = read_config();

    let mut install_args = vec![String::from("get"), String::from("-u"), String::from("-v")];
    install_args.extend(config.install);

    if let Err(error) = utils::process::command_spawn_wait("go", &install_args) {
        println!("warning: golang: unable to update packages: {}", error);
    };

    if which::which("gometalinter").is_ok() {
        if let Err(error) =
            utils::process::command_spawn_wait("gometalinter", &["--install", "--force"])
        {
            println!("warning: golang: unable to update linters: {}", error);
        };
    };

    Ok(Status::Changed(current_version, latest_version))
}
