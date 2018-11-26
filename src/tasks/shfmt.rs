use regex::Regex;

use crate::lib::{
    ghrtask::GHRTask,
    task::{self, Task},
};
use crate::utils::github::Asset;
use crate::utils::golang::{arch, os};

pub fn task() -> Task {
    Task {
        name: "shfmt".to_string(),
        sync,
        update,
    }
}

const GHR_TASK: GHRTask = GHRTask {
    asset_filter,
    #[cfg(windows)]
    command: "shfmt.exe",
    #[cfg(not(windows))]
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

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn trim_version(stdout: String) -> String {
    String::from(stdout.trim())
}

fn update() -> task::Result {
    GHR_TASK.update()
}
