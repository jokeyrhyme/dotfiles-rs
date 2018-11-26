use std::{fs, io, str};

use toml;

use crate::lib::{
    self,
    task::{self, Status, Task},
};
use crate::utils;

const ERROR_MSG: &str = "error: atom";

const COMMAND: &str = "apm";

#[derive(Debug, Deserialize)]
struct Config {
    disable: Vec<String>,
    install: Vec<String>,
    uninstall: Vec<String>,
}

pub fn task() -> Task {
    Task {
        name: "atom".to_string(),
        sync,
        update,
    }
}

fn configure_apm() -> io::Result<()> {
    match lib::python::which_v2() {
        Some(exe) => {
            utils::process::command_spawn_wait(
                "apm",
                &["config", "set", "python", exe.to_str().unwrap_or_default()],
            )?;
        }
        None => {
            utils::process::command_spawn_wait("apm", &["config", "delete", "python"])?;
        }
    };
    Ok(())
}

fn has_apm() -> bool {
    match utils::process::command_output("apm", &["--version"]) {
        Ok(output) => output.status.success(),
        Err(_error) => {
            false // Atom probably not installed
        }
    }
}

fn pkgs_installed() -> Vec<String> {
    let output = utils::process::command_output(COMMAND, &["ls", "--bare"]).expect(ERROR_MSG);
    let stdout = str::from_utf8(&output.stdout).unwrap();

    let mut pkgs: Vec<String> = Vec::new();

    // assume no scoped packages for now:
    // https://github.com/atom/apm/issues/766

    for line in stdout.lines() {
        let pkg = line;
        if !pkg.is_empty() {
            let name = pkg.split('@').next().unwrap();
            pkgs.push(String::from(name));
        }
    }
    pkgs
}

fn sync() -> task::Result {
    if !has_apm() {
        return Ok(Status::Skipped);
    }

    match configure_apm() {
        Ok(_) => {}
        Err(error) => {
            println!("warning: atom: unable to configure apm: {}", error);
        }
    };

    let cfg_path = utils::env::home_dir().join(".dotfiles/config/atom.toml");

    let contents = match fs::read_to_string(&cfg_path) {
        Ok(s) => s,
        Err(error) => {
            println!("atom: ignoring config: {}", error);
            return Ok(Status::Skipped);
        }
    };

    let config: Config = toml::from_str(&contents).expect("cannot parse .../atom.toml");

    let pkgs = pkgs_installed();

    for ext in config.install {
        if !pkgs.contains(&ext) {
            utils::process::command_spawn_wait(
                COMMAND,
                &["install", "--compatible", "--production", "--quiet", &ext],
            ).expect(ERROR_MSG);
        }
    }

    let mut disable_args = vec![String::from("disable")];
    disable_args.extend(config.disable);

    utils::process::command_spawn_wait(COMMAND, &disable_args).expect(ERROR_MSG);

    for ext in config.uninstall {
        if pkgs.contains(&ext) {
            utils::process::command_spawn_wait(COMMAND, &["uninstall", "--hard", &ext])
                .expect(ERROR_MSG);
        }
    }

    Ok(Status::Done)
}

fn update() -> task::Result {
    if !has_apm() {
        return Ok(Status::Skipped);
    }

    utils::process::command_spawn_wait(COMMAND, &["upgrade", "--confirm", "false"])
        .expect(ERROR_MSG);

    Ok(Status::Done)
}
