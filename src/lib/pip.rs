use std::{io, path::PathBuf, str};

use serde_derive::Deserialize;
use serde_json;

use crate::{
    lib::favourites::Favourites,
    utils::process::{command_output, command_spawn_wait},
};

const PIPS: &[&str] = &["pip", "pip3"];
const VERSION_ARGS: &[&str] = &["--version"];

#[derive(Debug, Deserialize, PartialEq)]
pub struct PipPackage {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct PipFavourites {
    install: Vec<String>,
    uninstall: Vec<String>,
}

impl Favourites for PipFavourites {
    fn cull(&mut self) -> io::Result<()> {
        let surplus = self.surplus();
        if surplus.is_empty() {
            return Ok(());
        }

        let mut args = vec![String::from("uninstall"), String::from("--yes")];
        args.extend(surplus);
        pip(&args)
    }
    fn fill(&mut self) -> io::Result<()> {
        let missing = self.missing();
        if missing.is_empty() {
            return Ok(());
        }

        let mut args = vec![String::from("install"), String::from("--user")];
        args.extend(missing);
        pip(&args)
    }
    fn found(&self) -> Vec<String> {
        found_versions().iter().map(|k| k.name.clone()).collect()
    }
    fn wanted(&self) -> Vec<String> {
        self.install.clone()
    }
    fn unwanted(&self) -> Vec<String> {
        self.uninstall.clone()
    }
}

pub fn found_versions() -> Vec<PipPackage> {
    if !has_pip() {
        return Vec::new();
    };
    let stdout = pip_output(&["list", "--format=json", "--user"]).unwrap_or_default();
    parse_pippackages(stdout)
}

pub fn has_pip() -> bool {
    pip_output(VERSION_ARGS).is_ok()
}

pub fn pip<S>(args: &[S]) -> io::Result<()>
where
    S: AsRef<str>,
{
    command_spawn_wait(pip_exe()?, args).map(|_| ())
}

pub fn pip_output<S>(args: &[S]) -> io::Result<String>
where
    S: AsRef<str>,
{
    let output = command_output(pip_exe()?, args)?;
    Ok(format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout).trim(),
        String::from_utf8_lossy(&output.stderr).trim(),
    )
    .to_string())
}

fn parse_pippackages<S>(s: S) -> Vec<PipPackage>
where
    S: AsRef<str>,
{
    match serde_json::from_str(s.as_ref()) {
        Ok(p) => p,
        Err(_) => Vec::new(),
    }
}

fn pip_exe() -> io::Result<PathBuf> {
    match which_v3() {
        Some(p) => Ok(p),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "")),
    }
}

fn which_v3() -> Option<PathBuf> {
    for pip in PIPS {
        let found = match which::which(&pip) {
            Ok(exe) => exe,
            Err(_) => continue,
        };
        match command_output(&found, VERSION_ARGS) {
            Ok(output) => {
                let stdout = String::from_utf8(output.stdout).unwrap_or_default();
                let first = stdout.lines().next().unwrap_or_default().trim();
                if !first.contains("python2.") && !first.contains("(python 2.") {
                    return Some(found);
                }
            }
            Err(_error) => continue,
        };
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pippackages_from_string() {
        let input =
            r#"[{"name": "astroid", "version": "2.2.5"}, {"name": "attrs", "version": "19.1.0"}]"#;
        let want = vec![
            PipPackage {
                name: String::from("astroid"),
                version: String::from("2.2.5"),
            },
            PipPackage {
                name: String::from("attrs"),
                version: String::from("19.1.0"),
            },
        ];
        assert_eq!(want, parse_pippackages(input));
    }
}
