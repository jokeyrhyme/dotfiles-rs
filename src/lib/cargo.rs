use std::{collections::HashMap, env::consts::OS, io, path::PathBuf, str};

use regex;
use serde_derive::Deserialize;

use crate::{
    lib::{favourites::Favourites, rust::bin_dir},
    utils::process::{command_output, command_spawn_wait},
};

#[derive(Debug, Deserialize)]
pub struct CargoFavourites {
    install: Vec<String>,
    uninstall: Vec<String>,
}

impl Favourites for CargoFavourites {
    fn cull(&mut self) -> io::Result<()> {
        let surplus = self.surplus();
        if surplus.is_empty() {
            return Ok(());
        }

        let mut args = vec![String::from("uninstall")];
        args.extend(surplus);
        cargo(&args)
    }
    fn fill(&mut self) -> io::Result<()> {
        let missing = self.missing();
        if missing.is_empty() {
            return Ok(());
        }

        let mut args = vec![String::from("install")];
        args.extend(missing);
        cargo(&args)
    }
    fn found(&self) -> Vec<String> {
        found_versions().keys().map(|k| k.clone()).collect()
    }
    fn wanted(&self) -> Vec<String> {
        self.install.clone()
    }
    fn unwanted(&self) -> Vec<String> {
        self.uninstall.clone()
    }
}

pub fn found_versions() -> HashMap<String, String> {
    if !has_cargo() {
        return HashMap::<String, String>::new();
    };
    let stdout = cargo_output(&["install", "--list"]).unwrap_or_default();
    parse_installed(stdout)
}

pub fn has_cargo() -> bool {
    match command_output(cargo_exe(), &["--version"]) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn cargo<S>(args: &[S]) -> io::Result<()>
where
    S: Into<String> + AsRef<str>,
{
    command_spawn_wait(cargo_exe(), args).map(|_| ())
}

pub fn cargo_output<S>(args: &[S]) -> io::Result<String>
where
    S: Into<String> + AsRef<str>,
{
    let output = command_output(cargo_exe(), args)?;
    Ok(format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout).trim(),
        String::from_utf8_lossy(&output.stderr).trim(),
    )
    .to_string())
}

fn cargo_exe() -> PathBuf {
    bin_dir().join(if OS == "windows" {
        "cargo.exe"
    } else {
        "cargo"
    })
}

fn parse_installed<S>(stdout: S) -> HashMap<String, String>
where
    S: Into<String>,
{
    let re = regex::Regex::new(r"^(?P<name>\S+)\sv(?P<version>\S+):").unwrap();
    let s = stdout.into();
    let mut krates: HashMap<String, String> = HashMap::new();

    for line in s.lines() {
        if let Some(caps) = re.captures(line) {
            let krate = caps.get(1).unwrap().as_str();
            let version = caps.get(2).unwrap().as_str();
            krates.insert(String::from(krate), String::from(version));
        }
    }
    krates
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
