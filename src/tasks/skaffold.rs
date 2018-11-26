use crate::lib::{
    ghrtask::GHRTask,
    task::{self, Task},
};
use crate::utils::github::Asset;
use crate::utils::golang::{arch, os};

pub fn task() -> Task {
    Task {
        name: "skaffold".to_string(),
        sync,
        update,
    }
}

const GHR_TASK: GHRTask = GHRTask {
    asset_filter,
    #[cfg(windows)]
    command: "skaffold.exe",
    #[cfg(not(windows))]
    command: "skaffold",
    repo: ("GoogleCloudPlatform", "skaffold"),
    trim_version,
    version_arg: "version",
};

fn asset_filter(asset: &Asset) -> bool {
    #[cfg(windows)]
    let name = format!("skaffold-{}-{}.exe", os(), arch());
    #[cfg(not(windows))]
    let name = format!("skaffold-{}-{}", os(), arch());

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
