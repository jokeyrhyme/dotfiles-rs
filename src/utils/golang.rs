use std::env::consts::{ARCH, OS};
use std::error::Error;
use std::path::PathBuf;
use std::{self, fmt, io, str};

use lib::version;
use utils;

#[derive(Debug)]
pub enum GolangError {
    NoTagsError,
    IoError(io::Error),
}

impl fmt::Display for GolangError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        match *self {
            GolangError::NoTagsError => fmt::Display::fmt(&"NoTagsError", f),
            GolangError::IoError(ref err) => fmt::Display::fmt(err, f),
        }
    }
}

impl Error for GolangError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            GolangError::NoTagsError => None,
            GolangError::IoError(ref err) => Some(err as &Error),
        }
    }

    fn description(&self) -> &str {
        match *self {
            GolangError::NoTagsError => &"NoTagsError",
            GolangError::IoError(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for GolangError {
    fn from(error: io::Error) -> GolangError {
        GolangError::IoError(error)
    }
}

pub fn arch() -> &'static str {
    if ARCH == "x86_64" {
        "amd64"
    } else {
        ARCH
    }
}

pub fn current_version() -> String {
    #[cfg(windows)]
    let exe_path = install_path().join("bin").join("go.exe");
    #[cfg(not(windows))]
    let exe_path = install_path().join("bin").join("go");

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

fn install_path() -> PathBuf {
    utils::env::home_dir().join(".local").join("go")
}

pub fn is_installed() -> bool {
    #[cfg(windows)]
    let exe_path = install_path().join("bin").join("go.exe");
    #[cfg(not(windows))]
    let exe_path = install_path().join("bin").join("go");

    exe_path.is_file()
}

pub fn latest_version() -> Result<String, GolangError> {
    let tags: Vec<utils::github::Tag> = match utils::github::fetch_tags("golang", "go") {
        Ok(t) => {
            t.into_iter()
                .filter(|t| {
                    // release tags look like "go1.10.2"
                    // other tags start with "weekly.", or "release.", etc
                    t.id.starts_with("go") && version::is_stable(t.id.as_str())
                })
                .collect()
        }
        Err(error) => {
            return Err(GolangError::IoError(error));
        }
    };
    if tags.is_empty() {
        return Err(GolangError::NoTagsError {});
    }
    match tags.last() {
        Some(latest) => Ok(latest.clone().id),
        None => Err(GolangError::NoTagsError {}),
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
