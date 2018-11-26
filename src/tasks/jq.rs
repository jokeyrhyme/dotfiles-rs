use std::env::consts::{ARCH, OS};

use crate::lib::{
    ghrtask::GHRTask,
    task::{self, Task},
};
use crate::utils::github::Asset;

pub fn task() -> Task {
    Task {
        name: "jq".to_string(),
        sync,
        update,
    }
}

const GHR_TASK: GHRTask = GHRTask {
    asset_filter,
    #[cfg(windows)]
    command: "jq.exe",
    #[cfg(not(windows))]
    command: "jq",
    repo: ("stedolan", "jq"),
    trim_version,
    version_arg: "--version",
};

fn asset_filter(asset: &Asset) -> bool {
    #[cfg(windows)]
    let name = format!("jq-{}.exe", os_arch());
    #[cfg(not(windows))]
    let name = format!("jq-{}", os_arch());

    asset.name == name
}

// this is unfortunately only true for jq 1.5,
// may need to make this smarter to match all past and future versions
fn os_arch() -> String {
    if ARCH == "x86_64" && OS == "macos" {
        return String::from("osx-amd64");
    }
    format!(
        "{}{}",
        match OS {
            "macos" => "darwin",
            "windows" => "win",
            _ => OS,
        },
        match ARCH {
            "x86_64" => "64",
            "x86" => "32",
            _ => ARCH,
        }
    )
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
