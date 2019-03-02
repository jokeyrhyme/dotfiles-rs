use std::{
    env::consts::{ARCH, OS},
    path::PathBuf,
    str,
};

use crate::lib::{task, version};
use crate::utils;

pub fn arch() -> &'static str {
    if ARCH == "x86_64" {
        "amd64"
    } else {
        ARCH
    }
}

pub fn bin_dir() -> PathBuf {
    goroot().join("bin")
}

pub fn current_version() -> String {
    let exe_path = bin_dir().join(if OS == "windows" { "go.exe" } else { "go" });
    match utils::process::command_output(exe_path.to_str().unwrap(), &["version"]) {
        Ok(output) => {
            let stdout = str::from_utf8(&output.stdout).unwrap_or_default().trim();
            let trailer = format!(" {}/{}", os(), arch());
            let headless = str::replace(stdout, "go version ", "");
            str::replace(&headless, &trailer, "")
        }
        Err(_error) => String::from(""),
    }
}

pub fn gopath() -> PathBuf {
    utils::env::home_dir().join("go")
}

pub fn goroot() -> PathBuf {
    utils::env::home_dir().join(".local").join("go")
}

pub fn is_installed() -> bool {
    let exe_path = bin_dir().join(if OS == "windows" { "go.exe" } else { "go" });
    exe_path.is_file()
}

pub fn latest_version() -> Result<String, task::Error> {
    let tags: Vec<utils::github::Tag> = utils::github::fetch_tags("golang", "go")?;
    let release_tags: Vec<utils::github::Tag> = tags
        .into_iter()
        .filter(|t| {
            // release tags look like "go1.10.2"
            // other tags start with "weekly.", or "release.", etc
            t.id.starts_with("go") && version::is_stable(t.id.as_str())
        })
        .collect();

    if release_tags.is_empty() {
        return Err(task::Error::NoTagsError {});
    }
    match release_tags.last() {
        Some(latest) => Ok(latest.clone().id),
        None => Err(task::Error::NoTagsError {}),
    }
}

pub fn os() -> &'static str {
    if OS == "macos" {
        "darwin"
    } else {
        OS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arch_is_not_x86_64() {
        assert_ne!(arch(), "x86_64");
    }

    #[test]
    fn latest_version_found() {
        assert!(latest_version().unwrap().len() > 0);
    }

    #[test]
    fn os_is_not_macos() {
        assert_ne!(os(), "macos");
    }
}
