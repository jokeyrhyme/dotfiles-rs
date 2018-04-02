use std;
use std::env::consts::{ARCH, OS};
use std::path::Path;

use utils;
use utils::github::Release;

pub fn sync() {
    println!("pkg: skaffold: syncing ...");

    if !is_installed() {
        let release = match utils::github::latest_release(&"GoogleCloudPlatform", &"skaffold") {
            Ok(r) => r,
            Err(error) => {
                println!("error: pkg: skaffold: {}", error);
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

    println!("pkg: skaffold: updating ...");

    match utils::process::command_output("skaffold", &["version"]) {
        Ok(output) => {
            let stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

            let release =
                match utils::github::latest_release(&"GoogleCloudPlatform", &"skaffold") {
                    Ok(r) => r,
                    Err(error) => {
                        println!("error: pkg: skaffold: {}", error);
                        return;
                    }
                };

            {
                let installed = stdout.trim_left_matches("v").trim();
                let latest = release.tag_name.trim_left_matches("v").trim();

                println!("pkg: skaffold: current={} latest={}", &installed, &latest);

                if installed == latest {
                    return;
                }
            }

            install_release_asset(release);
        }
        Err(_error) => {}
    };
}

fn asset_by_os_arch(asset: utils::github::Asset) -> Option<utils::github::Asset> {
    let arch = if ARCH == "x86_64" { "amd64" } else { ARCH };
    let name = format!("skaffold-{}-{}", OS, arch);
    if asset.name == name {
        Some(asset)
    } else {
        None
    }
}

fn install_release_asset(release: Release) {
    let asset = match release
        .assets
        .into_iter()
        .filter_map(asset_by_os_arch)
        .next() {
        Some(a) => a,
        None => {
            println!("pkg: skaffold: no asset matches OS and ARCH");
            return;
        }
    };

    println!("pkg: skaffold: installing ...");

    let bin_path = utils::env::home_dir().join(Path::new("bin/skaffold"));
    match utils::http::download(&asset.browser_download_url, &bin_path) {
        Ok(()) => {}
        Err(error) => {
            println!("error: pkg: skaffold: cannot download: {}", error);
            return;
        }
    };
    match utils::fs::set_executable(&bin_path) {
        Ok(()) => {}
        Err(error) => {
            println!("error: pkg: skaffold: cannot chmod a+rx: {}", error);
            return;
        }
    }
}

fn is_installed() -> bool {
    match utils::process::command_output("skaffold", &["version"]) {
        Ok(output) => output.status.success(),
        Err(_error) => false,
    }
}
