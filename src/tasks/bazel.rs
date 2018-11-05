use std::env::consts::ARCH;

use regex::Regex;

use lib::{
    ghrtask::GHRTask,
    task::{self, Task},
};
use utils::github::Asset;
use utils::golang::os;

pub fn task() -> Task {
    Task {
        name: "bazel".to_string(),
        sync,
        update,
    }
}

const GHR_TASK: GHRTask = GHRTask {
    asset_filter,
    #[cfg(windows)]
    command: "bazel.exe",
    #[cfg(not(windows))]
    command: "bazel",
    repo: ("bazelbuild", "bazel"),
    trim_version,
    version_arg: "version",
};

fn asset_filter(asset: &Asset) -> bool {
    let re = Regex::new(&format!("^bazel-.*-{}-{}(\\.exe)?$", os(), ARCH)).unwrap();

    re.is_match(&asset.name)
}

fn sync() -> task::Result {
    GHR_TASK.sync()
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn trim_version(stdout: String) -> String {
    for line in stdout.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts[0].trim() == "Build label" {
            return String::from(parts[1].trim());
        }
    }
    String::from("unexpected")
}

fn update() -> task::Result {
    GHR_TASK.update()
}