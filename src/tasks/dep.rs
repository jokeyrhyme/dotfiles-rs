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
    command: "dep.exe",
    #[cfg(not(windows))]
    command: "dep",
    repo: ("golang", "dep"),
    trim_version,
    version_arg: "version",
};

fn asset_filter(asset: &Asset) -> bool {
    #[cfg(windows)]
    let name = format!("dep-{}-{}.exe", os(), arch());
    #[cfg(not(windows))]
    let name = format!("dep-{}-{}", os(), arch());

    asset.name == name
}

fn trim_version(stdout: String) -> String {
    for line in stdout.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts[0].trim() == "version" {
            return String::from(parts[1].trim());
        }
    }
    String::from("unexpected")
}
