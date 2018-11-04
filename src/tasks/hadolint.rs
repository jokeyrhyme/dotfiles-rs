use std::env::consts::ARCH;

use inflector::Inflector;
use regex::Regex;

use lib::{
    ghrtask::GHRTask,
    task::{self, Task},
};
use utils::github::Asset;
use utils::golang::os;

pub fn task() -> Task {
    Task {
        name: "hadolint".to_string(),
        sync,
        update,
    }
}

const GHR_TASK: GHRTask = GHRTask {
    asset_filter,
    #[cfg(windows)]
    command: "hadolint.exe",
    #[cfg(not(windows))]
    command: "hadolint",
    repo: ("hadolint", "hadolint"),
    trim_version,
    version_arg: "--version",
};

fn asset_filter(asset: &Asset) -> bool {
    let os_title = os().to_title_case();

    #[cfg(windows)]
    let name = format!("hadolint-{}-{}.exe", os_title, ARCH);
    #[cfg(not(windows))]
    let name = format!("hadolint-{}-{}", os_title, ARCH);

    asset.name == name
}

fn sync() -> task::Result {
    GHR_TASK.sync()
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn trim_version(stdout: String) -> String {
    let re = Regex::new(r"(\d+\.\d+\.\d+)").unwrap();
    let caps = re.captures_iter(stdout.trim()).next().unwrap();
    match caps.get(1) {
        Some(c) => c.as_str().to_string(),
        None => String::from("unexpected"),
    }
}

fn update() -> task::Result {
    GHR_TASK.update()
}
