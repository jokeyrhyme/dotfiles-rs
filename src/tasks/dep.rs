use std::env::consts::OS;

use crate::lib::{
    ghrtask::GHRTask,
    task::{self, Status, Task},
};
use crate::utils::github::Asset;
use crate::utils::golang::{arch, os};

pub fn task() -> Task {
    Task {
        name: String::from("dep"),
        sync,
        update,
    }
}

const GHR_TASK: GHRTask = GHRTask {
    asset_filter,
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

fn trim_version(stdout: String) -> String {
    for line in stdout.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts[0].trim() == "version" {
            return String::from(parts[1].trim());
        }
    }
    String::from("unexpected")
}

fn update(sync: Status) -> task::Result {
    GHR_TASK.update(sync)
}
