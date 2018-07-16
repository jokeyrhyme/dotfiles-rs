use std::env::consts::ARCH;

use inflector::Inflector;
use regex::Regex;

use lib::ghrtask::GHRTask;
use utils::github::Asset;
use utils::golang::os;

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
    command: "hadolint.exe",
    #[cfg(not(windows))]
    command: "hadolint",
    repo: ("hadolint", "hadolint"),
    trim_version: trim_version,
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

fn trim_version(stdout: String) -> String {
    let re = Regex::new(r"(\d+\.\d+\.\d+)").unwrap();
    for caps in re.captures_iter(stdout.trim()) {
        return caps.get(1).unwrap().as_str().to_string();
    }
    String::from("unexpected")
}
