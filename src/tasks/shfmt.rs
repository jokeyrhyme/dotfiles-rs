use regex::Regex;

use crate::lib::{
    ghrtask::GHRTask,
    task::{self, Status, Task},
};
use crate::utils::github::Asset;
use crate::utils::golang::{arch, os};

pub fn task() -> Task {
    Task {
        name: String::from("shfmt"),
        sync,
        update,
    }
}

const GHR_TASK: GHRTask = GHRTask {
    asset_filter,
    command: "shfmt",
    repo: ("mvdan", "sh"),
    trim_version,
    version_arg: "--version",
};

fn asset_filter(asset: &Asset) -> bool {
    let re = Regex::new(&format!("^shfmt_.*_{}_{}(\\.exe)?$", os(), arch())).unwrap();

    re.is_match(&asset.name)
}

fn sync() -> task::Result {
    GHR_TASK.sync()
}

fn trim_version(stdout: String) -> String {
    String::from(stdout.trim())
}

fn update(sync: Status) -> task::Result {
    GHR_TASK.update(sync)
}
