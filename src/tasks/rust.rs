use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::result::Result;
use std::str;

use regex::Regex;
use toml;

use utils;

const ERROR_MSG: &str = "error: rust";

#[derive(Debug, Deserialize)]
struct Config {
    install: Vec<String>,
}

pub fn sync() {
    if !has_cargo() {
        return;
    }

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

    let krates = cargo_installed();

    let missing: Vec<String> = config
        .install
        .into_iter()
        .filter_map(|krate| {
            if krates.contains_key(&krate) {
                return None;
            }
            return Some(String::from(krate));
        })
        .collect();

    if missing.len() <= 0 {
        return; // nothing to do
    }

    let mut install_args = vec![String::from("install")];
    install_args.extend(missing);

    utils::process::command_spawn_wait("cargo", &install_args).expect(ERROR_MSG);
}

pub fn update() {
    if !has_rustup() {
        return;
    }

    println!("pkg: rust: updating to latest stable...");

    utils::process::command_spawn_wait("rustup", &["override", "set", "stable"]).expect(ERROR_MSG);

    utils::process::command_spawn_wait("rustup", &["update", "stable"]).expect(ERROR_MSG);

    if !has_cargo() {
        return;
    }

    println!("pkg: rust: updating crates...");

    let krates = cargo_installed();

    let outdated: Vec<String> = krates
        .into_iter()
        .filter_map(|(krate, version)| match cargo_latest_version(&krate) {
            Ok(latest) => {
                if version == latest {
                    return None;
                }
                return Some(krate);
            }
            Err(_) => None,
        })
        .collect();

    if outdated.len() <= 0 {
        return; // nothing to do
    }

    let mut install_args = vec![String::from("install"), String::from("--force")];
    install_args.extend(outdated);

    utils::process::command_spawn_wait("cargo", &install_args).expect(ERROR_MSG);
}

fn cargo_installed() -> HashMap<String, String> {
    if !has_cargo() {
        return HashMap::<String, String>::new();
    }

    let output = utils::process::command_output("cargo", &["install", "--list"]).expect(ERROR_MSG);
    let stdout = str::from_utf8(&output.stdout).unwrap();

    let krates: HashMap<String, String> = parse_installed(&stdout);
    return krates;
}

fn cargo_latest_version(krate: &str) -> Result<String, String> {
    let mut pattern = String::from("^");
    pattern.push_str(krate);
    pattern.push_str(r#"\s=\s"(\S+)""#);
    let re = Regex::new(&pattern).unwrap();
    let output = utils::process::command_output("cargo", &["search", "--limit", "1", krate])
        .expect(ERROR_MSG);
    let stdout = str::from_utf8(&output.stdout).unwrap();
    let lines = stdout.lines();
    for line in lines {
        match re.captures(line) {
            Some(caps) => {
                let version = String::from(caps.get(1).unwrap().as_str());
                return Ok(version);
            }
            None => (),
        };
    }
    return Err(String::from("not found"));
}

fn has_cargo() -> bool {
    match utils::process::command_output("cargo", &["--version"]) {
        Ok(output) => {
            return output.status.success();
        }
        Err(_error) => {
            return false; // cargo probably not installed
        }
    }
}

fn has_rustup() -> bool {
    match utils::process::command_output("rustup", &["--version"]) {
        Ok(output) => {
            return output.status.success();
        }
        Err(_error) => {
            return false; // rustup probably not installed
        }
    }
}

fn parse_installed(stdout: &str) -> HashMap<String, String> {
    let re = Regex::new(r"^(?P<name>\S+)\sv(?P<version>\S+):").unwrap();
    let lines = stdout.lines();
    let mut krates: HashMap<String, String> = HashMap::new();

    for line in lines {
        match re.captures(line) {
            Some(caps) => {
                let krate = caps.get(1).unwrap().as_str();
                let version = caps.get(2).unwrap().as_str();
                krates.insert(String::from(krate), String::from(version));
            }
            None => (),
        }
    }
    return krates;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_installed() {
        cargo_installed();
    }

    #[test]
    fn test_cargo_latest_version() {
        match cargo_latest_version("serde") {
            Ok(version) => assert!(version.len() > 0),
            Err(_) => assert!(false),
        }
        match cargo_latest_version("this-does-not-exist-maybe") {
            Ok(_) => assert!(false),
            Err(error) => assert_eq!("not found", error),
        }
    }

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
        let want: HashMap<String, String> = [
            (String::from("racer"), String::from("2.0.12")),
            (String::from("rustfmt"), String::from("0.10.0")),
            (String::from("rustsym"), String::from("0.3.2")),
        ].iter()
            .cloned()
            .collect();
        assert_eq!(want, parse_installed(input));
    }
}
