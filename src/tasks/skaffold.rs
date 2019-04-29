use std::env::consts::EXE_SUFFIX;

use crate::{
    lib::{
        ghrtask::GHRTask,
        task::{self, Status, Task},
    },
    utils::{
        github::Asset,
        golang::{arch, os},
    },
};

pub fn task() -> Task {
    Task {
        name: String::from("skaffold"),
        sync,
        update,
    }
}

const GHR_TASK: GHRTask = GHRTask {
    asset_filter,
    command: "skaffold",
    repo: ("GoogleCloudPlatform", "skaffold"),
    trim_version,
    version_arg: "version",
};

fn asset_filter(asset: &Asset) -> bool {
    let name = format!("skaffold-{}-{}{}", os(), arch(), EXE_SUFFIX);

    asset.name == name
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
