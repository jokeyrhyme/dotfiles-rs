use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use std::str;

use regex::Regex;
use toml;

use utils;

const ERROR_MSG: &str = "error: rust: update";

#[derive(Debug, Deserialize)]
struct Config {
    install: Vec<String>,
}

pub fn sync() {
    let mut cfg_path = utils::env::home_dir();
    cfg_path.push(Path::new(".dotfiles/config/rust.toml"));

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
        "cannot read .../rust.toml",
    );

    let config: Config = toml::from_str(&contents).expect("cannot parse .../rust.toml");
    println!("{:?}", config);

    let output = Command::new("cargo")
        .args(&["install", "--list"])
        .output()
        .expect(ERROR_MSG);
    let stdout = str::from_utf8(&output.stdout).unwrap();

    let krates: Vec<&str> = parse_installed(&stdout);

    for krate in config.install {
        if !krates.contains(&krate.as_str()) {
            Command::new("cargo")
                .args(&["install", krate.as_str()])
                .spawn()
                .expect(ERROR_MSG)
                .wait()
                .expect(ERROR_MSG);
        }
    }
}

pub fn update() {
    match Command::new("rustup").arg("--version").spawn() {
        Ok(_child) => {
            println!("pkg: rust: updating to latest stable...");

            Command::new("rustup")
                .args(&["override", "set", "stable"])
                .spawn()
                .expect(ERROR_MSG)
                .wait()
                .expect(ERROR_MSG);
            Command::new("rustup")
                .args(&["update", "stable"])
                .spawn()
                .expect(ERROR_MSG)
                .wait()
                .expect(ERROR_MSG);
        }
        Err(_error) => {
            // rustup probably not installed, skip!
        }
    }

    match Command::new("cargo").arg("--version").spawn() {
        Ok(_child) => {
            println!("pkg: rust: updating crates...");

            let output = Command::new("cargo")
                .args(&["install", "--list"])
                .output()
                .expect(ERROR_MSG);
            let stdout = str::from_utf8(&output.stdout).unwrap();

            let krates: Vec<&str> = parse_installed(&stdout);

            let mut install_args = vec!["install", "--force"];
            install_args.extend(krates);

            Command::new("cargo")
                .args(install_args)
                .spawn()
                .expect(ERROR_MSG)
                .wait()
                .expect(ERROR_MSG);
        }
        Err(_error) => {
            // cargo probably not installed, skip!
        }
    }
}

fn parse_installed(stdout: &str) -> Vec<&str> {
    let krate = Regex::new(r"^(?P<name>\S+)\sv\d+").unwrap();
    let lines = stdout.lines();
    return lines
        .filter_map(|line| match krate.captures(line) {
            Some(caps) => Some(caps.get(1).unwrap().as_str()),
            None => None,
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_installed() {
        let input = "
racer v2.0.12:
    racer
rustfmt v0.10.0:
    cargo-fmt
    rustfmt
rustsym v0.3.2:
    rustsym
";
        assert_eq!(parse_installed(input), vec!["racer", "rustfmt", "rustsym"]);
    }
}
