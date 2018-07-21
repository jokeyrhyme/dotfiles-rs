use regex::Regex;

use lib::ghrtask::GHRTask;
use utils::github::Asset;
use utils::golang::{arch, os};

pub fn sync() {
    match GHR_TASK.sync() {
        Ok(_) => {}
        Err(_) => {}
    }
}

pub fn update() {
    match GHR_TASK.update() {
        Ok(_) => {}
        Err(_) => {}
    }
}

const GHR_TASK: GHRTask = GHRTask {
    asset_filter: asset_filter,
    #[cfg(windows)]
    command: "shfmt.exe",
    #[cfg(not(windows))]
    command: "shfmt",
    repo: ("mvdan", "sh"),
    trim_version: trim_version,
    version_arg: "--version",
};

fn asset_filter(asset: &Asset) -> bool {
    let re = Regex::new(&format!("^shfmt_.*_{}_{}(\\.exe)?$", os(), arch())).unwrap();

    re.is_match(&asset.name)
}

fn trim_version(stdout: String) -> String {
    String::from(stdout.trim())
}
