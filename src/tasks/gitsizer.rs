use regex::Regex;

use lib::ghratask::GHRATask;
use utils::{
    github::Asset,
    golang::{arch, os},
};

pub fn sync() {
    match GHRA_TASK.sync() {
        _ => {}
    }
}

pub fn update() {
    match GHRA_TASK.update() {
        _ => {}
    }
}

const GHRA_TASK: GHRATask = GHRATask {
    asset_filter,
    #[cfg(windows)]
    command: "git-sizer.exe",
    #[cfg(not(windows))]
    command: "git-sizer",
    repo: ("github", "git-sizer"),
    trim_version,
    version_arg: "--version",
};

fn asset_filter(asset: &Asset) -> bool {
    let re = Regex::new(&format!(r"^git-sizer-.*-{}-{}\.zip$", os(), arch())).unwrap();

    re.is_match(&asset.name)
}

fn trim_version(stdout: String) -> String {
    String::from(stdout.trim())
}
