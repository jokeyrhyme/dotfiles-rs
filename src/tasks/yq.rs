use std::env::consts::EXE_SUFFIX;

use crate::{
    lib::{
        ghrtask::GHRTask,
        task::{self, Task},
    },
    utils::{
        github::Asset,
        golang::{arch, os},
    },
};

pub fn task() -> Task {
    Task {
        name: "yq".to_string(),
        sync,
        update,
    }
}

const GHR_TASK: GHRTask = GHRTask {
    asset_filter,
    command: "yq",
    repo: ("mikefarah", "yq"),
    trim_version,
    version_arg: "--version",
};

fn asset_filter(asset: &Asset) -> bool {
    let name = format!("yq_{}_{}{}", os(), arch(), EXE_SUFFIX);

    asset.name == name
}

fn sync() -> task::Result {
    GHR_TASK.sync()
}

#[allow(clippy::needless_pass_by_value)]
fn trim_version(stdout: String) -> String {
    String::from(stdout.trim())
}

fn update() -> task::Result {
    GHR_TASK.update()
}
