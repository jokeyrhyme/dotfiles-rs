use std::collections::HashMap;
use std::{fs, io, str};

use regex::Regex;
use toml;

use lib::{
    rust,
    task::{self, Status, Task},
};
use utils;

const ERROR_MSG: &str = "error: rust";

#[derive(Debug, Deserialize)]
struct Config {
    install: Vec<String>,
}

pub fn task() -> Task {
    Task {
        name: "rust".to_string(),
        sync,
        update,
    }
}

fn cargo_installed() -> HashMap<String, String> {
    if !has_cargo() {
        return HashMap::<String, String>::new();
    }

    let output = utils::process::command_output("cargo", &["install", "--list"]).expect(ERROR_MSG);
    let stdout = str::from_utf8(&output.stdout).unwrap();

    let krates: HashMap<String, String> = parse_installed(stdout);
    krates
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn cargo_latest_version<S>(krate: S) -> Result<String, String>
where
    S: Into<String> + AsRef<str>,
{
    let mut pattern = String::from("^");
    pattern.push_str(krate.as_ref());
    pattern.push_str(r#"\s=\s"(\S+)""#);
    let re = Regex::new(&pattern).unwrap();
    let output =
        utils::process::command_output("cargo", &["search", "--limit", "1", krate.as_ref()])
            .expect(ERROR_MSG);
    let stdout = str::from_utf8(&output.stdout).unwrap();
    let lines = stdout.lines();
    for line in lines {
        if let Some(caps) = re.captures(line) {
            let version = String::from(caps.get(1).unwrap().as_str());
            return Ok(version);
        };
    }
    Err(String::from("not found"))
}

fn fix_cargo_fmt() -> io::Result<()> {
    if !has_cargo() || !rust::has_rustup() {
        return Ok(());
    }
    if has_cargo_installed_rustfmt() {
        utils::process::command_spawn_wait(
            "cargo",
            &["uninstall", "rustfmt", "rustfmt-nightly", "rustfmt-preview"],
        )?;
    }
    if !has_cargo_fmt() {
        rust::rustup(&["component", "remove", "rustfmt-preview"])?;
        rust::rustup(&["component", "add", "rustfmt-preview"])?;
    }
    Ok(())
}

fn has_cargo() -> bool {
    match utils::process::command_output("cargo", &["--version"]) {
        Ok(output) => output.status.success(),
        Err(_error) => {
            false // cargo probably not installed
        }
    }
}

fn has_cargo_fmt() -> bool {
    match utils::process::command_output("cargo", &["fmt", "--help"]) {
        Ok(output) => output.status.success(),
        Err(_error) => {
            false // cargo probably not installed
        }
    }
}

fn has_cargo_installed_rustfmt() -> bool {
    let krates = cargo_installed();
    krates.contains_key("rustfmt")
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn parse_installed<S>(stdout: S) -> HashMap<String, String>
where
    S: Into<String> + AsRef<str>,
{
    let re = Regex::new(r"^(?P<name>\S+)\sv(?P<version>\S+):").unwrap();
    let lines = stdout.as_ref().lines();
    let mut krates: HashMap<String, String> = HashMap::new();

    for line in lines {
        if let Some(caps) = re.captures(line) {
            let krate = caps.get(1).unwrap().as_str();
            let version = caps.get(2).unwrap().as_str();
            krates.insert(String::from(krate), String::from(version));
        }
    }
    krates
}

fn sync() -> task::Result {
    if !has_cargo() {
        return Ok(Status::Skipped);
    }

    let cfg_path = utils::env::home_dir().join(".dotfiles/config/rust.toml");

    let contents = match fs::read_to_string(&cfg_path) {
        Ok(s) => s,
        Err(error) => {
            return Err(task::Error::IOError("ignoring config".to_string(), error));
        }
    };

    let config: Config = toml::from_str(&contents).expect("cannot parse .../rust.toml");

    let krates = cargo_installed();

    let missing: Vec<String> = config
        .install
        .into_iter()
        .filter_map(|krate| {
            if krates.contains_key(&krate) {
                return None;
            }
            Some(krate)
        }).collect();

    if missing.is_empty() {
        return Ok(Status::Done);
    }

    let mut install_args = vec![String::from("install")];
    install_args.extend(missing);

    utils::process::command_spawn_wait("cargo", &install_args).expect(ERROR_MSG);

    println!("rust: ensuring `cargo fmt` works ...");
    match fix_cargo_fmt() {
        Ok(()) => {}
        Err(error) => println!("error: rust: unable to fix `cargo fmt`: {:?}", error),
    };

    Ok(Status::Done)
}

fn update() -> task::Result {
    if !has_cargo() {
        return Ok(Status::Done);
    }

    let krates = cargo_installed();

    let outdated: Vec<String> = krates
        .into_iter()
        .filter_map(
            |(krate, version)| match cargo_latest_version(krate.as_str()) {
                Ok(latest) => {
                    if version == latest {
                        return None;
                    }
                    Some(krate)
                }
                Err(_) => None,
            },
        ).collect();

    if outdated.is_empty() {
        return Ok(Status::Done);
    }

    let mut install_args = vec![String::from("install"), String::from("--force")];
    install_args.extend(outdated);

    utils::process::command_spawn_wait("cargo", &install_args)?;
    Ok(Status::Done)
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
            Ok(version) => assert!(!version.is_empty()),
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
        ]
            .iter()
            .cloned()
            .collect();
        assert_eq!(want, parse_installed(input));
    }
}
