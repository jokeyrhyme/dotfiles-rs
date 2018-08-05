use lib::ghrtask::GHRTask;
use utils::github::Asset;
use utils::golang::{arch, os};

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
    command: "minikube.exe",
    #[cfg(not(windows))]
    command: "minikube",
    repo: ("kubernetes", "minikube"),
    trim_version,
    version_arg: "version",
};

fn asset_filter(asset: &Asset) -> bool {
    let name = format!("minikube-{}-{}", os(), arch());

    asset.name == name
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn trim_version(stdout: String) -> String {
    let line = stdout.lines().next().unwrap_or_default();
    let parts: Vec<&str> = line.splitn(2, ':').collect();
    if parts.len() < 2 {
        return String::from(parts[1].trim());
    }
    String::from("unexpected")
}
