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
    command: "gitleaks.exe",
    #[cfg(not(windows))]
    command: "gitleaks",
    repo: ("zricethezav", "gitleaks"),
    trim_version: trim_version,
    version_arg: "version",
};

fn asset_filter(asset: &Asset) -> bool {
    #[cfg(windows)]
    let name = format!("gitleaks-{}-{}.exe", os(), arch());
    #[cfg(not(windows))]
    let name = format!("gitleaks-{}-{}", os(), arch());

    asset.name == name
}

fn trim_version(stdout: String) -> String {
    String::from(stdout.trim())
}
