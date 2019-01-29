use std::{fs, io, str};

use regex::Regex;
use toml;

use crate::{
    lib::{
        cargo::{self, CargoFavourites},
        favourites::Favourites,
        rust,
        task::{self, Status, Task},
    },
    utils,
};

pub fn task() -> Task {
    Task {
        name: "rust".to_string(),
        sync,
        update,
    }
}

#[allow(clippy::needless_pass_by_value)]
fn cargo_latest_version<S>(krate: S) -> Result<String, String>
where
    S: Into<String> + AsRef<str>,
{
    let mut pattern = String::from("^");
    pattern.push_str(krate.as_ref());
    pattern.push_str(r#"\s=\s"(\S+)""#);
    let re = Regex::new(&pattern).unwrap();
    let stdout =
        cargo::cargo_output(&["search", "--limit", "1", krate.as_ref()]).unwrap_or_default();
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
    if !cargo::has_cargo() || !rust::has_rustup() {
        return Ok(());
    }
    if has_cargo_installed_rustfmt() {
        cargo::cargo(&["uninstall", "rustfmt", "rustfmt-nightly", "rustfmt-preview"])?;
    }
    if !has_cargo_fmt() {
        rust::rustup(&["component", "remove", "rustfmt-preview"])?;
        rust::rustup(&["component", "add", "rustfmt-preview"])?;
    }
    Ok(())
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
    cargo::found_versions().contains_key("rustfmt")
}

fn sync() -> task::Result {
    if !cargo::has_cargo() {
        return Ok(Status::Skipped);
    }

    let cfg_path = utils::env::home_dir().join(".dotfiles/config/rust.toml");

    let contents = match fs::read_to_string(&cfg_path) {
        Ok(s) => s,
        Err(error) => {
            return Err(task::Error::IOError("ignoring config".to_string(), error));
        }
    };

    let mut favs: CargoFavourites = toml::from_str(&contents).expect("cannot parse .../rust.toml");
    Favourites::fill_and_status(&mut favs)?;
    Favourites::cull_and_status(&mut favs)?;

    match fix_cargo_fmt() {
        Ok(()) => {}
        Err(error) => println!("error: rust: unable to fix `cargo fmt`: {:?}", error),
    };

    Ok(Status::Done)
}

fn update() -> task::Result {
    if !cargo::has_cargo() {
        return Ok(Status::Done);
    }

    let krates = cargo::found_versions();

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
        )
        .collect();

    if outdated.is_empty() {
        return Ok(Status::Done);
    }

    let mut install_args = vec![String::from("install"), String::from("--force")];
    install_args.extend(outdated);

    cargo::cargo(&install_args)?;
    Ok(Status::Done)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
