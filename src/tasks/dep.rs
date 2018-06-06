use std;

use utils;
use utils::github::{Asset, Release};
use utils::golang::{arch, os};

const ERROR_MSG: &str = "error: dep";

pub fn sync() {
    println!("dep: syncing ...");

    if !is_installed() {
        let release = match utils::github::latest_release(&"golang", &"dep") {
            Ok(r) => r,
            Err(error) => {
                println!("error: dep: {}", error);
                return;
            }
        };
        install_release_asset(release);
    }
}

pub fn update() {
    if !is_installed() {
        return;
    }

    println!("dep: updating ...");

    let current = installed_version();
    match utils::github::release_versus_current(
        &current,
        &String::from("golang"),
        &String::from("dep"),
    ) {
        Some(r) => install_release_asset(r),
        None => {}
    };
}

fn install_release_asset(release: Release) {
    let asset = match latest_asset(&release) {
        Some(a) => a,
        None => {
            println!("dep: no asset matches OS and ARCH");
            return;
        }
    };

    println!("dep: installing ...");

    #[cfg(windows)]
    let bin_path = utils::env::home_dir().join(".local/bin/dep.exe");
    #[cfg(not(windows))]
    let bin_path = utils::env::home_dir().join(".local/bin/dep");

    utils::github::download_release_asset(asset, &bin_path);
}

fn installed_version() -> String {
    let output = utils::process::command_output("dep", &["version"]).expect(ERROR_MSG);
    let stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();
    for line in stdout.lines() {
        let parts: Vec<&str> = line.splitn(2, ":").collect();
        if parts[0].trim() == "version" {
            return String::from(parts[1].trim());
        }
    }
    String::from("unexpected")
}

fn is_installed() -> bool {
    match utils::process::command_output("dep", &["version"]) {
        Ok(output) => output.status.success(),
        Err(_error) => false,
    }
}

fn latest_asset(release: &Release) -> Option<Asset> {
    return release
        .assets
        .to_vec()
        .into_iter()
        .filter(|asset| {
            #[cfg(windows)]
            let name = format!("dep-{}-{}.exe", os(), arch());
            #[cfg(not(windows))]
            let name = format!("dep-{}-{}", os(), arch());

            asset.name == name
        })
        .next();
}
