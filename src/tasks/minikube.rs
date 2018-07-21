use lib::ghrtask::GHRTask;
use utils::github::Asset;
use utils::golang::{arch, os};

pub fn sync() {
    match GHR_TASK.sync() {
        Ok(_) => {}
        Err(_) => {}
    }
}

pub fn update() {
    match GHR_TASK.update() {
        Ok(_) => {}
        Err(_) => {}
    }
}

const GHR_TASK: GHRTask = GHRTask {
    asset_filter: asset_filter,
    #[cfg(windows)]
    command: "minikube.exe",
    #[cfg(not(windows))]
    command: "minikube",
    repo: ("kubernetes", "minikube"),
    trim_version: trim_version,
    version_arg: "version",
};

fn asset_filter(asset: &Asset) -> bool {
    let name = format!("minikube-{}-{}", os(), arch());

    asset.name == name
}

fn trim_version(stdout: String) -> String {
    for line in stdout.lines() {
        let parts: Vec<&str> = line.splitn(2, ":").collect();
        return String::from(parts[1].trim());
    }
    String::from("unexpected")
}
