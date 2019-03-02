use regex::Regex;

use crate::lib::{
    ghratask::GHRATask,
    task::{self, Task},
};
use crate::utils::{
    github::Asset,
    golang::{arch, os},
};

pub fn task() -> Task {
    Task {
        name: String::from("git-sizer"),
        sync,
        update,
    }
}

const GHRA_TASK: GHRATask = GHRATask {
    asset_filter,
    command: "git-sizer",
    repo: ("github", "git-sizer"),
    trim_version,
    version_arg: "--version",
};

fn asset_filter(asset: &Asset) -> bool {
    let re = Regex::new(&format!(r"^git-sizer-.*-{}-{}\.zip$", os(), arch())).unwrap();

    re.is_match(&asset.name)
}

fn sync() -> task::Result {
    GHRA_TASK.sync()
}

#[allow(clippy::needless_pass_by_value)]
fn trim_version(stdout: String) -> String {
    String::from(stdout.trim())
}

fn update() -> task::Result {
    GHRA_TASK.update()
}
