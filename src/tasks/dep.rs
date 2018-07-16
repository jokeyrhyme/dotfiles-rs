use std;

use lib::ghrtask::GHRTask;
use utils::github::Asset;
use utils::golang::{arch, os};

const ERROR_MSG: &str = "error: dep";

const GHR_TASK: GHRTask = GHRTask {
    asset_filter: asset_filter,
    #[cfg(windows)]
    command: "dep.exe",
    #[cfg(not(windows))]
    command: "dep",
    repo: ("golang", "dep"),
    trim_version: trim_version,
    version_arg: "version",
};

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

fn asset_filter(asset: &Asset) -> bool {
    #[cfg(windows)]
    let name = format!("dep-{}-{}.exe", os(), arch());
    #[cfg(not(windows))]
    let name = format!("dep-{}-{}", os(), arch());

    asset.name == name
}

fn trim_version(stdout: String) -> String {
    for line in stdout.lines() {
        let parts: Vec<&str> = line.splitn(2, ":").collect();
        if parts[0].trim() == "version" {
            return String::from(parts[1].trim());
        }
    }
    String::from("unexpected")
}
