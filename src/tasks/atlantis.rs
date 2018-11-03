use regex::Regex;

use lib::{
    ghratask::GHRATask,
    task::{self, Status, Task},
};
use utils::{
    github::Asset,
    golang::{arch, os},
};

pub fn task() -> Task {
    Task { sync, update }
}

const GHRA_TASK: GHRATask = GHRATask {
    asset_filter,
    #[cfg(windows)]
    command: "atlantis.exe",
    #[cfg(not(windows))]
    command: "atlantis",
    repo: ("runatlantis", "atlantis"),
    trim_version,
    version_arg: "version",
};

fn asset_filter(asset: &Asset) -> bool {
    let re = Regex::new(&format!(r"^atlantis_{}_{}\.zip$", os(), arch())).unwrap();

    re.is_match(&asset.name)
}

fn sync() -> task::Result {
    match GHRA_TASK.sync() {
        _ => Ok(Status::Done),
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn trim_version(stdout: String) -> String {
    String::from(stdout.trim())
}

fn update() -> task::Result {
    match GHRA_TASK.update() {
        _ => Ok(Status::Done),
    }
}
