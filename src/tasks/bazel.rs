use std::env::consts::ARCH;

use regex::Regex;

use lib::ghrtask::GHRTask;
use utils::github::Asset;
use utils::golang::os;

pub fn sync() {
    match GHR_TASK.sync() {
        _ => {}
    }
}

pub fn update() {
    match GHR_TASK.update() {
        _ => {}
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
