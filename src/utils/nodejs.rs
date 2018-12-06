use std;
use std::env::consts::{ARCH, OS};
use std::path::PathBuf;
use std::string::String;

use serde_json;

use crate::lib::version;
use crate::utils;

const DIST_JSON_URL: &str = "https://nodejs.org/dist/index.json";
const ERROR_MSG: &str = "error: utils: nodejs";

#[derive(Debug, Deserialize)]
pub struct Release {
    pub files: Vec<String>,
    pub version: String,
}

pub fn arch() -> &'static str {
    match ARCH {
        "x86_64" => "x64",
        _ => ARCH,
    }
}

pub fn bin_dir() -> PathBuf {
    if OS == "windows" {
        install_path()
    } else {
        install_path().join("bin")
    }
}

pub fn current_version() -> String {
    match utils::process::command_output("node", &["--version"]) {
        Ok(output) => String::from(
            std::str::from_utf8(&output.stdout)
                .unwrap_or_default()
                .trim(),
        ),
        Err(_error) => String::from(""),
    }
}

pub fn has_node() -> bool {
    bin_dir()
        .join(if OS == "windows" { "node.exe" } else { "node" })
        .is_file()
}

pub fn has_npm() -> bool {
    match utils::process::command_output("npm", &["--version"]) {
        Ok(output) => output.status.success(),
        Err(_error) => {
            false // npm probably not installed
        }
    }
}

pub fn has_npx() -> bool {
    match utils::process::command_output("npx", &["--version"]) {
        Ok(output) => output.status.success(),
        Err(_error) => {
            false // npx probably not installed
        }
    }
}

pub fn has_yarn() -> bool {
    match utils::process::command_output("yarn", &["--version"]) {
        Ok(output) => output.status.success(),
        Err(_error) => {
            false // yarn probably not installed
        }
    }
}

pub fn install_path() -> PathBuf {
    utils::env::home_dir().join(".local").join("node")
}

pub fn latest_version() -> String {
    let req = utils::http::create_request(DIST_JSON_URL, &utils::http::EMPTY_HEADERS);
    let res = utils::http::fetch_request(&req).expect(ERROR_MSG);
    let body = res.body_as_string().expect(ERROR_MSG);
    let releases: Vec<Release> = serde_json::from_str(&body).expect(ERROR_MSG);

    let latest_release: &Release = releases
        .iter()
        .find(|r| {
            version::is_stable(r.version.as_str())
                && !r.files.is_empty()
                && r.files.iter().any(|f| {
                    f.starts_with(&format!(
                        "{}-{}",
                        utils::nodejs::release_os(),
                        utils::nodejs::arch()
                    ))
                })
        })
        .unwrap();

    String::from(latest_release.version.as_str().trim())
}

pub fn lib_dir() -> PathBuf {
    if OS == "windows" {
        install_path()
    } else {
        install_path().join("lib")
    }
}

pub fn os() -> &'static str {
    match OS {
        "macos" => "darwin",
        "windows" => "win",
        _ => OS,
    }
}

// the OS information in the release JSON is a bit different again :shrug:
pub fn release_os() -> &'static str {
    match OS {
        "macos" => "osx",
        _ => os(),
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
        let version = latest_version();
        assert!(version.starts_with('v'));
    }

    #[test]
    fn os_is_not_macos_or_windows() {
        assert_ne!(os(), "macos");
        assert_ne!(os(), "windows");
    }
}
