use std;
use std::env::consts::{ARCH, OS};

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
    command: "yq.exe",
    #[cfg(not(windows))]
    command: "yq",
    repo: ("mikefarah", "yq"),
    trim_version: trim_version,
    version_arg: "--version",
};

fn asset_filter(asset: &Asset) -> bool {
    #[cfg(windows)]
    let name = format!("yq_{}_{}.exe", os(), arch());
    #[cfg(not(windows))]
    let name = format!("yq_{}_{}", os(), arch());

    asset.name == name
}

fn trim_version(stdout: String) -> String {
    String::from(stdout.trim())
}
