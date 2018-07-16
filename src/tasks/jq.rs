use std;
use std::env::consts::{ARCH, OS};

use lib::ghrtask::GHRTask;
use utils::github::Asset;

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
    command: "jq.exe",
    #[cfg(not(windows))]
    command: "jq",
    repo: ("stedolan", "jq"),
    trim_version: trim_version,
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

fn trim_version(stdout: String) -> String {
    String::from(stdout.trim())
}
