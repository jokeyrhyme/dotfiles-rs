use std::env::consts::{ARCH, OS};

use serde_json;

use utils;

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

pub fn has_npm() -> bool {
    match utils::process::command_output("npm", &["--version"]) {
        Ok(output) => {
            return output.status.success();
        }
        Err(_error) => {
            return false; // npx probably not installed
        }
    }
}

pub fn has_npx() -> bool {
    match utils::process::command_output("npx", &["--version"]) {
        Ok(output) => {
            return output.status.success();
        }
        Err(_error) => {
            return false; // npx probably not installed
        }
    }
}

pub fn has_yarn() -> bool {
    match utils::process::command_output("yarn", &["--version"]) {
        Ok(output) => {
            return output.status.success();
        }
        Err(_error) => {
            return false; // npx probably not installed
        }
    }
}

pub fn latest_version() -> String {
    let req = utils::http::create_request(&DIST_JSON_URL, &utils::http::EMPTY_HEADERS);
    let res = utils::http::fetch_request(&req).expect(ERROR_MSG);
    let body = res.body_as_string().expect(ERROR_MSG);
    let releases: Vec<Release> = serde_json::from_str(&body).expect(ERROR_MSG);

    let latest_release: &Release = releases
        .iter()
        .filter(|r| {
            r.files.len() > 0 &&
                r.files.iter().any(|f| {
                    f.starts_with(&format!(
                        "{}-{}",
                        utils::nodejs::os(),
                        utils::nodejs::arch()
                    ))
                })
        })
        .next()
        .unwrap();

    latest_release.version.clone()
}

pub fn os() -> &'static str {
    match OS {
        "macos" => "osx",
        "windows" => "win",
        _ => OS,
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
