use crate::lib::{
    ghrtask::GHRTask,
    task::{self, Task},
};
use crate::utils::github::Asset;
use crate::utils::golang::{arch, os};

pub fn task() -> Task {
    Task {
        name: "yq".to_string(),
        sync,
        update,
    }
}

const GHR_TASK: GHRTask = GHRTask {
    asset_filter,
    #[cfg(windows)]
    command: "yq.exe",
    #[cfg(not(windows))]
    command: "yq",
    repo: ("mikefarah", "yq"),
    trim_version,
    version_arg: "--version",
};

fn asset_filter(asset: &Asset) -> bool {
    #[cfg(windows)]
    let name = format!("yq_{}_{}.exe", os(), arch());
    #[cfg(not(windows))]
    let name = format!("yq_{}_{}", os(), arch());

    asset.name == name
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
