use std::env::consts::{ARCH, OS};

use regex::Regex;

use lib::ghratask::GHRATask;
use utils::github::Asset;

pub fn sync() {
    match GHRA_TASK.sync() {
        Ok(_) => {}
        Err(_) => {}
    }
}

pub fn update() {
    match GHRA_TASK.update() {
        Ok(_) => {}
        Err(_) => {}
    }
}

const GHRA_TASK: GHRATask = GHRATask {
    asset_filter: asset_filter,
    #[cfg(windows)]
    command: "vale.exe",
    #[cfg(not(windows))]
    command: "vale",
    repo: ("errata-ai", "vale"),
    trim_version: trim_version,
    version_arg: "--version",
};

pub fn arch() -> &'static str {
    if ARCH == "x86_64" {
        "64-bit"
    } else {
        ARCH
    }
}

fn asset_filter(asset: &Asset) -> bool {
    #[cfg(windows)]
    let re = Regex::new(&format!(r"^vale_.*_{}_{}\.zip$", os(), arch())).unwrap();
    #[cfg(not(windows))]
    let re = Regex::new(&format!(r"^vale_.*_{}_{}\.tar\.gz$", os(), arch())).unwrap();

    re.is_match(&asset.name)
}

pub fn os() -> &'static str {
    match OS {
        "linux" => "Linux",
        "macos" => "macOS",
        "windows" => "Windows",
        _ => OS,
    }
}

fn trim_version(stdout: String) -> String {
    String::from(stdout.trim())
}
