use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::str;

use toml;

use utils;

const ERROR_MSG: &str = "error: atom";

const COMMAND: &str = "apm";

#[derive(Debug, Deserialize)]
struct Config {
    disable: Vec<String>,
    install: Vec<String>,
    uninstall: Vec<String>,
}

pub fn sync() {
    if !has_apm() {
        return;
    }

    // TODO: synchronise Atom settings

    let mut cfg_path = utils::env::home_dir();
    cfg_path.push(Path::new(".dotfiles/config/atom.toml"));

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
        "cannot read .../atom.toml",
    );

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
}

pub fn update() {
    if !has_apm() {
        return;
    }

    println!("pkg: atom: updating packages...");

    utils::process::command_spawn_wait(COMMAND, &["upgrade", "--confirm", "false"])
        .expect(ERROR_MSG);
}

fn has_apm() -> bool {
    match utils::process::command_output("apm", &["--version"]) {
        Ok(output) => {
            return output.status.success();
        }
        Err(_error) => {
            return false; // Atom probably not installed
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
        if pkg.len() >= 1 {
            let name = pkg.split("@").next().unwrap();
            pkgs.push(String::from(name));
        }
    }
    return pkgs;
}
