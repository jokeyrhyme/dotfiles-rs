use std::env::consts::OS;

use crate::lib::{
    ghrtask::GHRTask,
    task::{self, Task},
};
use crate::utils::github::Asset;
use crate::utils::golang::{arch, os};

pub fn task() -> Task {
    Task {
        name: "dep".to_string(),
        sync,
        update,
    }
}

const GHR_TASK: GHRTask = GHRTask {
    asset_filter,
    #[cfg(windows)]
    command: "dep.exe",
    #[cfg(not(windows))]
    command: "dep",
    repo: ("golang", "dep"),
    trim_version,
    version_arg: "version",
};

fn asset_filter(asset: &Asset) -> bool {
    let name = if OS == "windows" {
        format!("dep-{}-{}.exe", os(), arch())
    } else {
        format!("dep-{}-{}", os(), arch())
    };

    asset.name == name
}

fn sync() -> task::Result {
    GHR_TASK.sync()
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn trim_version(stdout: String) -> String {
    for line in stdout.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts[0].trim() == "version" {
            return String::from(parts[1].trim());
        }
    }
    String::from("unexpected")
}

fn update() -> task::Result {
    GHR_TASK.update()
}
